use image::{GenericImageView, ImageReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub channels: usize,
}
impl Texture {
    pub fn sample_pixel(&self, x: f32, y: f32) -> [f32; 4] {
        let invx = 1.0 / self.width as f32;
        let invy = 1.0 / self.height as f32;

        let tl = self.get_pixel(x - invx, y - invy);
        let tr = self.get_pixel(x + invx, y - invy);
        let bl = self.get_pixel(x - invx, y + invy);
        let br = self.get_pixel(x + invx, y + invy);

        let x = x * self.width as f32;
        let y = y * self.height as f32;
        let dx = x - ((x as i32) as f32);
        let dy = y - ((y as i32) as f32);

        let bot = [
            bl[0] + ((br[0] - bl[0]) * dx),
            bl[1] + ((br[1] - bl[1]) * dx),
            bl[2] + ((br[2] - bl[2]) * dx),
            bl[3] + ((br[3] - bl[3]) * dx),
        ];
        let top = [
            tl[0] + ((tr[0] - tl[0]) * dx),
            tl[1] + ((tr[1] - tl[1]) * dx),
            tl[2] + ((tr[2] - tl[2]) * dx),
            tl[3] + ((tr[3] - tl[3]) * dx),
        ];
        [
            bot[0] + ((top[0] - bot[0]) * dy),
            bot[1] + ((top[1] - bot[1]) * dy),
            bot[2] + ((top[2] - bot[2]) * dy),
            bot[3] + ((top[3] - bot[3]) * dy),
        ]
    }
    pub fn get_pixel(&self, x: f32, y: f32) -> [f32; 4] {
        let mut x = x;
        let mut y = y;
        // println!("{} {}", self.width, self.height);
        // println!("{} {}", x, y);
        x *= self.width as f32;
        y *= self.height as f32;
        //println!("{} {}", x, y);
        let x = x as u32;
        let y = y as u32;
        // println!("{} {}", x, y);
        let x = x % (self.width - 1);
        let y = y % (self.height - 1);
        // println!("{} {}", x, y);
        let x = x as usize;
        let y = y as usize;
        //println!("{} {}", x, y);
        match self.channels {
            4 => {
                let data: &Vec<(u8, u8, u8, u8)> = unsafe { std::mem::transmute(&self.data) };
                let pixel = &data[x + y * self.width as usize];
                [
                    pixel.0 as f32 / 255.99,
                    pixel.1 as f32 / 255.99,
                    pixel.2 as f32 / 255.99,
                    pixel.3 as f32 / 255.99,
                ]
            }
            3 => {
                let data: &Vec<(u8, u8, u8)> = unsafe { std::mem::transmute(&self.data) };
                let pixel = &data[x + y * self.width as usize];
                [
                    pixel.0 as f32 / 255.99,
                    pixel.1 as f32 / 255.99,
                    pixel.2 as f32 / 255.99,
                    1.0,
                ]
            }
            _ => panic!("Invalid texture channel count"),
        }
    }
}

pub fn load_texture(path: &str) -> Texture {
    let path = Path::new(path);
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    Texture {
        width: img.dimensions().0,
        height: img.dimensions().1,
        channels: img.color().bytes_per_pixel() as usize,
        data: img.into_bytes(),
    }
}
