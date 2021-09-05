pub mod geometry;
pub mod scene;
pub mod utils;
pub mod signals;

mod acoustic_raytrace;
pub use crate::acoustic_raytrace::AcousticRaytracer;

use nalgebra::{Point3, Transform3, Vector3};

pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;
pub type Transform = Transform3<f32>;
