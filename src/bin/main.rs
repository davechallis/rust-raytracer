use rand::prelude::*;
use rayon::prelude::*;
use image::{Rgb, ImageBuffer};
use indicatif::{ProgressBar, ProgressStyle};

use rtracer::vec3::Vec3;
use rtracer::ray::Ray;
use rtracer::config::Config;
use rtracer::scenes;
use rtracer::hitable::Hitable;

fn main() {
    let conf = Config::from_cli_args();
    let nx = conf.width();
    let ny = conf.height();
    let ns = conf.samples(); // num samples for antialiasing

    let aspect_ratio = nx as f32 / ny as f32;
    //let scene = scenes::two_spheres(aspect_ratio);
    //let scene = scenes::random_moving_sphere_scene(aspect_ratio);
    //let world: Box<dyn Hitable + Send + Sync> = Box::new(bvh::BvhNode::from_vec(scene.hitables, 0.0, 1.0));
    let scene = scenes::two_perlin_spheres(aspect_ratio);

    let mut imgbuf = ImageBuffer::new(nx, ny);

    let mut coords = Vec::with_capacity((nx * ny) as usize);
    for j in 0..ny {
        for i in 0..nx {
            coords.push((i, j));
        }
    }

    let pb = ProgressBar::new(coords.len() as u64 * u64::from(ns));
     pb.set_style(ProgressStyle::default_bar()
        .template("{elapsed_precise} (eta {eta}) [{wide_bar}] rays:{pos}/{len}")
        .progress_chars("█▉▊▋▌▍▎▏  "));

    let pixels: Vec<(u32, u32, Vec3)> = coords.par_iter()
        .map(|&(i, j)| {
            let j2 = ny - j; // render from bottom up to avoid image needing to be flipped
            let mut rng = rand::thread_rng();
            let mut col = Vec3::new(0.0, 0.0, 0.0); // mean colour over samples
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j2 as f32 + rng.gen::<f32>()) / ny as f32;
                let r = &scene.camera.get_ray(u, v);
                col += colour(&r, &scene.hitables, 0);
            }
            col /= ns as f32;

            pb.inc(u64::from(ns)); (i, j, col)
        })
        .collect();

    for (i, j, col) in pixels {
        imgbuf.put_pixel(i, j, Rgb(to_colour(col)));
    }

    imgbuf.save(conf.output()).unwrap();
    pb.finish_with_message("done");
    println!("Image written to: {}", conf.output().display());

    if conf.inline() {
        let png_data = std::fs::read(conf.output()).unwrap();
        render_inline(&png_data);
    }
}

fn render_inline(img: &[u8]) {
    println!("\x1b]1337;File=;inline=1:{}\x07", base64::encode(img));
}




fn colour(r: &Ray, world: &(dyn Hitable + Send + Sync), depth: usize) -> Vec3 {
    // shadow acne problem - due to numerical inaccuracy, t can be e.g. -0.00000001 or 0.0000001,
    // so ignore values very close to 0
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(hit) => {
            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            }

            match hit.material.scatter(r, &hit) {
                Some((attenuation, scattered)) => {
                    attenuation * colour(&scattered, world, depth + 1)
                },
                None => Vec3::new(0.0, 0.0, 0.0),
            }
        },
        None => {
            let unit_direction = r.direction().to_unit_vector();
            let t = 0.5 * (unit_direction[1] + 1.0);
            // linear blend of colours:
            // blended_value = (1-t) * start_value + t*end_value
            // with 0 <= t <= 1
            (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn to_colour(col: Vec3) -> [u8; 3] {
    [(255.99 * col[0].sqrt()) as u8,
     (255.99 * col[1].sqrt()) as u8,
     (255.99 * col[2].sqrt()) as u8]
}
