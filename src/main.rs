use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};

mod raytracer;

use raytracer::vec3;
use raytracer::ray;

fn main() -> std::io::Result<()> {
    let image_width: f32 = 1920.0;
    let image_height: f32 = 1080.0;
    
    let start_time = Instant::now();

    let mut file = File::create("raytacing.ppm")?;
    let header = "P3\n".to_owned() + &image_width.to_string() + " " + &image_height.to_string() + "\n255\n";
    file.write_all(header.as_bytes())?;

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin =  vec3::Vec3::origin();

    for i in (0..image_height as i32).rev() {
        println!("Processing: {}%", 100.0 - ((i as f32 / image_height) * 100.0));
        for j in 0..image_width as i32 {
            let u = j as f32 / image_width;
            let v = i as f32 / image_height;
            let mut r = ray::Ray{origin: origin, direction: lower_left_corner + u * horizontal + v * vertical};
            let color = ray::ray_color(&mut r);
            let _ = color.write_to_file(&mut file);
        }
    }
    println!("Done!");
    let duration = start_time.elapsed();
    println!("Took: {:?}", duration);

    Ok(())
}