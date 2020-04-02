use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;

use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
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

    pub fn write_to_file(&self, file: &mut File) -> Result<(), ()> {
        let _x = (255.999 * self.x) as i32;
        let _y = (255.999 * self.y) as i32;
        let _z = (255.999 * self.z) as i32;
        let line = _x.to_string() + " " + &_y.to_string() + " " + &_z.to_string() + "\n";
        let res = file.write_all(line.as_bytes());
        match res {
            Ok(_) => Ok(()),
            Err(e) => panic!("Error occured: {:?}", e)
        }
    }

    fn length_squared(&self) -> f32 {
        self.x.sqrt() + self.y.sqrt() + self.z.sqrt()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
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

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}