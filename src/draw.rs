use crate::{
    model::{Material, Model, Vertex},
    types::Matrix,
    window::Framebuffer,
};

pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub fn transform_normal(normal: [f32; 3], invmod: &Matrix) -> [f32; 3] {
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
pub fn project(p: &[f32; 3], mvp: &Matrix) -> ([f32; 3], f32) {
    let mut p4: Matrix = vec![p.to_vec()].into();
    p4[0].push(1.0);
    let proj_pos = &(mvp * &p4)[0];
    let rec = 1.0 / proj_pos[3].abs();
    let rec_pos = proj_pos.iter().map(|a| a * rec).collect::<Vec<_>>();
    ([rec_pos[0], rec_pos[1], rec_pos[2]], rec)
}
pub fn clip_to_screen(clip: [f32; 2], screen_size: &Vec<f32>) -> [f32; 2] {
    [
        (clip[0] * 0.5 + 0.5) * screen_size[0],
        (clip[1] * 0.5 + 0.5) * screen_size[1],
    ]
}
// Le trinagle
pub fn edge(a: &[f32; 2], b: &[f32; 2], p: &[f32; 2]) -> f32 {
    ((p[0] - a[0]) * (b[1] - a[1])) - ((p[1] - a[1]) * (b[0] - a[0]))
}
pub fn draw_line(
    fb: &mut Framebuffer,
    depth_buffer: &mut Framebuffer,
    v0: &[f32; 3],
    v1: &[f32; 3],
    mvp: &Matrix,
) {
    // println!("VLine from {:?} to {:?}", v0, v1);
    let v0_clip = project(v0, mvp);
    let v1_clip = project(v1, mvp);
    // println!("CLine from {:?} to {:?}", v0_clip, v1_clip);
    let screen_size = vec![fb.width() as f32, fb.height() as f32];
    let a = clip_to_screen([v0_clip.0[0], v0_clip.0[1]], &screen_size);
    let b = clip_to_screen([v1_clip.0[0], v1_clip.0[1]], &screen_size);

    let dx = (a[0] - b[0]).abs();
    let dy = (a[1] - b[1]).abs();
    let dist = ((dx * dx) + (dy * dy)).sqrt();
    let dd = (v0_clip.0[2] - v1_clip.0[2]).abs();
    let sx = if a[0] < b[0] { 1 } else { -1 };
    let sy = if a[1] < b[1] { 1 } else { -1 };
    let sd = if v0_clip.0[2] < v1_clip.0[2] {
        1.0
    } else {
        -1.0
    };
    let mut err = dx - dy;

    let mut x = a[0] as i32;
    let mut y = a[1] as i32;
    let xm = b[0] as i32;
    let ym = b[1] as i32;
    println!("Line from {}, {} to {},{}", x, y, xm, ym);
    while x != xm || y != ym {
        if x >= 0 && x < screen_size[0] as i32 && y >= 0 && y < screen_size[1] as i32 {
            let z = depth_buffer.get_pixel_f32(x as usize, y as usize);
            let xd = xm - x;
            let yd = ym - y;
            let ad = (((xd * xd) + (yd * yd)) as f32).sqrt();
            let t = ad / dist;
            let d = v0_clip.0[2] + (dd * t * sd);
            if d < z {
                depth_buffer.set_pixel_f32(x as usize, y as usize, d);
                fb.set_pixel(x as usize, y as usize, rgb_to_u32(255, 0, 0));
            }
        }
        let e2 = err * 2.0;

        if e2 > -dy {
            err -= dy;
            if x != xm {
                x += sx;
            }
        }
        if e2 < dx {
            err += dx;
            if y != ym {
                y += sy;
            }
        }
    }
}
pub fn draw_triangle(
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
    if v0_clip.1 < 0.0 && v1_clip.1 < 0.0 && v2_clip.1 < 0.0 {
        return; // Discard triangle
    }

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
                if !(0.0..=1.0).contains(&z) {
                    continue;
                }
                let d = depth_buffer.get_pixel_f32(x, y);

                if z < d {
                    // println!("{}", z);
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
                        let text = base_color_texture.get_pixel(tex[0], tex[1]);
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
pub fn draw_model(
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
