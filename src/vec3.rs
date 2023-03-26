use crate::random;
use crate::utils::new_struct;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

new_struct!(Vec3 { x: f64, y: f64, z: f64 } derive(Debug, Clone, Copy));

impl Vec3 {
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
        Vec3::new(random::rand(), random::rand(), random::rand())
    }
    pub fn rand_between(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random::rand_between(min, max),
            random::rand_between(min, max),
            random::rand_between(min, max),
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
            let p = Vec3::new(random::rand(), random::rand(), 0.0);
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

macro_rules! expand {
    ($token:tt, $attr:ident, "vec3") => {
        $token.$attr
    };
    ($token:tt, $attr:ident, "f64") => {
        $token
    };
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident) => {
        impl_binary_op!(Vec3, $trait, $method);
        impl_binary_op!(&Vec3, $trait, $method);
    };

    ($type:ty, $trait:ident, $method:ident) => {
        impl_binary_op!($type, "vec3", Vec3, "vec3", $trait, $method);
        impl_binary_op!($type, "vec3", &Vec3, "vec3", $trait, $method);

        impl_binary_op!($type, "vec3", f64, "f64", $trait, $method);
        impl_binary_op!($type, "vec3", &f64, "f64", $trait, $method);

        impl_binary_op!(f64, "f64", $type, "vec3", $trait, $method);
        impl_binary_op!(&f64, "f64", $type, "vec3", $trait, $method);
    };

    ($type1:ty, $expand1:tt, $type2:ty, $expand2:tt, $trait:ident, $method:ident) => {
        impl $trait<$type2> for $type1 {
            type Output = Vec3;
            fn $method(self, rhs: $type2) -> Vec3 {
                Vec3::new(
                    expand!(self, x, $expand1).$method(expand!(rhs, x, $expand2)),
                    expand!(self, y, $expand1).$method(expand!(rhs, y, $expand2)),
                    expand!(self, z, $expand1).$method(expand!(rhs, z, $expand2)),
                )
            }
        }
    };
}

impl_binary_op!(Add, add);
impl_binary_op!(Sub, sub);
impl_binary_op!(Mul, mul);
impl_binary_op!(Div, div);

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident) => {
        impl_assign_op!(Vec3, "vec3", $trait, $method);
        impl_assign_op!(&Vec3, "vec3", $trait, $method);

        impl_assign_op!(f64, "f64", $trait, $method);
        impl_assign_op!(&f64, "f64", $trait, $method);
    };

    ($type:ty, $expand:tt, $trait:ident, $method:ident) => {
        impl $trait<$type> for Vec3 {
            fn $method(&mut self, rhs: $type) {
                self.x.$method(expand!(rhs, x, $expand));
                self.y.$method(expand!(rhs, y, $expand));
                self.z.$method(expand!(rhs, z, $expand));
            }
        }
    };
}

impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(SubAssign, sub_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(DivAssign, div_assign);
