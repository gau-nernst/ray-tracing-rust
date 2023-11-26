use std::rc::Rc;

use crate::material::Material;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, material: Rc<dyn Material>, t: f32, front_face: bool) -> HitRecord {
        HitRecord {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }
    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_max = t_max;
        let mut rec = None;

        for obj in self.objects.iter() {
            match obj.hit(ray, t_min, t_max) {
                None => (),
                Some(tmp_rec) => {
                    t_max = tmp_rec.t;
                    rec = Some(tmp_rec);
                }
            }
        }

        rec
    }
}

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
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length2();
        let half_b = oc.dot(ray.direction);
        let c = oc.length2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let disc_sqrt = discriminant.sqrt();
        let mut root = (-half_b - disc_sqrt) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + disc_sqrt) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        Some(HitRecord::new(
            p,
            match front_face {
                true => outward_normal,
                false => -outward_normal,
            },
            self.material.clone(),
            root,
            front_face,
        ))
    }
}
