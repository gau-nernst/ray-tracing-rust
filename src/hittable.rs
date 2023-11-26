use std::cmp::Ordering;
use std::mem;
use std::ops::Index;
use std::rc::Rc;

use crate::material::Material;
use crate::pcg32::PCG32;
use crate::vec3::Vec3;

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
pub struct AABB {
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for axis in 0..3 {
            let inv_d = 1.0 / ray.direction[axis];
            let origin = ray.origin[axis];

            let mut t0 = (self[axis].0 - origin) * inv_d;
            let mut t1 = (self[axis].1 - origin) * inv_d;

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
impl Index<usize> for AABB {
    type Output = (f32, f32);
    fn index(&self, index: usize) -> &(f32, f32) {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bbox(&self) -> AABB;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new(), bbox: AABB::empty() }
    }
    pub fn push(&mut self, obj: Rc<dyn Hittable>) {
        self.bbox = AABB::from_aabb(self.bbox, obj.bbox());
        self.objects.push(obj);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = AABB::empty();
    }
}
impl Hittable for HittableList {
    fn bbox(&self) -> AABB {
        self.bbox
    }
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
    fn bbox(&self) -> AABB {
        self.bbox
    }
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

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}
impl BVHNode {
    fn compare_bbox(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
        let ax = a.bbox()[axis].0;
        let bx = b.bbox()[axis].0;
        if ax < bx {
            Ordering::Less
        } else if ax > bx {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    fn compare_bbox_x(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BVHNode::compare_bbox(a, b, 0)
    }
    fn compare_bbox_y(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BVHNode::compare_bbox(a, b, 1)
    }
    fn compare_bbox_z(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BVHNode::compare_bbox(a, b, 2)
    }
    pub fn new(objects: &[Rc<dyn Hittable>], rng: &mut PCG32) -> BVHNode {
        let mut objects = objects.to_vec();

        let left;
        let right;

        let compare_fn = match rng.u32_between(0, 3) {
            0 => BVHNode::compare_bbox_x,
            1 => BVHNode::compare_bbox_y,
            2 => BVHNode::compare_bbox_z,
            _ => panic!("This should not happen"),
        };

        match objects.len() {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => match compare_fn(&objects[0], &objects[1]) {
                Ordering::Less => {
                    left = objects[0].clone();
                    right = objects[1].clone();
                }
                _ => {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            },
            _ => {
                objects.sort_by(compare_fn);
                let mid = objects.len() / 2;
                left = Rc::new(BVHNode::new(&objects[..mid], rng));
                right = Rc::new(BVHNode::new(&objects[mid..], rng));
            }
        }

        let bbox = AABB::from_aabb(left.bbox(), right.bbox());
        BVHNode { left, right, bbox }
    }
}
impl Hittable for BVHNode {
    fn bbox(&self) -> AABB {
        self.bbox
    }
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }
        match self.left.hit(ray, t_min, t_max) {
            None => self.right.hit(ray, t_min, t_max),
            Some(left_rec) => match self.right.hit(ray, t_min, left_rec.t) {
                None => Some(left_rec),
                Some(right_rec) => Some(right_rec),
            },
        }
    }
}
