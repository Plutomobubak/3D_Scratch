mod draw;
mod model;
mod object;
mod physics;
mod texture;
mod types;
mod window;

use std::rc::Rc;
use std::time::SystemTime;
use std::{borrow::Borrow, cell::RefCell};

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
    let mut dir = [rot[1].sin(), rot[0].sin(), -rot[1].cos()];
    let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();

    if len != 0.0 {
        dir[0] /= len;
        dir[1] /= len;
        dir[2] /= len;
    }
    dir
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
                    [0.0f32.to_radians(), 0.0, 0.0],
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
    // let mut cube2 = object::Object::cube(
    //     [5.0, 0.0, 10.0],
    //     [0.0f32.to_radians(), 0.0, 0.0],
    //     [2.0, 2.0, 2.0],
    // )
    // .with_physics(Physics::new(6.0e13, false, GravType::Space));

    let timer = SystemTime::now();
    let mut depth_buffer =
        Framebuffer::new(window.framebuffer().width(), window.framebuffer().height());

    let speed = 0.01;
    let rotspeed = 0.002;
    let deltat = Rc::new(RefCell::new(0.0));
    let x = Rc::new(RefCell::new(0.0));
    let y = Rc::new(RefCell::new(0.0));
    let z = Rc::new(RefCell::new(-5.5));
    let rx = Rc::new(RefCell::new(0.0));
    let ry = Rc::new(RefCell::new(0.0));
    let rz = Rc::new(RefCell::new(0.0));

    let forward = {
        let x = Rc::clone(&x);
        let z = Rc::clone(&z);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *z.borrow_mut() +=
                speed * (ry.borrow_mut().to_owned() as f32).cos() * deltat.borrow_mut().to_owned();
            *x.borrow_mut() +=
                speed * (ry.borrow_mut().to_owned() as f32).sin() * deltat.borrow_mut().to_owned();
        }
    };
    let backward = {
        let x = Rc::clone(&x);
        let z = Rc::clone(&z);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *z.borrow_mut() -=
                speed * (ry.borrow_mut().to_owned() as f32).cos() * deltat.borrow_mut().to_owned();
            *x.borrow_mut() -=
                speed * (ry.borrow_mut().to_owned() as f32).sin() * deltat.borrow_mut().to_owned();
        }
    };
    let up = {
        let deltat = Rc::clone(&deltat);
        let y = Rc::clone(&y);
        move || {
            *y.borrow_mut() += speed * deltat.borrow_mut().to_owned();
        }
    };
    let down = {
        let deltat = Rc::clone(&deltat);
        let y = Rc::clone(&y);
        move || {
            *y.borrow_mut() -= speed * deltat.borrow_mut().to_owned();
        }
    };
    let links = {
        let z = Rc::clone(&z);
        let x = Rc::clone(&x);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *x.borrow_mut() +=
                speed * (ry.borrow_mut().to_owned() as f32).cos() * deltat.borrow_mut().to_owned();
            *z.borrow_mut() -=
                speed * (ry.borrow_mut().to_owned() as f32).sin() * deltat.borrow_mut().to_owned();
        }
    };
    let llinks = {
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *ry.borrow_mut() += rotspeed * deltat.borrow_mut().to_owned();
        }
    };
    let rechts = {
        let x = Rc::clone(&x);
        let z = Rc::clone(&z);
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *x.borrow_mut() -=
                speed * (ry.borrow_mut().to_owned() as f32).cos() * deltat.borrow_mut().to_owned();
            *z.borrow_mut() +=
                speed * (ry.borrow_mut().to_owned() as f32).sin() * deltat.borrow_mut().to_owned();
        }
    };
    let lrechts = {
        let ry = Rc::clone(&ry);
        let deltat = Rc::clone(&deltat);
        move || {
            *ry.borrow_mut() -= rotspeed * deltat.borrow_mut().to_owned();
        }
    };
    let ldown = {
        let rx = Rc::clone(&rx);
        let deltat = Rc::clone(&deltat);
        move || {
            *rx.borrow_mut() += rotspeed * deltat.borrow_mut().to_owned();
        }
    };
    let lup = {
        let rx = Rc::clone(&rx);
        let deltat = Rc::clone(&deltat);
        move || {
            *rx.borrow_mut() -= rotspeed * deltat.borrow_mut().to_owned();
        }
    };
    let lw = {
        let rz = Rc::clone(&rz);
        let deltat = Rc::clone(&deltat);
        move || {
            *rz.borrow_mut() -= rotspeed * deltat.borrow_mut().to_owned();
        }
    };
    let ls = {
        let rz = Rc::clone(&rz);
        let deltat = Rc::clone(&deltat);
        move || {
            *rz.borrow_mut() += rotspeed * deltat.borrow_mut().to_owned();
        }
    };
    window.set_callback(minifb::Key::W, None, Some(Box::new(forward)), None);
    window.set_callback(minifb::Key::S, None, Some(Box::new(backward)), None);
    window.set_callback(minifb::Key::A, None, Some(Box::new(links)), None);
    window.set_callback(minifb::Key::Q, None, Some(Box::new(llinks)), None);
    window.set_callback(minifb::Key::D, None, Some(Box::new(rechts)), None);
    window.set_callback(minifb::Key::E, None, Some(Box::new(lrechts)), None);
    window.set_callback(minifb::Key::Space, None, Some(Box::new(up)), None);
    window.set_callback(minifb::Key::LeftShift, None, Some(Box::new(down)), None);
    window.set_callback(minifb::Key::Down, None, Some(Box::new(ldown)), None);
    window.set_callback(minifb::Key::Up, None, Some(Box::new(lup)), None);
    window.set_callback(minifb::Key::Left, None, Some(Box::new(lw)), None);
    window.set_callback(minifb::Key::Right, None, Some(Box::new(ls)), None);

    while !window.should_close() {
        let mut start = timer.elapsed().unwrap().as_millis();

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
        let mut td = 16.5;
        let mut touch: Option<usize> = None;
        let shark_dir = rot_to_dir([
            shark.rotation[0],
            shark.rotation[1] - 90.0f32.to_radians(),
            shark.rotation[2],
        ]);
        let pdir = rot_to_dir([rot[0], -rot[1], rot[2]]);
        let mut rpos = pos;
        rpos[0] += pdir[0] * 0.05;
        rpos[2] += pdir[2] * 0.05;
        for i in 0..world.len() {
            let cube = &mut world[i];
            if cube.raycast(
                pos,
                [0.0, 1.0, -0.0],
                1.5,
                fb,
                &mut depth_buffer,
                &view_proj,
                false,
            ) < 3.5
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
            let t = cube.raycast(rpos, pdir, 16.5, fb, &mut depth_buffer, &view_proj, true);
            if t < td {
                td = t;
                touch = Some(i);
                println!("{}: {}", i, t);
            }
        }
        if !ground {
            *y.borrow_mut() -= 0.001 * deltat.borrow_mut().to_owned();
        }
        if touch.is_some() {
            println!("touch");
            let mut curr = world[touch.unwrap()].rotation;
            curr[1] += 45.0f32.to_radians();
            world[touch.unwrap()].rotation = curr;
        }
        println!("Raycast: {}", timer.elapsed().unwrap().as_millis() - start);
        start = timer.elapsed().unwrap().as_millis();
        for cube in world.iter_mut() {
            cube.render(fb, &mut depth_buffer, &view_proj);
        }
        println!("Render: {}", timer.elapsed().unwrap().as_millis() - start);
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
