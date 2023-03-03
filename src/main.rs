mod color;
mod ray;
mod vec3;

use color::write_color;
use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.length2();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length2() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return -half_b - discriminant.sqrt() / a;
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&center, 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - center).normalize();
        return 0.5 * (n + 1.0);
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

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    // NOTE: camera pointing in negative z direction
    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{img_w} {img_h}\n255");

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..img_w {
            let u = (i as f64 + 0.5) / img_w as f64;
            let v = (j as f64 + 0.5) / img_h as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }
    eprintln!("\nDone.")
}
