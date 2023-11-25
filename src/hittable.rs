use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        let oc = ray.origin - self.center;
        let a = ray.direction.length2();
        let half_b = oc.dot(ray.direction);
        let c = oc.length2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return f32::MAX;
        }

        let disc_sqrt = discriminant.sqrt();
        let root = (-half_b - disc_sqrt) / a;
        if t_min < root && root < t_max {
            return root;
        }
        let root = (-half_b + disc_sqrt) / a;
        if t_min < root && root < t_max {
            return root;
        }

        f32::MAX
    }
    pub fn hit_spheres(ray: &Ray, spheres: &[Sphere]) -> (usize, f32) {
        let mut sphere_idx = 0;
        let mut t_max = f32::MAX;

        for (idx, sphere) in spheres.iter().enumerate() {
            let t = sphere.hit(ray, 0.0001, t_max);
            if t < t_max {
                sphere_idx = idx;
                t_max = t;
            }
        }
        (sphere_idx, t_max)
    }
}
