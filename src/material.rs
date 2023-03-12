use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(
        &self,
        incident: &Vec3,
        normal: &Vec3,
        front_face: bool,
    ) -> Option<(Vec3, Vec3)> {
        match self {
            Material::Lambertian(albedo) => {
                let mut diffuse = normal + Vec3::random_unit_sphere().normalize();
                // catch degenerate scatter direction
                if diffuse.length2() < 1e-16 {
                    diffuse = *normal;
                }
                Some((diffuse, *albedo))
            }

            Material::Metal(albedo, fuzz) => {
                // only reflect when incident is opposite normal
                if incident.dot(normal) >= 0.0 {
                    return None;
                }
                let reflected = incident.reflect(normal) + Vec3::random_unit_sphere() * fuzz;
                Some((reflected, *albedo))
            }

            Material::Dielectric(eta) => {
                let eta = match front_face {
                    true => 1.0 / eta,
                    false => *eta,
                };
                let incident_norm = incident.normalize();
                let cos_theta = 1f64.min(-incident_norm.dot(normal));
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let refracted = match (eta * sin_theta <= 1.0)
                    && (schlick_reflectance(cos_theta, eta) < random())
                {
                    true => incident_norm.refract(normal, eta),
                    false => incident_norm.reflect(normal), // total internal reflection
                };
                Some((refracted, Vec3::zero()))
            }
        }
    }
}

fn schlick_reflectance(cosine: f64, eta: f64) -> f64 {
    let r0 = (1.0 - eta) / (1.0 + eta);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
