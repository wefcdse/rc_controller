#![allow(unused)]
use glam as _;
mod f32 {
    pub type Vec3 = glam::Vec3;
    pub type Float = f32;
    pub type Vec4 = glam::Vec4;
    pub type Quat = glam::Quat;
    pub const PI: Float = std::f32::consts::PI;
}
mod f64 {
    pub type Vec3 = glam::DVec3;
    pub type Float = f64;
    pub type Vec4 = glam::DVec4;
    pub type Quat = glam::DQuat;
    pub const PI: Float = std::f64::consts::PI;
}

pub use self::f64::*;
