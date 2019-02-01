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
    w: Vec3,
    lens_radius: f32,
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
        focus_dist: f32
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).to_unit_vector();
        let u = vup.cross(&w).to_unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let lower_left_corner = origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let ray_disc = self.lens_radius * utils::random_in_unit_disc();
        let offset = self.u * ray_disc[0] + self.v * ray_disc[1];
        Ray::new(self.origin + offset,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}
