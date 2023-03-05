mod camera;
mod color;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use material::Material;
use rand::prelude::*;
use ray::Ray;
use sphere::{HitRecord, Sphere};
use vec3::Vec3;

fn ray_color(ray: &Ray, spheres: &Vec<Sphere>, depth: i32) -> Vec3 {
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
        match hit_record.material.scatter(ray.direction, &hit_record) {
            Some(scatter) => scatter.attenuation * ray_color(&scatter.ray, spheres, depth - 1),
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

fn generate_spheres() -> Vec<Sphere> {
    let mut spheres = vec![
        Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::Lambertian(Vec3::new(0.5, 0.5, 0.5)),
        ),
        Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric(1.5)),
        Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::Lambertian(Vec3::new(0.4, 0.2, 0.1)),
        ),
        Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
        ),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material;
                let choose_mat = random::<f64>();
                if choose_mat < 0.8 {
                    material = Material::Lambertian(Vec3::rand() * Vec3::rand());
                } else if choose_mat < 0.95 {
                    material = Material::Metal(
                        Vec3::rand_between(0.5, 1.0),
                        thread_rng().gen_range(0.0..0.5),
                    )
                } else {
                    material = Material::Dielectric(1.5);
                }
                spheres.push(Sphere::new(center, 0.2, material));
            }
        }
    }
    spheres
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let img_w = 400;
    let img_h = (img_w as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let spheres = generate_spheres();

    println!("P3\n{img_w} {img_h}\n255");

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..img_w {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / img_w as f64;
                let v = (j as f64 + random::<f64>()) / img_h as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &spheres, max_depth);
            }
            pixel_color /= samples_per_pixel as f64;
            write_color(&pixel_color);
        }
    }
    eprintln!("\nDone.")
}
