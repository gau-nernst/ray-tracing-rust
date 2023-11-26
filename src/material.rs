use crate::hittable::HitRecord;
use crate::pcg32;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, incident: &Vec3, rec: &HitRecord, rng: &mut pcg32::PCG32State) -> Option<(Vec3, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _incident: &Vec3, rec: &HitRecord, rng: &mut pcg32::PCG32State) -> Option<(Vec3, Vec3)> {
        let mut diffuse = rec.normal + Vec3::random_unit_sphere(rng).normalize();
        // catch degenerate scatter direction
        if diffuse.length2() < 1e-16 {
            diffuse = rec.normal;
        }
        Some((diffuse, self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}
fn reflect(incident: Vec3, n: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(n) * n
}
impl Material for Metal {
    fn scatter(&self, incident: &Vec3, rec: &HitRecord, rng: &mut pcg32::PCG32State) -> Option<(Vec3, Vec3)> {
        // only reflect when incident is opposite normal
        if incident.dot(rec.normal) >= 0.0 {
            return None;
        }
        let reflected = reflect(*incident, rec.normal);
        Some((reflected + Vec3::random_unit_sphere(rng) * self.fuzz, self.albedo))
    }
}

pub struct Dielectric {
    eta: f32,
}
impl Dielectric {
    pub fn new(eta: f32) -> Dielectric {
        Dielectric { eta }
    }
}
fn refract(incident: Vec3, n: Vec3, eta: f32) -> Vec3 {
    let cos_theta = f32::min(-incident.dot(n), 1.0);
    let r_out_perp = eta * (incident + cos_theta * n);
    let r_out_para = -(1.0 - r_out_perp.length2()).abs().sqrt() * n;
    r_out_perp + r_out_para
}
fn schlick_reflectance(cos_theta: f32, eta: f32) -> f32 {
    let r0 = (1.0 - eta) / (1.0 + eta);
    let r02 = r0 * r0;
    r02 + (1.0 - r02) * (1.0 - cos_theta).powf(5.0)
}
impl Material for Dielectric {
    fn scatter(&self, incident: &Vec3, rec: &HitRecord, rng: &mut pcg32::PCG32State) -> Option<(Vec3, Vec3)> {
        let eta = match rec.front_face {
            true => 1.0 / self.eta,
            false => self.eta,
        };
        let incident_norm = incident.normalize();
        let cos_theta = (-incident_norm.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let refracted = match (eta * sin_theta <= 1.0) && (schlick_reflectance(cos_theta, eta) < rng.f32()) {
            true => refract(incident_norm, rec.normal, eta),
            false => reflect(incident_norm, rec.normal), // total internal reflection
        };
        Some((refracted, Vec3::zero()))
    }
}
