use crate::{
    draw::*,
    model::{Material, Mesh, Model, Vertex},
    physics::Physics,
    texture::{load_texture, Texture},
    types::Matrix,
    window::Framebuffer,
};
use std::sync::{Arc, Mutex};

pub struct Object {
    pub model: Model,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub physics: Option<Arc<Mutex<Physics>>>,
}
impl Object {
    pub fn new(model: Model, position: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) -> Self {
        Object {
            model,
            position,
            rotation,
            scale,
            physics: None,
        }
    }
    pub fn with_physics(mut self, physics: Arc<Mutex<Physics>>) -> Self {
        let mut physics = physics;
        physics.lock().unwrap().mass_center = self.position;
        self.physics = Some(physics);
        self
    }
    // Creates cube with center of 0,0,0 and edge lenght of 1, that can be shifted, rotated and
    // resized by params
    pub fn cube(
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
        texture_path: Option<&str>,
    ) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();

        // Sides
        for i in 0..4 {
            // println!("{}", i);
            for j in 0..4 {
                let mut position = [0.0; 3];
                if (i % 2) == 1 {
                    position = [
                        0.5 - ((i) % 4 > 1) as i32 as f32,
                        -0.5 + (j % 4 > 1) as i32 as f32,
                        -0.5 + ((i + j + 2) % 4 > 1) as i32 as f32,
                    ];
                } else {
                    position = [
                        -0.5 + ((j + i + 1) % 4 > 1) as i32 as f32,
                        -0.5 + (j % 4 > 1) as i32 as f32,
                        0.5 - ((i) % 4 > 1) as i32 as f32,
                    ];
                }
                let normal = [
                    ((i) % 2) as f32 * (2.0 - i as f32),
                    0.0,
                    ((i + 1) % 2) as f32 * (1.0 - i as f32),
                ];
                let tex_coord = [
                    ((i + ((j + 1) % 4 > 1) as i32) as f32) / 6.0,
                    position[1] + 0.5,
                ];
                // println!("{:?} {:?}", position, tex_coord);

                vertices.push(Vertex {
                    normal,
                    position,
                    tex_coord,
                });
            }
        }
        // Top
        for i in 0..4 {
            let position = [
                0.5 - (i % 4 > 1) as i32 as f32,
                -0.5,
                0.5 - ((i + 3) % 4 > 1) as i32 as f32,
            ];
            let tex_coord = [(4.0 + position[0] + 0.5) / 6.0, position[2] + 0.5];

            vertices.push(Vertex {
                normal: [0.0, -1.0, 0.0],
                position,
                tex_coord,
            });
        }
        // Bottom
        for i in 0..4 {
            let position = [
                0.5 - (i % 4 > 1) as i32 as f32,
                0.5,
                0.5 - ((i + 1) % 4 > 1) as i32 as f32,
            ];
            let tex_coord = [(5.0 + position[0] + 0.5) / 6.0, position[2] + 0.5];

            vertices.push(Vertex {
                normal: [0.0, 1.0, 0.0],
                position,
                tex_coord,
            });
        }
        // println!("{}", vertices.len());

        let mut indices: Vec<u32> = Vec::new();
        for i in 0..6 {
            indices.append(&mut vec![i * 4, i * 4 + 1, (i * 4) + 2]);
            indices.append(&mut vec![i * 4, i * 4 + 2, i * 4 + 3]);
        }
        // println!("{:?}", indices);

        let mesh = Mesh {
            vertices,
            indices,
            material_idx: 0,
        };
        let mut mat = Material::default();
        if let Some(path) = texture_path {
            let texture = load_texture(path);
            mat.base_color_texture = Some(texture);
        }

        let model = Model {
            meshes: vec![mesh],
            mats: vec![mat],
        };

        Object {
            model,
            position,
            rotation,
            scale,
            physics: None,
        }
    }
    pub fn update_physics(&mut self, delta: f32) {
        self.position = self
            .physics
            .as_mut()
            .unwrap()
            .lock()
            .unwrap()
            .update_physics(delta);
    }
    pub fn raycast(
        &self,
        ray_origin: [f32; 3],
        ray_dir: [f32; 3],
        dist: f32,
        fb: &mut Framebuffer,
        depth_buffer: &mut Framebuffer,
        mvp: &Matrix,
        draw: bool,
    ) -> (f32, [f32; 3]) {
        let mut mind = dist;
        let mut d0: Vertex = Vertex {
            position: [0.0; 3],
            normal: [0.0; 3],
            tex_coord: [0.0; 2],
        };
        let mut d1: Vertex = Vertex {
            position: [0.0; 3],
            normal: [0.0; 3],
            tex_coord: [0.0; 2],
        };
        let mut d2: Vertex = Vertex {
            position: [0.0; 3],
            normal: [0.0; 3],
            tex_coord: [0.0; 2],
        };
        let mut minint = Matrix::identity();
        for mesh in self.model.meshes.iter() {
            for i in 0..(mesh.indices.len() / 3) {
                let scale: Matrix = vec![
                    vec![self.scale[0], 0.0, 0.0, 0.0],
                    vec![0.0, self.scale[1], 0.0, 0.0],
                    vec![0.0, 0.0, self.scale[2], 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ]
                .into();
                let pos_matrix = (Matrix::trans(self.position).rotate(self.rotation)) * scale;
                let mut v0 = mesh.vertices[mesh.indices[i * 3] as usize].position;
                v0 = &pos_matrix * v0;
                let v0: Matrix = vec![v0.to_vec()].into();

                let mut v1 = mesh.vertices[mesh.indices[i * 3 + 1] as usize].position;
                v1 = &pos_matrix * v1;
                let v1: Matrix = vec![v1.to_vec()].into();

                let mut v2 = mesh.vertices[mesh.indices[i * 3 + 2] as usize].position;
                v2 = &pos_matrix * v2;
                let v2: Matrix = vec![v2.to_vec()].into();

                let origin: Matrix = vec![ray_origin.to_vec()].into();
                let ray: Matrix = vec![ray_dir.to_vec()].into();
                let e1 = &v1 - &v0;
                let e2 = &v2 - &v0;
                let mat: Matrix = vec![e1[0].clone(), e2[0].clone(), ray[0].clone()].into();
                let det = mat.det();
                let inv_det = 1.0 / det;
                let s = &origin - &v0;
                let mat_u: Matrix = vec![s[0].clone(), e2[0].clone(), ray[0].clone()].into();
                let u = mat_u.det() * inv_det;
                if !(0.0..=1.0).contains(&u) {
                    continue;
                }
                let mat_v: Matrix = vec![e1[0].clone(), s[0].clone(), ray[0].clone()].into();
                let v = mat_v.det() * inv_det;
                if !(0.0..=1.0).contains(&v) || (u + v > 1.0) {
                    continue;
                }
                let mat_t: Matrix = vec![e1[0].clone(), e2[0].clone(), s[0].clone()].into();
                let t = mat_t.det() * inv_det;
                if t > f32::EPSILON {
                    let intersection_point = &origin + &(ray.clone() * (-t));
                    // println!("{:?}", intersection_point[0]);
                    let dmat = &intersection_point - &origin;
                    let d = (dmat[0][0] * dmat[0][0]
                        + dmat[0][1] * dmat[0][1]
                        + dmat[0][2] * dmat[0][2])
                        .sqrt();
                    if d < mind {
                        mind = d;
                        d0 = mesh.vertices[mesh.indices[i * 3] as usize];
                        d1 = mesh.vertices[mesh.indices[i * 3 + 1] as usize];
                        d2 = mesh.vertices[mesh.indices[i * 3 + 2] as usize];
                        minint = vec![vec![
                            intersection_point[0][0] * -1.0,
                            intersection_point[0][1] * -1.0,
                            intersection_point[0][2] * -1.0,
                        ]]
                        .into();
                        if draw {
                            let mut origin = origin * -1.0;
                            origin[0][2] += 0.01;
                            let inter = intersection_point * -1.0;
                            draw_line(fb, depth_buffer, &origin.into(), &inter.into(), mvp);
                        }
                    }
                }
            }
        }
        if mind < dist {
            let mat = Material {
                base_col: [1.0, 0.0, 0.0, 1.0],
                ..Default::default()
            };
            // d0.position[0] *= -1.0;
            // d0.position[1] *= -1.0;
            // d0.position[2] *= -1.0;
            // d1.position[0] *= -1.0;
            // d1.position[1] *= -1.0;
            // d1.position[2] *= -1.0;
            // d2.position[0] *= -1.0;
            // d2.position[1] *= -1.0;
            // d2.position[2] *= -1.0;
            // draw_line(fb, depth_buffer, &d0.position, &d1.position, mvp);
            // draw_line(fb, depth_buffer, &d2.position, &d1.position, mvp);
            // draw_line(fb, depth_buffer, &d0.position, &d2.position, mvp);
            d0.position = ((&minint) + &Matrix::from(vec![vec![-0.05, -0.05, 0.1]]))[0]
                .clone()
                .try_into()
                .unwrap();
            d1.position = ((&minint) + &Matrix::from(vec![vec![0.05, -0.05, 0.1]]))[0]
                .clone()
                .try_into()
                .unwrap();
            d2.position = ((&minint) + &Matrix::from(vec![vec![0.0, 0.1, 0.1]]))[0]
                .clone()
                .try_into()
                .unwrap();

            draw_triangle(
                fb,
                depth_buffer,
                &d0,
                &d1,
                &d2,
                mvp,
                &Matrix::identity(),
                &mat,
            );
        }
        (mind, (minint * -1.0).into())
    }
    // Renders to Framebuffer using its properties and given view-projection matrix
    pub fn render(&self, fb: &mut Framebuffer, depth_buffer: &mut Framebuffer, view_proj: &Matrix) {
        // Transform by position
        let pos = [
            self.position[0] * -1.0,
            self.position[1] * -1.0,
            self.position[2] * -1.0,
        ];
        let pos_matrix = Matrix::trans(pos).rotate(self.rotation);
        // Scale
        let scale_matrix: Matrix = vec![
            vec![self.scale[0], 0.0, 0.0, 0.0],
            vec![0.0, self.scale[1], 0.0, 0.0],
            vec![0.0, 0.0, self.scale[2], 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
        .into();
        let mod_matrix = pos_matrix * scale_matrix;
        let invmod = mod_matrix.inverse().transpose();
        let mvp = view_proj * &mod_matrix;
        draw_model(fb, depth_buffer, &self.model, &mvp, &invmod);
    }
}
//
