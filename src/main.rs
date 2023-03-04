mod camera;
mod color;
mod rand;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use rand::RandomGenerator;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn ray_color(ray: &Ray, spheres: &Vec<Sphere>) -> Vec3 {
    for sphere in spheres {
        let hit_record = sphere.hit(ray, 0.0, f64::MAX);
        if hit_record.is_some() {
            return 0.5 * (hit_record.unwrap().normal + 1.0);
        }
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Vec3::one();
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white + t * (blue - white)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_w = 400;
    let img_h = (img_w as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let camera = Camera::new(aspect_ratio);
    let mut random = RandomGenerator::new(0);

    let spheres = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    println!("P3\n{img_w} {img_h}\n255");

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..img_w {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random.rand()) / img_w as f64;
                let v = (j as f64 + random.rand()) / img_h as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &spheres);
            }
            pixel_color /= samples_per_pixel as f64;
            write_color(&pixel_color);
        }
    }
    eprintln!("\nDone.")
}
