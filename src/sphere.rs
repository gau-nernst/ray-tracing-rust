use crate::material::Material;
use crate::ray::Ray;
use crate::utils::new_struct;
use crate::vec3::Vec3;

new_struct!(Sphere { center: Vec3, radius: f64, material: Box<dyn Material> });

impl Sphere {
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

    pub fn hit_spheres(ray: &Ray, spheres: &Vec<Sphere>) -> (usize, f64) {
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
}
