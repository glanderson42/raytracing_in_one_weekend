use crate::Vec3;

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

#[allow(dead_code)]
pub fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = Vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

#[allow(dead_code)]
pub fn ray_color(r: &mut Ray) -> Vec3 {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3{x: 0.0, y: 0.0, z: 1.0}).to_unit_vector();
        return 0.5 * Vec3{x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0};
    }
    let unit_direction = r.direction.to_unit_vector();
    t = 0.5 * (unit_direction.y + 1.0);
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