use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::*;
use super::vec3::*;

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Option<Box<dyn Material>>,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64, m: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat: Some(m),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) * (1.0 / self.radius);
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
