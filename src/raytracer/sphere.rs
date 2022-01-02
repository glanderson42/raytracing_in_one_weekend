use crate::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn origin() -> Sphere {
        Sphere { center: Vec3::origin(), radius: 0.0 }
    }

    pub fn new(vec: Vec3, radius: f32) -> Sphere {
        Sphere { center: vec, radius }
    }
}