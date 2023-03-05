use crate::rand::RandomGenerator;
use crate::ray::Ray;
use crate::sphere::HitRecord;
use crate::vec3::Vec3;

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Vec3,
}

impl ScatterResult {
    fn new(ray: Ray, attenuation: Vec3) -> ScatterResult {
        ScatterResult { ray, attenuation }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
    None,
}

impl Material {
    pub fn scatter(
        &self,
        incident: Vec3,
        hit_record: &HitRecord,
        random: &mut RandomGenerator,
    ) -> Option<ScatterResult> {
        let scatter_color = match self {
            Material::Lambertian(color) => {
                let mut diffuse = hit_record.normal + Vec3::random_unit_sphere(random).normalize();
                // catch degenerate scatter direction
                if diffuse.length2() < 1e-16 {
                    diffuse = hit_record.normal;
                }
                Some((diffuse, color.to_owned()))
            }

            Material::Metal(color, fuzz) => {
                // only reflect when incident is opposite normal
                match incident.dot(hit_record.normal) < 0.0 {
                    true => {
                        let reflected = incident.reflect(hit_record.normal)
                            + Vec3::random_unit_sphere(random) * fuzz.to_owned();
                        Some((reflected, color.to_owned()))
                    }
                    false => None,
                }
            }

            Material::Dielectric(eta) => {
                let mut eta = eta.to_owned();
                if hit_record.front_face {
                    eta = 1.0 / eta;
                }
                let incident_norm = incident.normalize();
                let cos_theta = f64::min(-incident_norm.dot(hit_record.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let refracted = match (eta * sin_theta <= 1.0)
                    && (schlick_reflectance(cos_theta, eta) < random.rand())
                {
                    true => incident_norm.refract(hit_record.normal, eta),
                    false => incident_norm.reflect(hit_record.normal), // total internal reflection
                };

                Some((refracted, Vec3::one()))
            }

            Material::None => None,
        };

        match scatter_color {
            Some((scatter, color)) => Some(ScatterResult::new(
                Ray::new(hit_record.incidence, scatter),
                color,
            )),
            None => None,
        }
    }
}

fn schlick_reflectance(cosine: f64, eta: f64) -> f64 {
    let r0 = (1.0 - eta) / (1.0 + eta);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
