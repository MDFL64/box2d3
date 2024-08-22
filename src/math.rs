// copied from wrapped2d

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[repr(C)]
pub struct AABB {
    pub lower_bound: Vec2,
    pub upper_bound: Vec2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rot {
    pub cos: f32,
    pub sin: f32,
}

impl Rot {
    pub fn from_angle(angle: f32) -> Rot {
        Rot {
            sin: angle.sin(),
            cos: angle.cos(),
        }
    }

    pub fn identity() -> Rot {
        Rot { sin: 0., cos: 1. }
    }

    pub fn x_axis(&self) -> Vec2 {
        Vec2 {
            x: self.cos,
            y: self.sin,
        }
    }

    pub fn y_axis(&self) -> Vec2 {
        Vec2 {
            x: -self.sin,
            y: self.cos,
        }
    }

    pub fn angle(&self) -> f32 {
        self.sin.atan2(self.cos)
    }
}

#[cfg(feature = "nalgebra")]
impl From<Rot> for nalgebra::Rotation2<f32> {
    fn from(r: Rot) -> nalgebra::Rotation2<f32> {
        nalgebra::Rotation2::new(nalgebra::Vector1::new(r.angle()))
    }
}

#[cfg(feature = "nalgebra")]
impl<'a> From<&'a nalgebra::Rotation2<f32>> for Rot {
    fn from(r: &'a nalgebra::Rotation2<f32>) -> Rot {
        use nalgebra::Rotation;
        Rot::from_angle(r.rotation().x)
    }
}

#[cfg(feature = "cgmath")]
impl From<Rot> for cgmath::Basis2<f32> {
    fn from(r: Rot) -> cgmath::Basis2<f32> {
        use cgmath::Rotation2;
        cgmath::Basis2::from_angle(cgmath::Rad(r.angle()))
    }
}

#[cfg(feature = "cgmath")]
impl<'a> From<&'a cgmath::Basis2<f32>> for Rot {
    fn from(r: &'a cgmath::Basis2<f32>) -> Rot {
        let col = r.as_ref().y;
        Rot {
            sin: col.x,
            cos: col.y,
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub rot: Rot,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            pos: Vec2 { x: 0., y: 0. },
            rot: Rot::identity(),
        }
    }
}

#[cfg(feature = "nalgebra")]
impl<'a> From<&'a Transform> for nalgebra::Isometry2<f32> {
    fn from(t: &'a Transform) -> nalgebra::Isometry2<f32> {
        nalgebra::Isometry2 {
            rotation: t.rot.into(),
            translation: t.pos.into(),
        }
    }
}

#[cfg(feature = "nalgebra")]
impl<'a> From<&'a nalgebra::Isometry2<f32>> for Transform {
    fn from(i: &'a nalgebra::Isometry2<f32>) -> Transform {
        Transform {
            pos: i.translation.into(),
            rot: (&i.rotation).into(),
        }
    }
}

impl<'a> Mul<Vec2> for &'a Transform {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        let x = (self.rot.cos * v.x - self.rot.sin * v.y) + self.pos.x;
        let y = (self.rot.sin * v.x + self.rot.cos * v.y) + self.pos.y;
        Vec2 { x: x, y: y }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Sweep {
    pub local_center: Vec2,
    pub c0: Vec2,
    pub c: Vec2,
    pub a0: f32,
    pub a: f32,
    pub alpha0: f32,
}
