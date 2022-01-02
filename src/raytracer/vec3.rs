use std::ops::{
    Add,
    Mul,
    Neg,
    Sub,
    Div,
    AddAssign,
    MulAssign,
    DivAssign,
    SubAssign,
};

use std::fs::File;
use std::io::prelude::*;

use image::RgbImage;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn compute(&mut self, scale: f32) -> (i32, i32, i32) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;

        let _x = (self.clamp(self.x, 0.0, 0.999) * 256.0) as i32;
        let _y = (self.clamp(self.y, 0.0, 0.999) * 256.0) as i32;
        let _z = (self.clamp(self.z, 0.0, 0.999) * 256.0) as i32;

        (_x, _y, _z)
    }

    pub fn save_ppm(&mut self, file: &mut File, samples_per_pixel: i32) -> Result<(), ()> {
        let _scale = 1.0 / samples_per_pixel as f32;
        
        let (_x, _z, _y) = self.compute(_scale);

        let line = _x.to_string() + " " + &_y.to_string() + " " + &_z.to_string() + "\n";
        let res = file.write_all(line.as_bytes());
        match res {
            Ok(_) => Ok(()),
            Err(e) => panic!("Error occured: {:?}", e)
        }
    }

    pub fn save_png(image: RgbImage) {
        
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn clamp(&self, x: f32, min: f32, max: f32) -> f32 {
        if x < min {
            return min;
        } else if x > max {
            return max;
        } else {
            return x;
        }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    
    #[allow(dead_code)]
    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: -(a.x * b.z - a.z * b.x),
            z: a.x * b.y - a.y * b.x
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { 
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[allow(dead_code)]
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}