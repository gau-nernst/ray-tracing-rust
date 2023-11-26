use std::mem;
use std::rc::Rc;

use crate::material::Material;
use crate::vec3::{Axis, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[rustfmt::skip]
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray { Ray { origin, direction } }
    pub fn at(&self, t: f32) -> Vec3 { self.origin + self.direction * t }
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
        HitRecord { p, normal, material, t, front_face }
    }
}

#[derive(Debug, Clone, Copy)]
struct AABB {
    x: (f32, f32),
    y: (f32, f32),
    z: (f32, f32),
}
impl AABB {
    fn empty() -> AABB {
        AABB {
            x: (f32::INFINITY, -f32::INFINITY),
            y: (f32::INFINITY, -f32::INFINITY),
            z: (f32::INFINITY, -f32::INFINITY),
        }
    }
    fn from_vec3(a: Vec3, b: Vec3) -> AABB {
        AABB {
            x: (a.0.min(b.0), a.0.max(b.0)),
            y: (a.1.min(b.1), a.1.max(b.1)),
            z: (a.2.min(b.2), a.2.max(b.2)),
        }
    }
    fn from_aabb(a: AABB, b: AABB) -> AABB {
        AABB {
            x: (a.x.0.min(b.x.0), a.x.1.max(b.x.1)),
            y: (a.y.0.min(b.y.0), a.y.1.max(b.y.1)),
            z: (a.z.0.min(b.z.0), a.z.1.max(b.z.1)),
        }
    }
    fn axis(&self, axis: Axis) -> (f32, f32) {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let inv_d = 1.0 / ray.direction.axis(axis);
            let origin = ray.origin.axis(axis);

            let mut t0 = (self.axis(axis).0 - origin) * inv_d;
            let mut t1 = (self.axis(axis).1 - origin) * inv_d;

            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bbox(&self) -> AABB;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new(), bbox: AABB::empty() }
    }
    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.bbox = AABB::from_aabb(self.bbox, obj.bbox());
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
    fn bbox(&self) -> AABB {
        self.bbox
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
    bbox: AABB,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            bbox: AABB::from_vec3(center - radius, center + radius),
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
    fn bbox(&self) -> AABB {
        self.bbox
    }
}
