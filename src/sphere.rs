use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub incidence: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length2();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let disc_sqrt = discriminant.sqrt();
        let root = (-half_b - disc_sqrt) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + disc_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let incidence = ray.at(root);
        let outward_normal = (incidence - self.center) / self.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        return Some(HitRecord {
            t: root,
            incidence,
            normal,
            front_face,
        });
    }
}
