use std::collections::HashSet;
use std::usize;

use minifb::Key;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Pressed,
    Held,
    Released,
}

pub struct Window {
    window: minifb::Window,
    framebuffer: Framebuffer,
    input: Input,
}
pub struct Framebuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}
pub struct Input {
    key_states: std::collections::HashMap<Key, State>,
    pressed_keys: HashSet<Key>,
    held_keys: HashSet<Key>,
    released_keys: HashSet<Key>,
}
impl Window {
    pub fn new(name: &str, w: usize, h: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(name, w, h, options).expect("Failed to create window");
        let input = Input::new();

        Window {
            window,
            input,
            framebuffer: Framebuffer::new(w, h),
        }
    }
    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
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
        self.input.process_input(&self.window);
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
    pub fn get_pixel(&mut self, x: usize, y: usize) -> u32 {
        self.data[x + y * self.width]
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
impl Input {
    pub fn new() -> Self {
        Self {
            key_states: std::collections::HashMap::new(),
            pressed_keys: HashSet::new(),
            held_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    pub fn process_input(&mut self, window: &minifb::Window) {
        let current_keys: HashSet<Key> = window.get_keys().into_iter().collect();

        self.pressed_keys.clear();
        self.released_keys.clear();
        self.held_keys.clear();

        for key in &current_keys {
            let state = self.key_states.entry(*key).or_insert(State::Released);
            match *state {
                State::Released => {
                    self.pressed_keys.insert(*key);
                    *state = State::Pressed;
                }
                State::Pressed | State::Held => {
                    self.held_keys.insert(*key);
                    *state = State::Held;
                }
            }
        }

        self.key_states.retain(|key, state| {
            if !current_keys.contains(key) {
                if *state == State::Pressed || *state == State::Held {
                    self.released_keys.insert(*key);
                }
                *state = State::Released;
                false
            } else {
                true
            }
        });
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_key_held(&self, key: Key) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.released_keys.contains(&key)
    }
}
