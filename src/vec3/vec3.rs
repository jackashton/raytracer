use crate::utils::clamp;
use num_traits::{AsPrimitive, Float, Num, Signed};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<N: Num> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N: Num + Copy> Vec3<N> {
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: N::zero(),
            y: N::zero(),
            z: N::zero(),
        }
    }

    pub fn dot(&self, other: &Vec3<N>) -> N {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Vec3<N>) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }
}

impl<N: Num + Copy + PartialOrd> Vec3<N> {
    pub fn clamp(&mut self, min: N, max: N) -> Self {
        self.x = clamp(self.x, min, max);
        self.y = clamp(self.y, min, max);
        self.z = clamp(self.z, min, max);
        *self
    }
}

impl<N: Float> Vec3<N> {
    pub fn length(&self) -> N {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn sqrt(&mut self) -> Self {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
        *self
    }

    pub fn cbrt(&mut self) -> Self {
        self.x = self.x.cbrt();
        self.y = self.y.cbrt();
        self.z = self.z.cbrt();
        *self
    }

    pub fn powf(&mut self, n: N) -> Self {
        self.x = self.x.powf(n);
        self.y = self.y.powf(n);
        self.z = self.z.powf(n);
        *self
    }
}

impl Vec3<f64> {
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl<N: Num + Copy + Signed> Neg for Vec3<N> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<N: Num> Add for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<N: Num + Copy> Add<N> for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: N) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<N: Num + Copy> AddAssign for Vec3<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<N: Num> Sub for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<N: Num + Copy> Sub<N> for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: N) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<N: Num> Mul<Vec3<N>> for Vec3<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<N: Num + Copy> Mul<N> for Vec3<N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<N: Num + Copy> MulAssign<Vec3<N>> for Vec3<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<N: Num + Copy> MulAssign<N> for Vec3<N> {
    fn mul_assign(&mut self, rhs: N) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<N: Num> Div<Vec3<N>> for Vec3<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<N: Num + Copy> Div<N> for Vec3<N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<N: Num> PartialEq for Vec3<N> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<N: Num> From<Vec3<N>> for [N; 3] {
    fn from(v: Vec3<N>) -> [N; 3] {
        let Vec3 { x, y, z } = v;
        [x, y, z]
    }
}

impl<T: Num + Copy + 'static> Vec3<T> {
    pub fn from<U: AsPrimitive<T> + Num>(other: Vec3<U>) -> Vec3<T> {
        Vec3 {
            x: other.x.as_(),
            y: other.y.as_(),
            z: other.z.as_(),
        }
    }
}

pub type Color = Vec3<u8>;

pub type Point3<N> = Vec3<N>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.7);
        let vec2: Vec3<f64> = Vec3::new(0.1, 0.3, 0.3);
        let res: Vec3<f64> = Vec3::new(0.30000000000000004, 0.7, 1.0);
        assert_eq!(vec1 + vec2, res);

        let vec1: Vec3<u8> = Vec3::new(1, 2, 3);
        let vec2: Vec3<u8> = Vec3::new(3, 2, 1);
        let res: Vec3<u8> = Vec3::new(4, 4, 4);
        assert_eq!(vec1 + vec2, res);

        let res: Vec3<u8> = Vec3::new(2, 3, 4);
        assert_eq!(vec1 + 1, res)
    }

    #[test]
    fn test_sub() {
        let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.8);
        let vec2: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
        let res: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
        assert_eq!(vec1 - vec2, res);

        let vec1: Vec3<u8> = Vec3::new(5, 6, 2);
        let vec2: Vec3<u8> = Vec3::new(3, 2, 1);
        let res: Vec3<u8> = Vec3::new(2, 4, 1);
        assert_eq!(vec1 - vec2, res);

        let res: Vec3<u8> = Vec3::new(4, 5, 1);
        assert_eq!(vec1 - 1, res)
    }

    #[test]
    fn test_mul() {
        let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let vec2: Vec3<f64> = Vec3::new(2.0, 2.0, 2.0);
        let res: Vec3<f64> = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(vec1 * vec2, res);
        assert_eq!(vec1 * 2.0, res);
    }

    #[test]
    fn test_div() {
        let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 4.0);
        let vec2: Vec3<f64> = Vec3::new(2.0, 2.0, 2.0);
        let res: Vec3<f64> = Vec3::new(0.5, 1.0, 2.0);
        assert_eq!(vec1 / vec2, res);
        assert_eq!(vec1 / 2.0, res);
    }

    #[test]
    fn test_length() {
        let vec1: Vec3<f64> = Vec3::new(0.0, 3.0, 4.0);
        let res: f64 = 5.0;
        assert_eq!(vec1.length(), res);
    }

    #[test]
    fn test_unit_vector() {
        let vec1: Vec3<f64> = Vec3::new(0.0, 3.0, 4.0);
        let res: Vec3<f64> = Vec3::new(0.0, 3.0 / 5.0, 4.0 / 5.0);
        assert_eq!(vec1.normalize(), res);
    }

    #[test]
    fn test_dot() {
        let vec1: Vec3<f64> = Vec3::new(1.0, 3.0, 4.0);
        let vec2: Vec3<f64> = Vec3::new(2.0, 3.0, 5.0);
        let res: f64 = 31.0;
        assert_eq!(vec1.dot(&vec2), res);
    }

    #[test]
    fn test_cross() {
        let vec1: Vec3<i32> = Vec3::new(1, 3, 4);
        let vec2: Vec3<i32> = Vec3::new(2, 3, 5);
        let res: Vec3<i32> = Vec3::new(3, 3, -3);
        assert_eq!(vec1.cross(&vec2), res);
    }

    #[test]
    fn test_neg() {
        let vec = Vec3::new(1, 3, 4);
        assert_eq!(-vec, (vec * -1));
    }
}
