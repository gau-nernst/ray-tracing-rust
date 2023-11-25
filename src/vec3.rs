use crate::pcg32;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn one() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }
    pub fn length(self) -> f32 {
        self.length2().sqrt()
    }
    pub fn length2(self) -> f32 {
        self.dot(self)
    }
    pub fn dot(self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - other.1 * self.2,
            self.2 * other.0 - other.2 * self.0,
            self.0 * other.1 - other.0 * self.1,
        )
    }
    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    pub fn refract(self, n: Vec3, eta: f32) -> Vec3 {
        let cos_theta = f32::min(-self.dot(n), 1.0);
        let r_out_perp = eta * (self + cos_theta * n);
        let r_out_para = -(1.0 - r_out_perp.length2()).abs().sqrt() * n;
        r_out_perp + r_out_para
    }
    pub fn rand(rng: &mut pcg32::PCG32State) -> Vec3 {
        Vec3(rng.f32(), rng.f32(), rng.f32())
    }
    pub fn rand_between(rng: &mut pcg32::PCG32State, lo: f32, hi: f32) -> Vec3 {
        Vec3(
            rng.f32_between(lo, hi),
            rng.f32_between(lo, hi),
            rng.f32_between(lo, hi),
        )
    }
    pub fn random_unit_sphere(rng: &mut pcg32::PCG32State) -> Vec3 {
        loop {
            let p = Vec3::rand_between(rng, -1.0, 1.0);
            if p.length2() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_disk(rng: &mut pcg32::PCG32State) -> Vec3 {
        loop {
            let p = Vec3(rng.f32(), rng.f32(), 0.0);
            if p.length2() < 1.0 {
                return p;
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}
impl Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self + rhs.0, self + rhs.1, self + rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}
impl Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self - rhs.0, self - rhs.1, self - rhs.2)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}
