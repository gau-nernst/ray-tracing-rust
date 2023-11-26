use crate::hittable::{Hittable, HittableList, Ray};
use crate::pcg32::PCG32;
use crate::vec3::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // NOTE: camera pointing in negative z direction
        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - focus_distance * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }
    fn get_ray(&self, s: f32, t: f32, rng: &mut PCG32) -> Ray {
        let rd = self.lens_radius * Vec3::random_unit_disk(rng);
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

pub struct Renderer {
    pub img_width: u32,
    pub img_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}
impl Renderer {
    pub fn new(img_width: u32, aspect_ratio: f32, samples_per_pixel: u32, max_depth: u32) -> Renderer {
        Renderer {
            img_width,
            img_height: (img_width as f32 / aspect_ratio) as u32,
            samples_per_pixel,
            max_depth,
        }
    }
    pub fn render(&self, objects: &HittableList, camera: &Camera, buffer: &mut [u8]) {
        for j in 0..self.img_height {
            eprint!("Line {j}\r");
            for i in 0..self.img_width {
                let mut rng = PCG32::new(17 + j as u64, 23 + i as u64);

                let mut pixel_color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + rng.f32()) / self.img_width as f32;
                    let v = ((self.img_height - 1 - j) as f32 + rng.f32()) / self.img_height as f32;

                    let r = camera.get_ray(u, v, &mut rng);
                    pixel_color = pixel_color + Renderer::ray_color(&r, objects, self.max_depth, &mut rng);
                }
                pixel_color = pixel_color / self.samples_per_pixel as f32;

                let offset = ((j * self.img_width + i) * 3) as usize;
                buffer[offset] = (pixel_color.0.sqrt().clamp(0.0, 1.0) * 255.0) as u8;
                buffer[offset + 1] = (pixel_color.1.sqrt().clamp(0.0, 1.0) * 255.0) as u8;
                buffer[offset + 2] = (pixel_color.2.sqrt().clamp(0.0, 1.0) * 255.0) as u8;
            }
        }
    }
    fn ray_color(ray: &Ray, objects: &HittableList, depth: u32, rng: &mut PCG32) -> Vec3 {
        if depth == 0 {
            return Vec3::zero();
        }

        match objects.hit(ray, 0.001, f32::INFINITY) {
            None => {
                // background
                let unit_direction = ray.direction.normalize();
                let t = 0.5 * (unit_direction.1 + 1.0);
                let color1 = Vec3::one();
                let color2 = Vec3(0.5, 0.7, 1.0);
                color1 + t * (color2 - color1)
            }
            Some(rec) => match rec.material.scatter(&ray.direction, &rec, rng) {
                None => Vec3::zero(),
                Some((scatter, color)) => {
                    let scatter_ray = Ray::new(rec.p, scatter);
                    color * Renderer::ray_color(&scatter_ray, objects, depth - 1, rng)
                }
            },
        }
    }
}
