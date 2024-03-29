use crate::Vec3;

use crate::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn origin() -> HitRecord {
        HitRecord { p: Vec3::origin(), normal: Vec3::origin(), t: 0.0, front_face: false }
    }
    
    #[allow(dead_code)]
    pub fn set_face_normal(&mut self, r: &mut Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction, *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}