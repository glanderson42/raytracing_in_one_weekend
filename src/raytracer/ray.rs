use crate::raytracer::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(r: &mut Ray) -> Vec3 {
    let unit_direction = r.direction.to_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0
    } + t * Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0
    }
}