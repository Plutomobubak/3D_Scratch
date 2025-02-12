use std::collections::HashMap;
use std::usize;

use minifb::Key;

#[derive(Clone, Copy)]
enum State {
    PRESSED,
    HELD,
    RELEASED,
}

pub struct Window {
    window: minifb::Window,
    framebuffer: Framebuffer,
    callbacks: Vec<(State, Box<dyn FnMut()>, Box<dyn FnMut()>, Box<dyn FnMut()>)>,
}
pub struct Framebuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}
impl Window {
    pub fn new(name: &str, w: usize, h: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(name, w, h, options).expect("Failed to create window");
        let mut callbacks: Vec<(State, Box<dyn FnMut()>, Box<dyn FnMut()>, Box<dyn FnMut()>)> =
            Vec::new();
        for _ in 0..300 {
            callbacks.push((
                State::RELEASED,
                Box::new(|| {}),
                Box::new(|| {}),
                Box::new(|| {}),
            ));
        }

        Window {
            window,
            callbacks,
            framebuffer: Framebuffer::new(w, h),
        }
    }
    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }
    pub fn set_callback(
        &mut self,
        key: Key,
        press: Option<Box<dyn FnMut()>>,
        hold: Option<Box<dyn FnMut()>>,
        release: Option<Box<dyn FnMut()>>,
    ) {
        let i: usize = key as usize;
        if let Some(press) = press {
            self.callbacks[i].1 = press;
        }
        if let Some(hold) = hold {
            self.callbacks[i].2 = hold;
        }
        if let Some(release) = release {
            self.callbacks[i].3 = release;
        }
    }
    pub fn process_input(&mut self) {
        let mut h: Vec<usize> = Vec::new();
        self.window.get_keys().iter().for_each(|k| {
            let i = *k as usize;
            h.push(i);
            let k = &mut self.callbacks[i];
            match k.0 {
                State::RELEASED => {
                    k.1();
                    k.0 = State::PRESSED;
                }
                State::PRESSED => {
                    k.0 = State::HELD;
                }
                State::HELD => {
                    k.2();
                }
            }
        });
        for l in 0..self.callbacks.len() {
            if !h.contains(&l) {
                let k = &mut self.callbacks[l];
                match k.0 {
                    State::HELD | State::PRESSED => {
                        k.3();
                        k.0 = State::RELEASED;
                    }
                    State::RELEASED => {}
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(
                &self.framebuffer.data,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .expect("Failed to update with buffer");

        let (width, height) = self.window.get_size();
        if width != self.framebuffer.width || height != self.framebuffer.height {
            self.framebuffer = Framebuffer::new(width, height)
        }
    }
}
impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.data[x + y * self.width] = val;
    }
    pub fn set_pixel_f32(&mut self, x: usize, y: usize, val: f32) {
        self.data[x + y * self.width] = (val * u32::MAX as f32) as u32;
    }
    pub fn get_pixel_f32(&mut self, x: usize, y: usize) -> f32 {
        self.data[x + y * self.width] as f32 / u32::MAX as f32
    }
    pub fn clear(&mut self, col: u32) {
        for i in 0..self.data.len() {
            self.data[i] = col;
        }
    }
}
