use crate::random;
use crate::utils::new_struct;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, incident: &Vec3, normal: &Vec3, front_face: bool) -> Option<(Vec3, Vec3)>;
}

new_struct!(Lambertian { albedo: Vec3 });

impl Material for Lambertian {
    fn scatter(&self, _incident: &Vec3, normal: &Vec3, _front_face: bool) -> Option<(Vec3, Vec3)> {
        let mut diffuse = *normal + Vec3::random_unit_sphere().normalize();
        // catch degenerate scatter direction
        if diffuse.length2() < 1e-16 {
            diffuse = *normal;
        }
        Some((diffuse, self.albedo))
    }
}

new_struct!(Metal {
    albedo: Vec3,
    fuzz: f32
});

impl Material for Metal {
    fn scatter(&self, incident: &Vec3, normal: &Vec3, _front_face: bool) -> Option<(Vec3, Vec3)> {
        // only reflect when incident is opposite normal
        if incident.dot(*normal) >= 0.0 {
            return None;
        }
        let reflected = incident.reflect(*normal) + Vec3::random_unit_sphere() * self.fuzz;
        Some((reflected, self.albedo))
    }
}

new_struct!(Dielectric { eta: f32 });

impl Dielectric {
    fn schlick_reflectance(cosine: f32, eta: f32) -> f32 {
        let r0 = (1.0 - eta) / (1.0 + eta);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident: &Vec3, normal: &Vec3, front_face: bool) -> Option<(Vec3, Vec3)> {
        let eta = match front_face {
            true => 1.0 / self.eta,
            false => self.eta,
        };
        let incident_norm = incident.normalize();
        let cos_theta = 1f32.min(-incident_norm.dot(*normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let refracted = match (eta * sin_theta <= 1.0)
            && (Dielectric::schlick_reflectance(cos_theta, eta) < random::randf32())
        {
            true => incident_norm.refract(*normal, eta),
            false => incident_norm.reflect(*normal), // total internal reflection
        };
        Some((refracted, Vec3::zero()))
    }
}
