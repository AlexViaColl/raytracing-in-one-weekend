use super::material::Material;
use super::ray::*;
use super::vec3::*;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Box<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
