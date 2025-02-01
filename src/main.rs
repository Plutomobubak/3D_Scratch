mod draw;
mod model;
mod object;
mod physics;
mod texture;
mod types;
mod window;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

use draw::draw_line;
use model::{load_model, Model, Vertex};
use physics::{GravType, Physics};
use types::Matrix;
use window::{Framebuffer, Window};

fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn rot_to_dir(rot: [f32; 3]) -> [f32; 3] {
    let cos_pitch = rot[0].cos();
    let sin_pitch = rot[0].sin();
    let cos_yaw = rot[1].cos();
    let sin_yaw = rot[1].sin();

    [
        cos_pitch * sin_yaw,  // X
        sin_pitch,            // Y
        -cos_pitch * cos_yaw, // Z
    ]
}
fn main() {
    let mut window = Window::new("asdf", 512, 512);

    let mut helmet = object::Object::new(
        load_model("./assets/helmet/DamagedHelmet.gltf"),
        [0.0, 0.0, 0.0],
        [
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            180.0f32.to_radians(),
        ],
        [1.0, 1.0, 1.0],
    );
    let mut shark = object::Object::new(
        load_model("./assets/blahaj.glb"),
        [0.0, -2.0, 10.0],
        [
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            -90.0f32.to_radians(),
        ],
        [1.0, 1.0, 1.0],
    )
    .with_physics(Physics::new(6.0e12, false, GravType::Space));
    let mut turtle = object::Object::new(
        load_model("./assets/turtle.glb"),
        [0.0, -0.5, 0.0],
        [
            0.0f32.to_radians(),
            180.0f32.to_radians(),
            180.0f32.to_radians(),
        ],
        [1.0, 1.0, 1.0],
    );
    let mut world: Vec<object::Object> = Vec::new();
    for i in -6..4 {
        for j in -6..4 {
            world.push(
                object::Object::cube(
                    [i as f32, -2.0, j as f32],
                    [0.0f32.to_radians(), 0.0f32.to_radians(), 0.0],
                    [1.0, 1.0, 1.0],
                    Some("./assets/grass.png"),
                ), //.with_physics(Physics::new(6.0e12, false, GravType::Space)),
            );
        }
    }
    for i in 0..4 {
        world.push(object::Object::new(
            load_model("./assets/cylinder/cylinder.gltf"),
            [0.0, i as f32 - 1.0, 0.0],
            [
                0.0f32.to_radians(),
                0.0f32.to_radians(),
                180.0f32.to_radians(),
            ],
            [0.5, 0.5, 0.5],
        ));
    }
    let mut dur: Vec<f32> = vec![1000.0; world.len()];

    let timer = SystemTime::now();
    let mut depth_buffer =
        Framebuffer::new(window.framebuffer().width(), window.framebuffer().height());

    let speed = 0.01;
    let rotspeed = 0.002;

    enum Action {
        MINING,
        PLACING,
        NO,
    }
    let deltat = Rc::new(RefCell::new(0.0));
    let x = Rc::new(RefCell::new(0.0));
    let y = Rc::new(RefCell::new(0.0));
    let z = Rc::new(RefCell::new(-5.5));
    let rx = Rc::new(RefCell::new(0.0));
    let ry = Rc::new(RefCell::new(0.0));
    let rz = Rc::new(RefCell::new(0.0));
    let action = Rc::new(RefCell::new(Action::NO));

    let forward = {
        let x = Rc::clone(&x);
        let z = Rc::clone(&z);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *z.borrow_mut() += speed * (*ry.borrow() as f32).cos() * *deltat.borrow();
            *x.borrow_mut() += speed * (*ry.borrow() as f32).sin() * *deltat.borrow();
        }
    };
    let backward = {
        let x = Rc::clone(&x);
        let z = Rc::clone(&z);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *z.borrow_mut() -= speed * (*ry.borrow() as f32).cos() * *deltat.borrow();
            *x.borrow_mut() -= speed * (*ry.borrow() as f32).sin() * *deltat.borrow();
        }
    };
    let up = {
        let deltat = Rc::clone(&deltat);
        let y = Rc::clone(&y);
        move || {
            *y.borrow_mut() += speed * *deltat.borrow();
        }
    };
    let down = {
        let deltat = Rc::clone(&deltat);
        let y = Rc::clone(&y);
        move || {
            *y.borrow_mut() -= speed * *deltat.borrow();
        }
    };
    let links = {
        let z = Rc::clone(&z);
        let x = Rc::clone(&x);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *x.borrow_mut() += speed * (*ry.borrow() as f32).cos() * *deltat.borrow();
            *z.borrow_mut() -= speed * (*ry.borrow() as f32).sin() * *deltat.borrow();
        }
    };
    let llinks = {
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *ry.borrow_mut() += rotspeed * *deltat.borrow();
        }
    };
    let rechts = {
        let x = Rc::clone(&x);
        let z = Rc::clone(&z);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *x.borrow_mut() -= speed * (*ry.borrow() as f32).cos() * *deltat.borrow();
            *z.borrow_mut() += speed * (*ry.borrow() as f32).sin() * *deltat.borrow();
        }
    };
    let lrechts = {
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *ry.borrow_mut() -= rotspeed * *deltat.borrow();
        }
    };
    let ldown = {
        let rx = Rc::clone(&rx);
        let deltat = Rc::clone(&deltat);
        move || {
            *rx.borrow_mut() += rotspeed * *deltat.borrow();
        }
    };
    let lup = {
        let rx = Rc::clone(&rx);
        let deltat = Rc::clone(&deltat);
        move || {
            *rx.borrow_mut() -= rotspeed * *deltat.borrow();
        }
    };
    let mine = {
        let action = Rc::clone(&action);
        move || {
            *action.borrow_mut() = Action::MINING;
        }
    };
    let place = {
        let action = Rc::clone(&action);
        move || {
            *action.borrow_mut() = Action::PLACING;
        }
    };
    window.set_callback(minifb::Key::W, None, Some(Box::new(forward)), None);
    window.set_callback(minifb::Key::S, None, Some(Box::new(backward)), None);
    window.set_callback(minifb::Key::A, None, Some(Box::new(links)), None);
    window.set_callback(minifb::Key::Left, None, Some(Box::new(llinks)), None);
    window.set_callback(minifb::Key::D, None, Some(Box::new(rechts)), None);
    window.set_callback(minifb::Key::Right, None, Some(Box::new(lrechts)), None);
    window.set_callback(minifb::Key::Space, None, Some(Box::new(up)), None);
    window.set_callback(minifb::Key::LeftShift, None, Some(Box::new(down)), None);
    window.set_callback(minifb::Key::Down, None, Some(Box::new(ldown)), None);
    window.set_callback(minifb::Key::Up, None, Some(Box::new(lup)), None);
    window.set_callback(minifb::Key::Enter, None, Some(Box::new(mine)), None);
    window.set_callback(minifb::Key::NumPad1, Some(Box::new(place)), None, None);

    while !window.should_close() {
        let mut start = timer.elapsed().unwrap().as_millis();

        *action.borrow_mut() = Action::NO;
        window.process_input();
        let pos = [*x.borrow_mut(), *y.borrow_mut(), *z.borrow_mut()];
        let rot = [*rx.borrow_mut(), *ry.borrow_mut(), *rz.borrow_mut()];
        // Process buffers
        let fb = window.framebuffer();
        if depth_buffer.width() != fb.width() || depth_buffer.height() != fb.height() {
            depth_buffer = Framebuffer::new(fb.width(), fb.height());
        }
        fb.clear(rgb_to_u32(0, 0, 0));
        depth_buffer.clear(u32::MAX);

        //View and projection
        let view_matrix = Matrix::identity().rotate(rot) * Matrix::trans(pos);
        let proj_matrix = Matrix::projection(
            60.0f32.to_radians(),
            fb.width() as f32 / fb.height() as f32,
            0.1,
            300.0,
        );
        let view_proj = proj_matrix * view_matrix;

        // Rotate objects

        let elapsed = timer.elapsed().unwrap().as_secs_f32();
        // helmet.rotation[1] = elapsed;
        //cube.rotation[1] = elapsed;
        shark.rotation[1] = elapsed;
        // turtle.rotation[1] = elapsed;

        //shark.update_physics(delta);
        //cube.update_physics(delta);
        // cube2.update_physics(delta);

        //Render objects

        shark.render(fb, &mut depth_buffer, &view_proj);
        let mut ground = false;
        let mut td = (6.5, [0.0; 3]);
        let mut touch: Option<usize> = None;
        let shark_dir = rot_to_dir([
            shark.rotation[0],
            shark.rotation[1] - 90.0f32.to_radians(),
            shark.rotation[2],
        ]);
        let pdir = rot_to_dir([rot[0], -rot[1], rot[2]]);
        for i in 0..world.len() {
            let cube = &mut world[i];
            if cube
                .raycast(
                    pos,
                    [0.0, 1.0, -0.0],
                    1.5,
                    fb,
                    &mut depth_buffer,
                    &view_proj,
                    false,
                )
                .0
                < 3.5
            {
                ground = true;
            }
            cube.raycast(
                shark.position,
                shark_dir,
                16.5,
                fb,
                &mut depth_buffer,
                &view_proj,
                true,
            );
            let t = cube.raycast(pos, pdir, 6.5, fb, &mut depth_buffer, &view_proj, true);
            if t.0 < td.0 {
                td = t;
                touch = Some(i);
            }
        }
        if !ground {
            *y.borrow_mut() -= 0.001 * deltat.borrow_mut().to_owned();
        }
        if touch.is_some() {
            let scale: Matrix = vec![
                vec![world[touch.unwrap()].scale[0], 0.0, 0.0, 0.0],
                vec![0.0, world[touch.unwrap()].scale[1], 0.0, 0.0],
                vec![0.0, 0.0, world[touch.unwrap()].scale[2], 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ]
            .into();
            let mut pos_matrix = Matrix::trans(world[touch.unwrap()].position) * -1.0;
            pos_matrix = pos_matrix.rotate(world[touch.unwrap()].rotation) * scale;
            for mesh in world[touch.unwrap()].model.meshes.iter() {
                for i in 0..mesh.indices.len() / 4 {
                    let v0 = &pos_matrix * mesh.vertices[mesh.indices[3 * i] as usize].position;
                    let v1 = &pos_matrix * mesh.vertices[mesh.indices[3 * i + 1] as usize].position;
                    let v2 = &pos_matrix * mesh.vertices[mesh.indices[3 * i + 2] as usize].position;
                    let v3 = &pos_matrix * mesh.vertices[mesh.indices[3 * i + 3] as usize].position;
                    draw_line(fb, &mut depth_buffer, &v0, &v1, &view_proj);
                    draw_line(fb, &mut depth_buffer, &v1, &v2, &view_proj);
                    draw_line(fb, &mut depth_buffer, &v2, &v3, &view_proj);
                    draw_line(fb, &mut depth_buffer, &v3, &v0, &view_proj);
                }
            }
            let mut mining = false;
            match *action.borrow() {
                Action::PLACING => {
                    world.push(object::Object::cube(
                        td.1, // Entirely random number 100% randomness guaranteed
                        [0.0f32.to_radians(), 0.0f32.to_radians(), 0.0],
                        [1.0, 1.0, 1.0],
                        Some("./assets/grass.png"),
                    ));
                    dur.push(1000.0);
                }
                Action::MINING => {
                    dur[touch.unwrap()] -= *deltat.borrow();
                    if dur[touch.unwrap()] <= 0.0 {
                        world.remove(touch.unwrap());
                        dur.remove(touch.unwrap());
                    }
                    mining = true;
                }
                Action::NO => (),
            }
            for i in 0..dur.len() {
                if i != touch.unwrap() || !mining {
                    dur[i] = 1000.0;
                }
            }
        }
        println!("Raycast: {}", timer.elapsed().unwrap().as_millis() - start);
        start = timer.elapsed().unwrap().as_millis();
        for cube in world.iter_mut() {
            cube.render(fb, &mut depth_buffer, &view_proj);
        }
        println!("Render: {}", timer.elapsed().unwrap().as_millis() - start);
        let screen_size = [fb.width(), fb.height()];
        for x in screen_size[0] / 2 - 2..screen_size[0] / 2 + 2 {
            for y in screen_size[1] / 2 - 2..screen_size[1] / 2 + 2 {
                fb.set_pixel(x, y, rgb_to_u32(255, 0, 0));
            }
        }
        // cube2.render(fb, &mut depth_buffer, &view_proj);
        // turtle.render(fb, &mut depth_buffer, &view_proj);
        //helmet.render(fb, &mut depth_buffer, &view_proj);
        *deltat.borrow_mut() = (timer.elapsed().unwrap().as_millis() - start) as f32;

        //Benchmark
        // println!("Matrix multiplications: {}", types::get_matrix_mul_count());
        // println!(
        //     "Time between frames: {}",
        //     timer.elapsed().unwrap().as_millis() - start
        // );
        // types::reset_matrix_mul_count();
        window.update();
    }
}
