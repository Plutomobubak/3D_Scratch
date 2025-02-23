use crate::window::Input;
use minifb::Key;

const SPEED: f32 = 0.01;
const ROTSPEED: f32 = 0.002;
#[derive(Debug)]
pub enum Action {
    Mining,
    Placing,
    No,
}
#[derive(Debug)]
pub struct Controls {
    pub pos: [f32; 3],
    pub rot: [f32; 3],
    pub action: Action,
}
impl Controls {
    pub fn new() -> Self {
        Controls {
            pos: [0.0, 100.0, -5.5],
            rot: [(90.0f32).to_radians(),0.0 ,0.0],
            action: Action::No,
        }
    }
    pub fn update(&mut self, input: &Input, deltat: f32) {
        self.update_pos(input, deltat);
    }
    fn update_pos(&mut self, input: &Input, deltat: f32) {
        let speedd = SPEED * deltat;
        let rotdd = ROTSPEED * deltat;
        let cosrydd = self.rot[1].cos() * speedd;
        let sinrydd = self.rot[1].sin() * speedd;
        // Forward/Backward
        if input.is_key_held(Key::W) {
            self.pos[0] += sinrydd;
            self.pos[2] += cosrydd;
        } else if input.is_key_held(Key::S) {
            self.pos[0] -= sinrydd;
            self.pos[2] -= cosrydd;
        }
        // Up / Down
        if input.is_key_held(Key::Space) {
            self.pos[1] += speedd;
        } else if input.is_key_held(Key::LeftShift) {
            self.pos[1] -= speedd;
        }
        //Left / Right
        if input.is_key_held(Key::A) {
            self.pos[0] += cosrydd;
            self.pos[2] -= sinrydd;
        }
        if input.is_key_held(Key::D) {
            self.pos[0] -= cosrydd;
            self.pos[2] += sinrydd;
        }

        // Look Left / Right
        if input.is_key_held(Key::Left) {
            self.rot[1] += rotdd;
        } else if input.is_key_held(Key::Right) {
            self.rot[1] -= rotdd;
        }
        // Look Down / Up
        if input.is_key_held(Key::Up) {
            self.rot[0] -= rotdd;
        } else if input.is_key_held(Key::Down) {
            self.rot[0] += rotdd;
        }

        // Place/ Mine
        if input.is_key_held(Key::Enter) || input.is_key_held(Key::NumPadEnter) {
            self.action = Action::Mining
        } else if input.is_key_down(Key::NumPad1) {
            self.action = Action::Placing
        } else {
            self.action = Action::No
        }
    }
    pub fn rot_dir(&self) -> [f32; 3] {
        rot_to_dir([self.rot[0], -self.rot[1], self.rot[2]])
    }
}
pub fn rot_to_dir(rot: [f32; 3]) -> [f32; 3] {
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
