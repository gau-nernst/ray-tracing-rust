mod camera;
mod material;
mod ray;
mod sphere;
mod tiff;
mod vec3;

use std::time::Instant;

use camera::Camera;
use material::Material;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use tiff::TiffFile;
use vec3::Vec3;

fn hit_spheres(ray: &Ray, spheres: &Vec<Sphere>) -> (usize, f64) {
    let mut sphere_idx = 0;
    let mut t_max = f64::MAX;
    for (idx, sphere) in spheres.iter().enumerate() {
        let t = sphere.hit(ray, 0.0001, t_max);
        if t < t_max {
            sphere_idx = idx;
            t_max = t;
        }
    }
    (sphere_idx, t_max)
}

fn ray_color(ray: &Ray, spheres: &Vec<Sphere>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    let (sphere_idx, t) = hit_spheres(ray, spheres);

    if t == f64::MAX {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color1 = Vec3::one();
        let color2 = Vec3::new(0.5, 0.7, 1.0);
        return color1 + t * (color2 - color1);
    }

    let ref sphere = spheres[sphere_idx];
    let incidence = ray.at(t);
    let outward_normal = (incidence - sphere.center) / sphere.radius;
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = match front_face {
        true => outward_normal,
        false => -outward_normal,
    };

    let scatter_color = sphere.material.scatter(&ray.direction, &normal, front_face);
    if scatter_color.is_none() {
        return Vec3::zero();
    }

    let (scatter, color) = scatter_color.unwrap();
    let scatter_ray = Ray::new(incidence, scatter);
    color * ray_color(&scatter_ray, spheres, depth - 1)
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
    let something = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            if (center - something).length() > 0.9 {
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
    let img_h = (img_w as f64 / aspect_ratio) as u32;
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

    let mut tiff_file = TiffFile::new(&"sample.tiff", img_w, img_h);
    let now = Instant::now();

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..img_w {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let (offset_x, offset_y): (f64, f64) = random();
                let u = (i as f64 + offset_x) / img_w as f64;
                let v = (j as f64 + offset_y) / img_h as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &spheres, max_depth);
            }
            pixel_color /= samples_per_pixel as f64;
            tiff_file.write_image_data(&pixel_color);
        }
    }

    eprintln!("\nDone.");
    let elapsed_time = now.elapsed();
    eprintln!("{} seconds.", elapsed_time.as_secs());
}
