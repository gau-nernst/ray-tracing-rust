mod camera;
mod hittable;
mod material;
mod pcg32;
mod tiff;
mod vec3;
use std::rc::Rc;

use std::time::Instant;

use camera::Camera;
use hittable::{Ray, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use pcg32::PCG32State;
use tiff::TiffFile;
use vec3::Vec3;

fn ray_color(ray: &Ray, spheres: &Vec<Sphere>, depth: i32, rng: &mut pcg32::PCG32State) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let (sphere_idx, t) = Sphere::hit_spheres(ray, spheres);

    if t == f32::MAX {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.1 + 1.0);
        let color1 = Vec3::one();
        let color2 = Vec3(0.5, 0.7, 1.0);
        return color1 + t * (color2 - color1);
    }

    let sphere = &spheres[sphere_idx];
    let incidence = ray.at(t);
    let mut normal = (incidence - sphere.center) / sphere.radius;
    let front_face = ray.direction.dot(normal) < 0.0;
    if !front_face {
        normal = -normal;
    }

    match sphere.material.scatter(&ray.direction, &normal, front_face, rng) {
        None => Vec3::zero(),
        Some((scatter, color)) => {
            let scatter_ray = Ray::new(incidence, scatter);
            color * ray_color(&scatter_ray, spheres, depth - 1, rng)
        }
    }
}

fn generate_spheres() -> Vec<Sphere> {
    let mut rng = PCG32State::new(19, 29);

    let mut spheres = vec![
        Sphere::new(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            Rc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))),
        ),
        Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5))),
        Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)))),
        Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0))),
    ];
    let something = Vec3(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(a as f32 + 0.9 * rng.f32(), 0.2, b as f32 + 0.9 * rng.f32());
            if (center - something).length() > 0.9 {
                let material: Rc<dyn Material>;
                let choose_mat = rng.f32();
                if choose_mat < 0.8 {
                    material = Rc::new(Lambertian::new(Vec3::rand(&mut rng) * Vec3::rand(&mut rng)));
                } else if choose_mat < 0.95 {
                    material = Rc::new(Metal::new(Vec3::rand_between(&mut rng, 0.5, 1.0), rng.f32() * 0.5));
                } else {
                    material = Rc::new(Dielectric::new(1.5));
                }
                spheres.push(Sphere::new(center, 0.2, material));
            }
        }
    }
    spheres
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 400;
    let img_height = (img_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let max_depth = 10;

    let camera = Camera::new(
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let spheres = generate_spheres();
    let mut buffer = vec![0_u8; (img_height * img_width * 3) as usize];

    let now = Instant::now();

    for j in 0..img_height {
        eprint!("Line {j}\r");
        for i in 0..img_width {
            let mut rng = PCG32State::new(17 + j as u64, 23 + i as u64);

            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.f32()) / img_width as f32;
                let v = ((img_height - 1 - j) as f32 + rng.f32()) / img_height as f32;

                let r = camera.get_ray(u, v, &mut rng);
                pixel_color = pixel_color + ray_color(&r, &spheres, max_depth, &mut rng);
            }
            pixel_color = pixel_color / samples_per_pixel as f32;

            let offset = ((j * img_width + i) * 3) as usize;
            buffer[offset] = (pixel_color.0.sqrt().clamp(0.0, 1.0) * 255.0) as u8;
            buffer[offset + 1] = (pixel_color.1.sqrt().clamp(0.0, 1.0) * 255.0) as u8;
            buffer[offset + 2] = (pixel_color.2.sqrt().clamp(0.0, 1.0) * 255.0) as u8;
        }
    }

    eprintln!("\nDone.");
    let elapsed_time = now.elapsed();
    eprintln!("{} seconds.", elapsed_time.as_secs());

    let mut tiff_file = TiffFile::new("sample.tiff", img_width, img_height);
    tiff_file.write(&buffer);
}
