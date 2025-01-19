use crate::{
    model::{Material, Mesh, Model, Vertex},
    physics::Physics,
    texture::{load_texture, Texture},
    types::Matrix,
    window::Framebuffer,
};
use std::sync::{Arc, Mutex};

pub struct Object {
    pub model: Model,
    position: [f32; 3],
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
            println!("{}", i);
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
                println!("{:?} {:?}", position, tex_coord);

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
        println!("{}", vertices.len());

        let mut indices: Vec<u32> = Vec::new();
        for i in 0..6 {
            indices.append(&mut vec![i * 4, i * 4 + 1, (i * 4) + 2]);
            indices.append(&mut vec![i * 4, i * 4 + 2, i * 4 + 3]);
        }
        println!("{:?}", indices);
        // let face_normals = [
        //     [0.0, 0.0, 1.0],  // Front face
        //     [0.0, 0.0, -1.0], // Back face
        //     [-1.0, 0.0, 0.0], // Left face
        //     [1.0, 0.0, 0.0],  // Right face
        //     [0.0, 1.0, 0.0],  // Top face
        //     [0.0, -1.0, 0.0], // Bottom face
        // ];
        // let vertex_to_faces = [
        //     vec![0, 2, 4], // Top-left-front
        //     vec![0, 3, 4], // Top-right-front
        //     vec![1, 2, 4], // Top-left-back
        //     vec![1, 3, 4], // Top-right-back
        //     vec![0, 2, 5], // Bottom-left-front
        //     vec![0, 3, 5], // Bottom-right-front
        //     vec![1, 2, 5], // Bottom-left-back
        //     vec![1, 3, 5], // Bottom-right-back
        // ];
        //
        // for i in 0..8 {
        //     let position = [
        //         0.5 - (((i % 4) > 1) as i32) as f32,
        //         0.5 - (((i > 3) as i32) as f32),
        //         0.5 - ((((i + 1) % 4) > 1) as i32) as f32,
        //     ];
        //
        //     // Average the normals of adjacent faces
        //     let mut normal = [0.0, 0.0, 0.0];
        //     for &face in &vertex_to_faces[i] {
        //         normal[0] += face_normals[face][0];
        //         normal[1] += face_normals[face][1];
        //         normal[2] += face_normals[face][2];
        //     }
        //
        //     // Normalize the resulting normal vector
        //     let length = ((normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2])
        //         as f32)
        //         .sqrt();
        //     normal[0] /= length;
        //     normal[1] /= length;
        //     normal[2] /= length;
        //
        //     let tex_coord = [(i as f32 % 4.0) * 1.0 / 6.0, position[1] + 0.5];
        //
        //     vertices.push(Vertex {
        //         normal,
        //         position,
        //         tex_coord,
        //     });
        // }
        // let mut indices: Vec<u32> = Vec::new();
        // indices.append(&mut vec![0, 1, 2, 0, 2, 3]);
        // indices.append(&mut vec![4, 6, 5, 4, 7, 6]);
        // for i in 0..4 {
        //     indices.append(&mut vec![i, i + 4, (i + 1) % 4]);
        //     indices.append(&mut vec![(i + 1) % 4, i + 4, ((i + 1) % 4 + 4)]);
        // }

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
    // Renders to Framebuffer using its properties and given view-projection matrix
    pub fn render(&self, fb: &mut Framebuffer, depth_buffer: &mut Framebuffer, view_proj: &Matrix) {
        // Transform by position
        let pos = [
            self.position[0] * -1.0,
            self.position[1] * -1.0,
            self.position[2] * -1.0,
        ];
        let pos_matrix = Matrix::view(pos);
        // Apply rotation
        let sina = self.rotation[0].sin();
        let cosa = self.rotation[0].cos();
        let sinb = self.rotation[1].sin();
        let cosb = self.rotation[1].cos();
        let sinc = self.rotation[2].sin();
        let cosc = self.rotation[2].cos();

        let rot_matrix: Matrix = vec![
            vec![
                cosb * cosc,
                (sina * sinb * cosc) - (cosa * sinc),
                (cosa * sinb * cosc) + (sina * sinc),
                0.0,
            ],
            vec![
                cosb * sinc,
                (sina * sinb * sinc) + (cosa * cosc),
                (cosa * sinb * sinc) - (sina * cosc),
                0.0,
            ],
            vec![-sinb, sina * cosb, cosa * cosb, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
        .into();
        // Scale
        let scale_matrix: Matrix = vec![
            vec![self.scale[0], 0.0, 0.0, 0.0],
            vec![0.0, self.scale[1], 0.0, 0.0],
            vec![0.0, 0.0, self.scale[2], 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
        .into();
        let mod_matrix = pos_matrix * (rot_matrix * scale_matrix);
        let invmod = mod_matrix.inverse().transpose();
        let mvp = view_proj * &mod_matrix;
        draw_model(fb, depth_buffer, &self.model, &mvp, &invmod);
    }
}
//
fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn transform_normal(normal: [f32; 3], invmod: &Matrix) -> [f32; 3] {
    // Convert normal to homogeneous coordinates (4D)
    let normal_homogeneous = [normal[0], normal[1], normal[2], 0.0]; // The fourth component is 0 for normals

    // Multiply by the inverse-transpose matrix (4x4)

    [
        normal_homogeneous[0] * invmod[0][0]
            + normal_homogeneous[1] * invmod[1][0]
            + normal_homogeneous[2] * invmod[2][0]
            + normal_homogeneous[3] * invmod[3][0],
        normal_homogeneous[0] * invmod[0][1]
            + normal_homogeneous[1] * invmod[1][1]
            + normal_homogeneous[2] * invmod[2][1]
            + normal_homogeneous[3] * invmod[3][1],
        normal_homogeneous[0] * invmod[0][2]
            + normal_homogeneous[1] * invmod[1][2]
            + normal_homogeneous[2] * invmod[2][2]
            + normal_homogeneous[3] * invmod[3][2],
    ]
}
// This shit
fn project(p: &[f32; 3], mvp: &Matrix) -> ([f32; 3], f32) {
    let mut p4: Matrix = vec![p.to_vec()].into();
    p4[0].push(1.0);
    let proj_pos = &(mvp * &p4)[0];
    let rec = 1.0 / proj_pos[3];
    let rec_pos = proj_pos.iter().map(|a| a * rec).collect::<Vec<_>>();
    ([rec_pos[0], rec_pos[1], rec_pos[2]], rec)
}
fn clip_to_screen(clip: [f32; 2], screen_size: &Vec<f32>) -> [f32; 2] {
    [
        (clip[0] * 0.5 + 0.5) * screen_size[0],
        (clip[1] * 0.5 + 0.5) * screen_size[1],
    ]
}
// Le trinagle
fn edge(a: &[f32; 2], b: &[f32; 2], p: &[f32; 2]) -> f32 {
    ((p[0] - a[0]) * (b[1] - a[1])) - ((p[1] - a[1]) * (b[0] - a[0]))
}
fn draw_triangle(
    fb: &mut Framebuffer,
    depth_buffer: &mut Framebuffer,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    mvp: &Matrix,
    invmod: &Matrix,
    mat: &Material,
) {
    let v0_clip = project(&v0.position, mvp);
    let v1_clip = project(&v1.position, mvp);
    let v2_clip = project(&v2.position, mvp);

    let screen_size = vec![fb.width() as f32, fb.height() as f32];
    let a = clip_to_screen([v0_clip.0[0], v0_clip.0[1]], &screen_size);
    let b = clip_to_screen([v1_clip.0[0], v1_clip.0[1]], &screen_size);
    let c = clip_to_screen([v2_clip.0[0], v2_clip.0[1]], &screen_size);

    // println!("{:?}\n{:?}\n{:?}\n", a, b, c);

    let xs = (a[0].min(b[0]).min(c[0]).max(0.0)).floor() as usize;
    let ys = (a[1].min(b[1]).min(c[1]).max(0.0)).floor() as usize;
    let xl = (a[0].max(b[0]).max(c[0]).min(screen_size[0] - 1.0)).ceil() as usize;
    let yl = (a[1].max(b[1]).max(c[1]).min(screen_size[1] - 1.0)).ceil() as usize;
    for x in xs..xl {
        for y in ys..yl {
            let p: [f32; 2] = [x as f32, y as f32];
            let a0 = edge(&b, &c, &p);
            let a1 = edge(&c, &a, &p);
            let a2 = edge(&a, &b, &p);

            let inside = a0 < 0.0 && a1 < 0.0 && a2 < 0.0;
            if inside {
                let area_rep = 1.0 / edge(&a, &b, &c);
                let bary0 = a0 * area_rep;
                let bary1 = a1 * area_rep;
                let bary2 = a2 * area_rep;
                let correction = 1.0 / (bary0 * v0_clip.1 + bary1 * v1_clip.1 + bary2 * v2_clip.1);

                //println!("{:?}", v1_clip.0);
                let z = (v0_clip.0[2] * bary0) + (v1_clip.0[2] * bary1) + (v2_clip.0[2] * bary2);
                //println!("{}", z);
                let d = depth_buffer.get_pixel_f32(x, y);

                if z < d {
                    depth_buffer.set_pixel_f32(x, y, z);
                    // Directly interpolate normals
                    let normal = [
                        (v0.normal[0] * v0_clip.1 * bary0
                            + v1.normal[0] * v1_clip.1 * bary1
                            + v2.normal[0] * v2_clip.1 * bary2)
                            * correction,
                        (v0.normal[1] * v0_clip.1 * bary0
                            + v1.normal[1] * v1_clip.1 * bary1
                            + v2.normal[1] * v2_clip.1 * bary2)
                            * correction,
                        (v0.normal[2] * v0_clip.1 * bary0
                            + v1.normal[2] * v1_clip.1 * bary1
                            + v2.normal[2] * v2_clip.1 * bary2)
                            * correction,
                    ];

                    // Normalize the interpolated normal
                    let len = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
                    let normal = [normal[0] / len, normal[1] / len, normal[2] / len];

                    let normal = transform_normal(normal, &invmod);

                    let tex = [
                        (v0.tex_coord[0] * v0_clip.1 * bary0
                            + v1.tex_coord[0] * v1_clip.1 * bary1
                            + v2.tex_coord[0] * v2_clip.1 * bary2)
                            * correction,
                        (v0.tex_coord[1] * v0_clip.1 * bary0
                            + v1.tex_coord[1] * v1_clip.1 * bary1
                            + v2.tex_coord[1] * v2_clip.1 * bary2)
                            * correction,
                    ];

                    // Scale the normal to color space
                    let norm_col = [
                        ((normal[0] * 0.5 + 0.5) * 255.99) as u8,
                        ((normal[1] * 0.5 + 0.5) * 255.99) as u8,
                        ((normal[2] * 0.5 + 0.5) * 255.99) as u8,
                    ];

                    let mut base_color = mat.base_col;
                    if let Some(base_color_texture) = &mat.base_color_texture {
                        let text = base_color_texture.sample_pixel(tex[0], tex[1]);
                        base_color = [
                            base_color[0] * text[0],
                            base_color[1] * text[1],
                            base_color[2] * text[2],
                            base_color[3] * text[3],
                        ];
                    };
                    let light = [0.3, -0.7, 0.5];
                    let intensity =
                        (normal[0] * light[0] + normal[1] * light[1] + normal[2] * light[2])
                            .clamp(0.2, 1.0);
                    let color = rgb_to_u32(
                        (base_color[0] * intensity * 255.99) as u8,
                        (base_color[1] * intensity * 255.99) as u8,
                        (base_color[2] * intensity * 255.99) as u8,
                    );
                    fb.set_pixel(
                        x, y,
                        color,
                        //rgb_to_u32(norm_col[0] as u8, norm_col[1] as u8, norm_col[2] as u8),
                    );
                }
            }
        }
    }
}
fn draw_model(
    fb: &mut Framebuffer,
    depth_buffer: &mut Framebuffer,
    model: &Model,
    mvp: &Matrix,
    invmod: &Matrix,
) {
    for mesh in &model.meshes {
        for i in 0..(mesh.indices.len() / 3) {
            let v0 = mesh.vertices[mesh.indices[i * 3] as usize];
            let v1 = mesh.vertices[mesh.indices[i * 3 + 1] as usize];
            let v2 = mesh.vertices[mesh.indices[i * 3 + 2] as usize];

            draw_triangle(
                fb,
                depth_buffer,
                &v0,
                &v1,
                &v2,
                mvp,
                invmod,
                &model.mats[mesh.material_idx],
            );
        }
    }
}
