use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn float_max(a: f32, b: f32) -> f32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

pub fn float_min(a: f32, b: f32) -> {
    if a >= b {
        return a;
    } else {
        return b;
    }
}