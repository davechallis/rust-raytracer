use std::path::Path;
use image::GenericImageView;
use image::Pixel;
use crate::vec3::Vec3;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Image {
    im: image::DynamicImage,
    nx: u32,
    ny: u32,
}

impl Image {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let im = image::open(path).unwrap();
        let (nx, ny) = im.dimensions();
        Self { im, nx, ny }
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _point: &Vec3) -> Vec3 {
        let nx = self.nx as f32;
        let ny = self.ny as f32;

        let mut i = u * nx;
        let mut j = (1.0 - v) * ny - 0.001;

        if i < 0.0 {
            i = 0.0;
        }

        if j < 0.0 {
            j = 0.0;
        }

        if i > (nx - 1.0) {
            i = nx - 1.0;
        }

        if j > (ny - 1.0) {
            j = ny - 1.0;
        }

        let rgb = self.im.get_pixel(i as u32, j as u32).to_rgb();
        Vec3::new(rgb[0] as f32 / 255.0,
                  rgb[1] as f32 / 255.0,
                  rgb[2] as f32 / 255.0)
    }
}