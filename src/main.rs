mod controls;
mod draw;
mod model;
mod object;
mod physics;
mod texture;
mod types;
mod window;

use std::time::SystemTime;

use controls::{rot_to_dir, Action, Controls};
use draw::draw_line;
use model::load_model;
use physics::{GravType, Physics};
use types::Matrix;
use window::{Framebuffer, Window};

fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn main() {
    let mut window = Window::new("asdf", 512, 512);

    // let mut helmet = object::Object::new(
    //     load_model("./assets/helmet/DamagedHelmet.gltf"),
    //     [0.0, 0.0, 0.0],
    //     [
    //         0.0f32.to_radians(),
    //         0.0f32.to_radians(),
    //         180.0f32.to_radians(),
    //     ],
    //     [1.0, 1.0, 1.0],
    // );
    let mut shark = object::Object::new(
        load_model("./assets/blahaj.glb"),
        [0.0, -2.0, 50.0],
        [
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            -90.0f32.to_radians(),
        ],
        [1.0, 1.0, 1.0],
    )
    .with_physics(Physics::new(2.0e3, false, GravType::Space));
    shark.physics.as_mut().unwrap().lock().unwrap().veloc[0] = 0.0895;
    let ball = object::Object::new(
        load_model("./assets/ball/ball.gltf"),
        [0.0, -2.0, 0.0],
        [
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            180.0f32.to_radians(),
        ],
        [10.0, 10.0, 10.0],
    )
    .with_physics(Physics::new(6.0e9, false, GravType::Space));
    let mut world: Vec<object::Object> = Vec::new();
    world.push(ball);
    let mut dur: Vec<f32> = vec![1000.0; world.len()];

    let timer = SystemTime::now();
    let mut depth_buffer =
        Framebuffer::new(window.framebuffer().width(), window.framebuffer().height());
    let mut player = Controls::new();
    let mut deltat = 0.1;

    while !window.should_close() {
        let start = timer.elapsed().unwrap().as_millis();
        player.update(window.input(), deltat);

        // Process buffers
        let fb = window.framebuffer();
        if depth_buffer.width() != fb.width() || depth_buffer.height() != fb.height() {
            depth_buffer = Framebuffer::new(fb.width(), fb.height());
        }
        fb.clear(rgb_to_u32(0, 0, 0));
        depth_buffer.clear(u32::MAX);

        //View and projection
        let view_matrix = Matrix::identity().rotate(player.rot) * Matrix::trans(player.pos);
        let proj_matrix = Matrix::projection(
            60.0f32.to_radians(),
            fb.width() as f32 / fb.height() as f32,
            0.1,
            300.0,
        );
        let view_proj = proj_matrix * view_matrix;

        // Rotate objects

        let elapsed = timer.elapsed().unwrap().as_secs_f32();
        shark.rotation[1] = elapsed;

        //Render objects

        // Raycast
        let mut groundd = 99.9;
        let mut td = (6.5, [0.0; 3]);
        let mut touch: Option<usize> = None;
        let shark_dir = rot_to_dir([
            shark.rotation[0],
            shark.rotation[1] - 90.0f32.to_radians(),
            shark.rotation[2],
        ]);
        let pdir = player.rot_dir();
        for i in 0..world.len() {
            let cube = &mut world[i];
            // Ground check
            let d = cube
                .raycast(
                    player.pos,
                    [0.0, 1.0, -0.0],
                    2.5,
                    fb,
                    &mut depth_buffer,
                    &view_proj,
                    false,
                )
                .0;
            if d < groundd {
                groundd = d;
            }
            // Point
            cube.raycast(
                shark.position,
                shark_dir,
                16.5,
                fb,
                &mut depth_buffer,
                &view_proj,
                true,
            );
            let t = cube.raycast(
                player.pos,
                pdir,
                6.5,
                fb,
                &mut depth_buffer,
                &view_proj,
                true,
            );
            if t.0 < td.0 {
                td = t;
                touch = Some(i);
            }
        }
        if groundd >= 2.5 {
            //player.pos[1] -= 0.001 * deltat;
        } else if groundd < 1.5 {
            player.pos[1] += 0.005 * deltat;
        }
        // Interact with blocks
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
                for i in 0..mesh.indices.len() / 3 {
                    let scal = Matrix::identity() * 1.05;
                    let v1 = &pos_matrix
                        * (&scal * mesh.vertices[mesh.indices[3 * i + 1] as usize].position);
                    let v2 = &pos_matrix
                        * (&scal * mesh.vertices[mesh.indices[3 * i + 2] as usize].position);
                    draw_line(
                        fb,
                        &mut depth_buffer,
                        &v1,
                        &v2,
                        &view_proj,
                        &[255, 255, 255],
                    );
                }
            }
            let mut mining = false;
            match player.action {
                Action::Placing => {
                    let hit = world[touch.unwrap()].position;
                    let xdif = td.1[0] - hit[0];
                    let ydif = td.1[1] - hit[1];
                    let zdif = td.1[2] - hit[2];
                    let dir = if xdif.abs() > ydif.abs() {
                        if xdif.abs() > zdif.abs() {
                            [1.0 * xdif.signum(), 0.0, 0.0]
                        } else {
                            [0.0, 0.0, 1.0 * zdif.signum()]
                        }
                    } else if ydif.abs() > zdif.abs() {
                        [0.0, 1.0 * ydif.signum(), 0.0]
                    } else {
                        [0.0, 0.0, 1.0 * zdif.signum()]
                    };
                    let new_pos = [hit[0] + dir[0], hit[1] + dir[1], hit[2] + dir[2]];
                    world.push(object::Object::cube(
                        new_pos,
                        [0.0f32.to_radians(), 0.0f32.to_radians(), 0.0],
                        [1.0, 1.0, 1.0],
                        Some("./assets/grass.png"),
                    ));
                    dur.push(1000.0);
                }
                Action::Mining => {
                    dur[touch.unwrap()] -= deltat;
                    if dur[touch.unwrap()] <= 0.0 {
                        world.remove(touch.unwrap());
                        dur.remove(touch.unwrap());
                    }
                    mining = true;
                }
                Action::No => (),
            }
            for i in 0..dur.len() {
                if i != touch.unwrap() || !mining {
                    dur[i] = 1000.0;
                }
            }
        }
        // Render all
        for cube in world.iter_mut() {
            cube.update_physics(deltat);
            cube.render(fb, &mut depth_buffer, &view_proj);
        }
        shark.update_physics(deltat);
        shark.render(fb, &mut depth_buffer, &view_proj);
        // Draw cursor in the middle
        let screen_size = [fb.width(), fb.height()];
        for x in screen_size[0] / 2 - 2..screen_size[0] / 2 + 2 {
            for y in screen_size[1] / 2 - 2..screen_size[1] / 2 + 2 {
                fb.set_pixel(x, y, rgb_to_u32(255, 0, 0));
            }
        }
        deltat = (timer.elapsed().unwrap().as_millis() - start) as f32;

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
