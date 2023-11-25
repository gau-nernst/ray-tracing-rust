use crate::pcg32;
use crate::utils::new_struct;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        incident: &Vec3,
        normal: &Vec3,
        front_face: bool,
        rng: &mut pcg32::PCG32State,
    ) -> Option<(Vec3, Vec3)>;
}

new_struct!(Lambertian { albedo: Vec3 });

impl Material for Lambertian {
    fn scatter(
        &self,
        _incident: &Vec3,
        normal: &Vec3,
        _front_face: bool,
        rng: &mut pcg32::PCG32State,
    ) -> Option<(Vec3, Vec3)> {
        let mut diffuse = *normal + Vec3::random_unit_sphere(rng).normalize();
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

fn reflect(incident: Vec3, n: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(n) * n
}
fn refract(incident: Vec3, n: Vec3, eta: f32) -> Vec3 {
    let cos_theta = f32::min(-incident.dot(n), 1.0);
    let r_out_perp = eta * (incident + cos_theta * n);
    let r_out_para = -(1.0 - r_out_perp.length2()).abs().sqrt() * n;
    r_out_perp + r_out_para
}

impl Material for Metal {
    fn scatter(
        &self,
        incident: &Vec3,
        normal: &Vec3,
        _front_face: bool,
        rng: &mut pcg32::PCG32State,
    ) -> Option<(Vec3, Vec3)> {
        // only reflect when incident is opposite normal
        if incident.dot(*normal) >= 0.0 {
            return None;
        }
        let reflected = reflect(*incident, *normal) + Vec3::random_unit_sphere(rng) * self.fuzz;
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
    fn scatter(
        &self,
        incident: &Vec3,
        normal: &Vec3,
        front_face: bool,
        rng: &mut pcg32::PCG32State,
    ) -> Option<(Vec3, Vec3)> {
        let eta = match front_face {
            true => 1.0 / self.eta,
            false => self.eta,
        };
        let incident_norm = incident.normalize();
        let cos_theta = 1f32.min(-incident_norm.dot(*normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let refracted = match (eta * sin_theta <= 1.0) && (Dielectric::schlick_reflectance(cos_theta, eta) < rng.f32())
        {
            true => refract(incident_norm, *normal, eta),
            false => reflect(incident_norm, *normal), // total internal reflection
        };
        Some((refracted, Vec3::zero()))
    }
}
