mod model;
mod object;
mod physics;
mod texture;
mod types;
mod window;

use std::time::SystemTime;

use model::{load_model, Model, Vertex};
use physics::{GravType, Physics};
use types::Matrix;
use window::{Framebuffer, Window};

fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
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
        [0.0, 5.0, 10.0],
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
    let mut cube = object::Object::cube(
        [0.0, -0.0, 0.0],
        [0.0f32.to_radians(), 0.0, 0.0],
        [2.0, 2.0, 2.0],
        Some("./assets/grass.png"),
    )
    .with_physics(Physics::new(6.0e12, false, GravType::Space));
    // let mut cube2 = object::Object::cube(
    //     [5.0, 0.0, 10.0],
    //     [0.0f32.to_radians(), 0.0, 0.0],
    //     [2.0, 2.0, 2.0],
    // )
    // .with_physics(Physics::new(6.0e13, false, GravType::Space));

    let timer = SystemTime::now();
    let mut depth_buffer =
        Framebuffer::new(window.framebuffer().width(), window.framebuffer().height());

    fn press() {
        println!("press")
    }
    fn hold() {
        println!("hold")
    }
    fn release() {
        println!("release")
    }
    window.set_callback(minifb::Key::Escape, press, hold, release);

    while !window.should_close() {
        let start = timer.elapsed().unwrap().as_millis();

        window.process_input();
        // Process buffers
        let fb = window.framebuffer();
        if depth_buffer.width() != fb.width() || depth_buffer.height() != fb.height() {
            depth_buffer = Framebuffer::new(fb.width(), fb.height());
        }
        fb.clear(rgb_to_u32(0, 0, 0));
        depth_buffer.clear(u32::MAX);

        //View and projection
        let view_matrix = Matrix::view([0.0, 0.0, -5.5]);
        let proj_matrix = Matrix::projection(
            60.0f32.to_radians(),
            fb.width() as f32 / fb.height() as f32,
            0.1,
            300.0,
        );
        let view_proj = proj_matrix * view_matrix;

        // Rotate objects

        let elapsed = timer.elapsed().unwrap().as_secs_f32();
        helmet.rotation[1] = elapsed;
        cube.rotation[1] = elapsed;
        shark.rotation[1] = elapsed;
        turtle.rotation[1] = elapsed;
        let delta = (timer.elapsed().unwrap().as_millis() - start) as f32 / 1000.0;

        //shark.update_physics(delta);
        //cube.update_physics(delta);
        // cube2.update_physics(delta);

        //Render objects

        shark.render(fb, &mut depth_buffer, &view_proj);
        cube.render(fb, &mut depth_buffer, &view_proj);
        // cube2.render(fb, &mut depth_buffer, &view_proj);
        turtle.render(fb, &mut depth_buffer, &view_proj);
        //helmet.render(fb, &mut depth_buffer, &view_proj);

        //Benchmark
        // println!("Matrix multiplications: {}", types::get_matrix_mul_count());
        // println!(
        //     "Time between frames: {}",
        //     timer.elapsed().unwrap().as_millis() - start
        // );
        types::reset_matrix_mul_count();
        window.update();
    }
}
