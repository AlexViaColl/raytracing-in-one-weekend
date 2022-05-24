use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        from: Point3,
        at: Point3,
        vup: Vec3,
        vfov: f64, // Vertical field-of-view in degrees
        aspect_ratio: f64,
        apperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(from - at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;
        let lens_radius = apperture / 2.0;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
