use crate::vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct Solid {
    color: Vec3,
}
impl Solid {
    pub fn new(r: f32, g: f32, b: f32) -> Solid {
        Solid { color: Vec3(r, g, b) }
    }
}
impl Texture for Solid {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}
