use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.length2();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return f64::MAX;
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

        return f64::MAX;
    }
}
