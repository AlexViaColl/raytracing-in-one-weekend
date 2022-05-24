pub mod camera;
pub mod hittable;
pub mod material;
pub mod rand;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::*;
use hittable::{HitRecord, Hittable, HittableList};
use material::*;
use rand::*;
use ray::*;
use sphere::Sphere;
use vec3::*;

#[rustfmt::skip]
fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();

    let material_group = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    //let material_center = Box::new(Dielectric::new(1.5));
    //let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_group)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0), -0.45, material_left)));
    world.add(Box::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    // Camera
    let from = Point3::new(3.0, 3.0, 2.0);
    let at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (from - at).length();
    let apperture = 2.0;
    let cam = Camera::new(from, at, vup, 20.0, aspect_ratio, apperture, dist_to_focus);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in 0..image_width {
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
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat.as_deref().unwrap().scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);

        //let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();  // A Simple Diffuse Material
        //let target = rec.p + rec.normal + Vec3::random_unit_vector();       // True Lambertian Reflection
        //let target = rec.p + Vec3::random_in_hemisphere(rec.normal);      // Alternative Diffuse Formulation (Hemispherical scattering)
        //return ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
