use rand::prelude::*; 

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utils;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    /// # Arguments
    ///
    /// * `look_from` - location of camera itself
    /// * `look_to` - location the camera is looking at
    /// * `vup` - up vector, controls rotation of camera
    /// * `vertical_fov` - vertical field of view in degrees
    /// * `aspect_ratio` - typically width / height of image size (nx / ny)
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let origin = look_from.clone();
        let w = (look_from - look_at).to_unit_vector();
        let u = vup.cross(&w).to_unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = &origin - &(half_width * focus_dist * &u) - half_height * focus_dist * &v - focus_dist * &w;
        let horizontal = 2.0 * half_width * focus_dist * &u;
        let vertical = 2.0 * half_height * focus_dist * &v;
        Camera { origin, lower_left_corner, horizontal, vertical, u, v, lens_radius, time0, time1 }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let mut rng = thread_rng();
        let ray_disc = self.lens_radius * utils::random_in_unit_disc();
        let offset = &self.u * ray_disc[0] + &self.v * ray_disc[1];
        let time = self.time0 + rng.gen::<f32>() * (self.time1 - self.time0);
        Ray::new_at_time(&self.origin + &offset,
                         &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset,
                         time)
    }
}
