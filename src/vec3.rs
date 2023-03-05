use rand::prelude::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }
    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }
    pub fn length2(&self) -> f64 {
        self.dot(self)
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - other.y * self.z,
            self.z * other.x - other.z * self.x,
            self.x * other.y - other.x * self.y,
        )
    }
    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    pub fn refract(&self, n: &Vec3, eta: f64) -> Vec3 {
        let cos_theta = 1f64.min(-self.dot(n));
        let r_out_perp = eta * (self + cos_theta * n);
        let r_out_para = -(1.0 - r_out_perp.length2()).abs().sqrt() * n;
        r_out_perp + r_out_para
    }
    pub fn rand() -> Vec3 {
        let (x, y, z) = random();
        Vec3::new(x, y, z)
    }
    pub fn rand_between(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::rand_between(-1.0, 1.0);
            if p.length2() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_disk() -> Vec3 {
        loop {
            let (x, y) = random();
            let p = Vec3::new(x, y, 0.0);
            if p.length2() < 1.0 {
                return p;
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

macro_rules! impl_op_f64 {
    ($t:ty, $tr:ident, $method:ident) => {
        impl $tr<f64> for $t {
            type Output = Vec3;
            fn $method(self, rhs: f64) -> Vec3 {
                Vec3::new(
                    self.x.$method(rhs),
                    self.y.$method(rhs),
                    self.z.$method(rhs),
                )
            }
        }
        impl $tr<$t> for f64 {
            type Output = Vec3;
            fn $method(self, rhs: $t) -> Vec3 {
                Vec3::new(
                    self.$method(rhs.x),
                    self.$method(rhs.y),
                    self.$method(rhs.z),
                )
            }
        }
    };
}
macro_rules! impl_op {
    ($t:ty, $tr:ident, $method:ident) => {
        impl_op!($t, $t, $tr, $method);
        impl_op!($t, &$t, $tr, $method);
        impl_op!(&$t, $t, $tr, $method);
        impl_op!(&$t, &$t, $tr, $method);
        impl_op_f64!($t, $tr, $method);
        impl_op_f64!(&$t, $tr, $method);
    };
    ($t1:ty, $t2:ty, $tr:ident, $method:ident) => {
        impl $tr<$t2> for $t1 {
            type Output = Vec3;
            fn $method(self, rhs: $t2) -> Vec3 {
                Vec3::new(
                    self.x.$method(rhs.x),
                    self.y.$method(rhs.y),
                    self.z.$method(rhs.z),
                )
            }
        }
    };
}
impl_op!(Vec3, Add, add);
impl_op!(Vec3, Sub, sub);
impl_op!(Vec3, Mul, mul);
impl_op!(Vec3, Div, div);
