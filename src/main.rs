use std::fs::File;
use std::io::prelude::*;

mod vec3;

fn main() -> std::io::Result<()> {
    let image_width: f32 = 200.0;
    let image_height: f32 = 100.0;
    let mut file = File::create("raytacing.ppm")?;
    let header = "P3\n".to_owned() + &image_width.to_string() + " " + &image_height.to_string() + "\n255\n";
    file.write_all(header.as_bytes())?;
    for i in (0..image_height as i32).rev() {
        println!("Scanline remaining: {}", i);
        for j in 0..image_width as i32 {
            let r = j as f32 / image_width;
            let g = i as f32 / image_height;
            let b = 0.2;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            let line = ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
            file.write_all(line.as_bytes())?;
        }
    }
    println!("Done!");

    let mut vec = vec3::Vec3::new(10.0, 20.0, 30.0);
    vec += vec3::Vec3::new(1.0, 1.0, 1.0);
    vec *= 3.0;
    println!("vec: {:?}", vec);
    Ok(())
}