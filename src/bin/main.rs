use rand::prelude::*;
use rayon::prelude::*;
use image::{Pixel, Rgb, GenericImage, ImageBuffer};

use rtracer::vec3::Vec3;
use rtracer::ray::Ray;
use rtracer::material::{Dielectric, Material, Metal, Lambertian};
use rtracer::hitable::{HitRecord, Hitable};
use rtracer::camera::Camera;

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10; // num samples for antialiasing

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up_vector = Vec3::new(0.0, 1.0, 0.0);
    let field_of_view = 20.0;
    let aspect_ratio = nx as f32 / ny as f32;
    let focal_distance = 10.0; //(look_from - look_at).length();
    let aperture = 0.1;
    let camera = Camera::new(look_from, look_at, up_vector, field_of_view, aspect_ratio, aperture, focal_distance);

    let world = random_sphere_scene();

//    let world: Vec<Box<dyn Hitable>> = vec![
//        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
//        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
//        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))),
//        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5))),
//        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Dielectric::new(1.5))),
//    ];
//    //let r = (std::f32::consts::PI / 4.0).cos();
    //let world: Vec<Box<dyn Hitable>> = vec![
        //Box::new(Sphere::new(Vec3::new(-r, 0.0, -1.0), r, Lambertian::new(Vec3::new(0.0, 0.0, 1.0)))),
        //Box::new(Sphere::new(Vec3::new(r, 0.0, -1.0), r, Lambertian::new(Vec3::new(1.0, 0.0, 0.0)))),
    //];

    let mut imgbuf = ImageBuffer::new(nx, ny);

    let mut coords = Vec::with_capacity((nx * ny) as usize);
    for j in (0..ny).rev() {
        for i in 0..nx {
            coords.push((i, j));
        }
    }
    let pixels: Vec<(u32, u32, Rgb<u8>)> = coords.par_iter()
        .map(|&(i, j)| {
            let mut rng = rand::thread_rng();
            let mut col = Vec3::new(0.0, 0.0, 0.0); // mean colour over samples
            for _ in 0..ns {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / ny as f32;
                let r = camera.get_ray(u, v);
                col += colour(&r, world.as_slice(), 0);
            }
            col /= ns as f32;

            (i, j, Rgb(to_colour(col)))
        })
        .collect();

    for (i, j, pixel) in pixels {
        imgbuf.put_pixel(i, j, pixel);
    }

    //imgbuf.save("/tmp/o.png").unwrap();
}

fn random_sphere_scene() -> Vec<Box<dyn Hitable + Send + Sync>> {
    let mut rng = rand::thread_rng();
    let n = 500;
    let sphere_radius = 0.2;
    let mut list: Vec<Box<dyn Hitable + Send + Sync>> = Vec::with_capacity(n + 1);

    // giant sphere for ground
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Vec3::new(a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                                   sphere_radius,
                                   b as f32 + 0.9 * rng.gen_range(0.0, 1.0));

            if (center - Vec3::new(4.0, sphere_radius, 0.0)).length() > 0.9 {
                let sphere: Box<dyn Hitable + Send + Sync> = {
                    if choose_mat < 0.8 { // diffuse
                        let albedo = Vec3::new(rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                                               rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                                               rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0));
                        Box::new(Sphere::new(center, sphere_radius, Lambertian::new(albedo)))
                    } else if choose_mat < 0.95 { // metal
                        let albedo = Vec3::new(rng.gen_range(0.5, 1.0),
                                               rng.gen_range(0.5, 1.0),
                                               rng.gen_range(0.5, 1.0));
                        let fuzz = 0.5 * rng.gen_range(0.0, 1.0);
                        Box::new(Sphere::new(center, sphere_radius, Metal::new(albedo, fuzz)))
                    } else { // glass
                        Box::new(Sphere::new(center, sphere_radius, Dielectric::new(1.5)))
                    }
                };

                list.push(sphere);
            }
        }
    }
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5))));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    list
}

fn colour(r: &Ray, world: &[Box<dyn Hitable + Send + Sync>], depth: usize) -> Vec3 {
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
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn print_header(nx: u32, ny: u32) {
    println!("P3\n{} {}\n255", nx, ny);
}

fn print_colour(col: Vec3) {
    // square root for gamma correction - raise to power of 1/gamma, e.g. sqrt in this case
    println!("{} {} {}",
             (255.99 * col[0].sqrt()) as i32,
             (255.99 * col[1].sqrt()) as i32,
             (255.99 * col[2].sqrt()) as i32)
}

fn to_colour(col: Vec3) -> [u8; 3] {
    [(255.99 * col[0].sqrt()) as u8,
     (255.99 * col[1].sqrt()) as u8,
     (255.99 * col[2].sqrt()) as u8]
}



struct Sphere<M: Material> {
    center: Vec3,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    fn new(center: Vec3, radius: f32, material: M) -> Self {
        Sphere { center, radius, material }
    }

    fn surface_normal(&self, p: &Vec3) -> Vec3 {
        (*p - self.center) / self.radius
    }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center; // vector from ray source to sphere center
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let normal = self.surface_normal(&p);
                return Some(HitRecord { t, p, normal, material: &self.material });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let normal = self.surface_normal(&p);
                return Some(HitRecord { t, p, normal, material: &self.material });
            }
        }

        None
    }
}
