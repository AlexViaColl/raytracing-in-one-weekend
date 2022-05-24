pub mod camera;
pub mod hittable;
pub mod rand;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::*;
use hittable::{HitRecord, Hittable, HittableList};
use rand::*;
use ray::*;
use sphere::Sphere;
use vec3::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in (0..image_width).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.")
}

#[rustfmt::skip]
fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, f64::INFINITY, &mut rec) {
        //let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();  // A Simple Diffuse Material
        let target = rec.p + rec.normal + Vec3::random_unit_vector();       // True Lambertian Reflection
        //let target = rec.p + Vec3::random_in_hemisphere(rec.normal);      // Alternative Diffuse Formulation
        return ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
