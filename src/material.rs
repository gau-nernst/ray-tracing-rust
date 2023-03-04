use crate::rand::RandomGenerator;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

impl Material {
    pub fn scatter(
        &self,
        incident: Vec3,
        incidence: Vec3,
        normal: Vec3,
        random: &mut RandomGenerator,
    ) -> (Ray, Vec3) {
        match self {
            Material::Lambertian(color) => {
                let diffuse = normal + Vec3::random_unit_sphere(random).normalize();
                (Ray::new(incidence, diffuse), color.to_owned())
            }
            Material::Metal(color) => {
                let reflected = incident.reflect(normal);
                (Ray::new(incidence, reflected), color.to_owned())
            }
        }
    }
}
