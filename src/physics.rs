use crate::types::Matrix;
use std::sync::{Arc, Mutex};

const G: f32 = 6.6743015e-11;

#[derive(Clone, Copy)]
pub enum GravType {
    None,
    Down,
    Space,
}
#[derive(Clone)]
pub struct Physics {
    pub mass: f32,
    pub stat: bool,
    pub grav_type: GravType,
    pub mass_center: [f32; 3],
    pub force: [f32; 3],
    pub accel: [f32; 3],
    pub veloc: [f32; 3],
}
lazy_static::lazy_static! {
    static ref REGISTRY: Mutex<Vec<Arc<Mutex<Physics>>>> = Mutex::new(Vec::new());
}

impl Physics {
    pub fn new(mass: f32, stat: bool, grav_type: GravType) -> Arc<Mutex<Self>> {
        let force = [0.0; 3];
        let accel = [0.0; 3];
        let veloc = [0.0; 3];
        let mass_center = [0.0; 3];
        let phys = Arc::new(Mutex::new(Self {
            mass,
            stat,
            grav_type,
            mass_center,
            force,
            accel,
            veloc,
        }));
        REGISTRY.lock().unwrap().push(Arc::clone(&phys));
        phys
    }
    pub fn update_physics(&mut self, delta: f32) -> [f32; 3] {
        self.force = [0.0; 3];
        match self.grav_type {
            GravType::Space => self.apply_gravity(),
            GravType::Down => self.apply_gravity(),
            GravType::None => {}
        };
        self.accel[0] = self.force[0] / self.mass;
        self.accel[1] = self.force[1] / self.mass;
        self.accel[2] = self.force[2] / self.mass;
        self.mass_center[0] += (self.veloc[0] * delta) + (self.accel[0] * delta / 2.0);
        self.mass_center[1] += (self.veloc[1] * delta) + (self.accel[1] * delta / 2.0);
        self.mass_center[2] += (self.veloc[2] * delta) + (self.accel[2] * delta / 2.0);

        self.veloc[0] += self.accel[0] * delta;
        self.veloc[1] += self.accel[1] * delta;
        self.veloc[2] += self.accel[2] * delta;
        println!("{:?}", self.veloc);
        self.mass_center
    }
    pub fn apply_gravity(&mut self) {
        let mut registry = REGISTRY.lock().unwrap();
        for phys in registry.iter() {
            let phys = phys.try_lock();
            if phys.is_err() {
                continue;
            }
            let phys = phys.unwrap();
            println!("{:?}", self.mass_center);
            let disx = phys.mass_center[0] - self.mass_center[0];
            let disy = phys.mass_center[1] - self.mass_center[1];
            let disz = phys.mass_center[2] - self.mass_center[2];
            let mut dis = ((disx * disx) + (disy * disy) + (disz * disz)).sqrt();
            if dis < 2.0 {
                continue;
            }
            println!("{} {} {}, {}", disx, disy, disz, dis);
            let forc = G * self.mass * phys.mass / (dis * dis);
            println!("forc {}", forc);
            self.force[0] += (disx / dis) * forc;
            self.force[1] += (disy / dis) * forc;
            self.force[2] += (disz / dis) * forc;
            println!("f {:?}", self.force);
        }
        println!("");
    }
}
