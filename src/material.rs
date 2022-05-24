use super::hittable::HitRecord;
use super::rand::*;
use super::ray::Ray;
use super::vec3::*;

pub trait Material: MaterialClone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}
#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}
