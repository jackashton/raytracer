use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Num};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<N: Num> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N: Num> Vec3<N> where N: Copy {
    pub fn new(x: N, y: N, z: N) -> Vec3<N> {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> N {
        self.x
    }

    pub fn y(&self) -> N {
        self.y
    }

    pub fn z(&self) -> N {
        self.z
    }
}

impl<N: Num> Add for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<N: Num> Sub for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<N: Num> PartialEq for Vec3<N> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[test]
fn test_add() {
    let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.7);
    let vec2: Vec3<f64> = Vec3::new(0.1, 0.3, 0.3);
    let res: Vec3<f64> = Vec3::new(0.30000000000000004, 0.7, 1.0);
    assert_eq!(vec1 + vec2, res);
}

#[test]
fn test_sub() {
    let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.8);
    let vec2: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
    let res: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
    assert_eq!(vec1 - vec2, res);
}