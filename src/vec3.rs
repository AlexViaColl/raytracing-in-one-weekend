use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Neg, Sub};

use super::rand::{random_double, random_double_range};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}
pub type Point3 = Vec3;
pub type Color = Vec3;
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    // Static methods
    pub fn random() -> Self {
        Self::new(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Self {
        unit_vector(Self::random_in_unit_sphere())
    }
    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, v: Vec3) -> Self {
        Self::new(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, v: Self) -> Self {
        Self::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, v: Self) -> Self {
        Self::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}
pub fn unit_vector(v: Vec3) -> Vec3 {
    let f = 1.0 / v.length();
    v * f
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v, n)
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32
    );
}
