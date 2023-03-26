use crate::utils::new_struct;
use crate::vec3::Vec3;

new_struct!(Ray {
    origin: Vec3,
    direction: Vec3
});

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
