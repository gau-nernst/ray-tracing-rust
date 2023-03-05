mod camera;
mod color;
mod material;
mod rand;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use material::Material;
use rand::RandomGenerator;
use ray::Ray;
use sphere::{HitRecord, Sphere};
use vec3::Vec3;

fn ray_color(ray: &Ray, spheres: &Vec<Sphere>, depth: i32, random: &mut RandomGenerator) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    let mut hit_record = HitRecord {
        t: f64::MAX,
        incidence: Vec3::zero(),
        normal: Vec3::zero(),
        front_face: false,
        material: Material::None,
    };
    for sphere in spheres {
        // shadow acne
        let current_hit_record = sphere.hit(ray, 0.0001, hit_record.t);
        if current_hit_record.is_some() {
            hit_record = current_hit_record.unwrap();
        }
    }
    if hit_record.t < f64::MAX {
        match hit_record
            .material
            .scatter(ray.direction, &hit_record, random)
        {
            Some(scatter) => {
                scatter.attenuation * ray_color(&scatter.ray, spheres, depth - 1, random)
            }
            None => Vec3::zero(),
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let white = Vec3::one();
        let blue = Vec3::new(0.5, 0.7, 1.0);
        white + t * (blue - white)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_w = 400;
    let img_h = (img_w as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_to = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        look_from,
        look_to,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        2.0,
        (look_from - look_to).length(),
    );
    let random = &mut RandomGenerator::new(0);

    let spheres = vec![
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian(Vec3::new(0.8, 0.8, 0.0)),
        ),
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Vec3::new(0.1, 0.2, 0.5)),
        ),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric(1.5)),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, Material::Dielectric(1.5)),
        Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.1),
        ),
    ];

    println!("P3\n{img_w} {img_h}\n255");

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..img_w {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random.rand()) / img_w as f64;
                let v = (j as f64 + random.rand()) / img_h as f64;
                let r = camera.get_ray(u, v, random);
                pixel_color += ray_color(&r, &spheres, max_depth, random);
            }
            pixel_color /= samples_per_pixel as f64;
            write_color(&pixel_color);
        }
    }
    eprintln!("\nDone.")
}
