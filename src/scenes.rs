//! Defines a few scenes for testing image outputs.

use rand::prelude::*;
use crate::vec3::Vec3;
use crate::material::{Dielectric, DiffuseLight, Metal, Lambertian};
use crate::texture;
use crate::hitable::{Rotate, Translate, Cuboid, FlipNormals, Rectangle, Hitable, MovingSphere, Sphere};
use crate::camera::Camera;
use crate::bvh;

pub struct Scene<T: Hitable + Send + Sync> {
    pub camera: Camera,
    pub hitables: T,
}

//pub fn random_sphere_scene() -> Vec<Box<dyn Hitable + Send + Sync>> {
//    let mut rng = rand::thread_rng();
//    let n = 500;
//    let sphere_radius = 0.2;
//    let mut list: Vec<Box<dyn Hitable + Send + Sync>> = Vec::with_capacity(n + 1);
//
//    // giant sphere for ground
//    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(texture::Constant::from_rgb(0.5, 0.5, 0.5)))));
//
//    for a in -11..11 {
//        for b in -11..11 {
//            let choose_mat = rng.gen::<f32>();
//            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(),
//                                   sphere_radius,
//                                   b as f32 + 0.9 * rng.gen::<f32>());
//
//            if (&center - Vec3::new(4.0, sphere_radius, 0.0)).length() > 0.9 {
//                let sphere: Box<dyn Hitable + Send + Sync> = {
//                    if choose_mat < 0.7 { // diffuse
//                        let albedo = texture::Constant::from_rgb(
//                            rng.gen::<f32>() * rng.gen::<f32>(),
//                            rng.gen::<f32>() * rng.gen::<f32>(),
//                            rng.gen::<f32>() * rng.gen::<f32>()
//                        );
//                        Box::new(Sphere::new(center, sphere_radius, Lambertian::new(albedo)))
//                    } else if choose_mat < 0.90 { // metal
//                        let albedo = texture::Constant::from_rgb(
//                            rng.gen_range(0.5, 1.0),
//                            rng.gen_range(0.5, 1.0),
//                            rng.gen_range(0.5, 1.0)
//                        );
//                        let fuzz = 0.5 * rng.gen::<f32>();
//                        Box::new(Sphere::new(center, sphere_radius, Metal::new(albedo, fuzz)))
//                    } else { // glass
//                        Box::new(Sphere::new(center, sphere_radius, Dielectric::new(1.5)))
//                    }
//                };
//
//                list.push(sphere);
//            }
//        }
//    }
//    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &Dielectric::new(1.5))));
//    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &Lambertian::new(texture::Constant::from_rgb(0.4, 0.2, 0.1)))));
//    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &Metal::new(texture::Constant::from_rgb(0.7, 0.6, 0.5), 0.0))));
//
//    list
//}
//
//pub fn random_moving_sphere_scene(aspect_ratio: f32) -> Scene<bvh::BvhNode> {
//    let look_from = Vec3::new(13.0, 2.0, 3.0);
//    let look_at = Vec3::zeros();
//    let up_vector = Vec3::new(0.0, 1.0, 0.0);
//    let field_of_view = 20.0;
//    let aperture = 0.0;
//    let focal_distance = 10.0;
//    let time0 = 0.0;
//    let time1 = 1.0;
//
//    let camera = Camera::new(look_from, look_at,
//                             up_vector, field_of_view, aspect_ratio, aperture, focal_distance,
//                             time0, time1);
//
//    let mut rng = rand::thread_rng();
//    let n = 500;
//    let sphere_radius = 0.2;
//    let mut list: Vec<Box<dyn Hitable + Send + Sync>> = Vec::with_capacity(n + 1);
//
//    // giant sphere for ground
//    let checker = texture::Checker::new(
//        texture::Constant::from_rgb(0.2, 0.3, 0.1),
//        texture::Constant::from_rgb(0.9, 0.9, 0.9),
//    );
//    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(checker))));
//
//    for a in -11..11 {
//        for b in -11..11 {
//            let choose_mat = rng.gen::<f32>();
//            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(),
//                                   sphere_radius,
//                                   b as f32 + 0.9 * rng.gen::<f32>());
//
//            if (&center - Vec3::new(4.0, sphere_radius, 0.0)).length() > 0.9 {
//                let sphere: Box<dyn Hitable + Send + Sync> = {
//                    if choose_mat < 0.7 { // diffuse
//                        let albedo = texture::Constant::from_rgb(
//                            rng.gen::<f32>() * rng.gen::<f32>(),
//                            rng.gen::<f32>() * rng.gen::<f32>(),
//                            rng.gen::<f32>() * rng.gen::<f32>()
//                        );
//                        let center1 = &center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
//                        Box::new(MovingSphere::new(center, center1, 0.0, 1.0, sphere_radius, Lambertian::new(albedo)))
//                    } else if choose_mat < 0.90 { // metal
//                        let albedo = texture::Constant::from_rgb(
//                            rng.gen_range(0.5, 1.0),
//                            rng.gen_range(0.5, 1.0),
//                            rng.gen_range(0.5, 1.0)
//                        );
//                        let fuzz = 0.5 * rng.gen::<f32>();
//                        Box::new(Sphere::new(center, sphere_radius, Metal::new(albedo, fuzz)))
//                    } else { // glass
//                        Box::new(Sphere::new(center, sphere_radius, Dielectric::new_glass()))
//                    }
//                };
//
//                list.push(sphere);
//            }
//        }
//    }
//    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new_glass())));
//    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(texture::Constant::from_rgb(0.4, 0.2, 0.1)))));
//    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::new(texture::Constant::from_rgb(0.7, 0.6, 0.5), 0.0))));
//
//    let hitables = bvh::BvhNode::from_vec(list, time0, time1);
//    Scene { camera, hitables }
//}
//
//pub fn two_spheres(aspect_ratio: f32) -> Scene<bvh::BvhNode> {
//    let look_from = Vec3::new(13.0, 2.0, 3.0);
//    let look_at = Vec3::zeros();
//    let up_vector = Vec3::new(0.0, 1.0, 0.0);
//    let field_of_view = 20.0;
//    let aperture = 0.0;
//    let focal_distance = 10.0;
//    let time0 = 0.0;
//    let time1 = 1.0;
//
//    let camera = Camera::new(look_from, look_at,
//                             up_vector, field_of_view, aspect_ratio, aperture, focal_distance,
//                             time0, time1);
//
//    let checker = texture::Checker::new(
//        texture::Constant::from_rgb(0.2, 0.3, 0.1),
//        texture::Constant::from_rgb(0.9, 0.9, 0.9),
//    );
//
//    let hitables: Vec<Box<dyn Hitable + Send + Sync>> = vec![
//        Box::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Lambertian::new(checker.clone()))),
//        Box::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Lambertian::new(checker.clone()))),
//    ];
//    let hitables = bvh::BvhNode::from_vec(hitables, time0, time1);
//    Scene { camera, hitables }
//}
//
//pub fn two_perlin_spheres(aspect_ratio: f32) -> Scene<bvh::BvhNode> {
//    let look_from = Vec3::new(13.0, 2.0, 3.0);
//    let look_at = Vec3::zeros();
//    let up_vector = Vec3::new(0.0, 1.0, 0.0);
//    let field_of_view = 20.0;
//    let aperture = 0.0;
//    let focal_distance = 10.0;
//    let time0 = 0.0;
//    let time1 = 1.0;
//
//    let camera = Camera::new(look_from, look_at,
//                             up_vector, field_of_view, aspect_ratio, aperture, focal_distance,
//                             time0, time1);
//
//    let noise = texture::Noise::new(4.0);
//    let hitables: Vec<Box<dyn Hitable + Send + Sync>> = vec![
//        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(noise.clone()))),
//        Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(noise.clone()))),
//    ];
//    let hitables = bvh::BvhNode::from_vec(hitables, time0, time1);
//    Scene { camera, hitables }
//}
//
//pub fn earth_sphere(aspect_ratio: f32) -> Scene<bvh::BvhNode> {
//    let look_from = Vec3::new(0.0, 10.0, 10.0);
//    let look_at = Vec3::new(0.0, 2.0, 0.0);
//    let up_vector = Vec3::new(0.0, 1.0, 0.0);
//    let field_of_view = 20.0;
//    let aperture = 0.0;
//    let focal_distance = 10.0;
//    let time0 = 0.0;
//    let time1 = 1.0;
//
//    let camera = Camera::new(look_from, look_at,
//                             up_vector, field_of_view, aspect_ratio, aperture, focal_distance,
//                             time0, time1);
//
//    let earth_img = texture::Image::new("/Users/dsc/src/rust-raytracer/earthmap1k.jpg");
//    let hitables: Vec<Box<dyn Hitable + Send + Sync>> = vec![
//        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(texture::Noise::new(4.0)))),
//        Box::new(Sphere::new(Vec3::new(1.5, 2.0, 0.0), 1.5, Lambertian::new(earth_img.clone()))),
//        Box::new(Sphere::new(Vec3::new(-1.5, 2.0, 0.0), 1.5, Metal::new(earth_img.clone(), 0.9))),
//    ];
//    let hitables = bvh::BvhNode::from_vec(hitables, time0, time1);
//    Scene { camera, hitables }
//}
//
//pub fn simple_light(aspect_ratio: f32) -> Scene<bvh::BvhNode> {
//    let look_from = Vec3::new(16.0, 5.0, 3.0);
//    let look_at = Vec3::new(0.0, 2.0, 0.0);
//    let up_vector = Vec3::new(0.0, 1.0, 0.0);
//    let field_of_view = 30.0;
//    let aperture = 0.0;
//    let focal_distance = 10.0;
//    let time0 = 0.0;
//    let time1 = 1.0;
//    let camera = Camera::new(look_from, look_at,
//                             up_vector, field_of_view, aspect_ratio, aperture, focal_distance,
//                             time0, time1);
//
//    let noise = texture::Noise::new(4.0);
//    let light = texture::Constant::from_rgb(6.0, 0.0, 0.0); // light > 1.0, bright enough to light things
//    let blue_light = texture::Constant::from_rgb(0.0, 4.0, 0.0); // light > 1.0, bright enough to light things
//    let hitables: Vec<Box<dyn Hitable + Send + Sync>> = vec![
//        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(noise.clone()))),
//        Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(noise.clone()))),
//        Box::new(Sphere::new(Vec3::new(1.0, 6.0, 02.0), 0.5, DiffuseLight::new(light.clone()))),
//        Box::new(Rectangle::new_xy((3.0, 5.0), (1.0, 3.0), -2.0, DiffuseLight::new(blue_light.clone()))),
//    ];
//    let hitables = bvh::BvhNode::from_vec(hitables, time0, time1);
//    Scene { camera, hitables }
//}

pub fn cornell_box(aspect_ratio: f32) -> Scene<bvh::BvhNode> {
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let up_vector = Vec3::new(0.0, 1.0, 0.0);
    let field_of_view = 40.0;
    let aperture = 0.0;
    let focal_distance = 10.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let camera = Camera::new(look_from, look_at,
                             up_vector, field_of_view, aspect_ratio, aperture, focal_distance,
                             time0, time1);

    let red = Lambertian::new(texture::Constant::from_rgb(0.65, 0.05, 0.05));
    let white = Lambertian::new(texture::Constant::from_rgb(0.73, 0.73, 0.73));
    let green = Lambertian::new(texture::Constant::from_rgb(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(texture::Constant::from_rgb(15.0, 15.0, 15.0));

    let hitables: Vec<Box<dyn Hitable + Send + Sync>> = vec![
        Box::new(FlipNormals::new(Rectangle::new_yz((0.0, 555.0), (0.0, 555.0), 555.0, green.clone()))),
        Box::new(Rectangle::new_yz((0.0, 555.0), (0.0, 555.0), 0.0, red.clone())),
        Box::new(Rectangle::new_xz((213.0, 343.0), (227.0, 332.0), 554.0, light.clone())),
        Box::new(FlipNormals::new(Rectangle::new_xz((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()))),
        Box::new(Rectangle::new_xz((0.0, 555.0), (0.0, 555.0), 0.0, white.clone())),
        Box::new(FlipNormals::new(Rectangle::new_xy((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()))),
        Box::new(
            Translate::new(
                Rotate::new_y(
                    Cuboid::new(Vec3::zeros(), Vec3::new(165.0, 165.0, 165.0), white.clone()),
                    -18.0
                ),
                Vec3::new(130.0, 0.0, 65.0)
            )
        ),
        Box::new(
            Translate::new(
                Rotate::new_y(
                    Cuboid::new(Vec3::zeros(), Vec3::new(165.0, 330.0, 165.0), white.clone()),
                    15.0
                ),
                Vec3::new(265.0, 0.0, 295.0)
            )
        ),
    ];
    let hitables = bvh::BvhNode::from_vec(hitables, time0, time1);
    Scene { camera, hitables }
}

