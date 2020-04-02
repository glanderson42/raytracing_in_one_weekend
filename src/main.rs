use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

mod raytracer;

fn main() -> std::io::Result<()> {
    let image_width: f32 = 1920.0;
    let image_height: f32 = 1080.0;
    
    let start_time = Instant::now();

    let mut file = File::create("raytacing.ppm")?;
    let header = "P3\n".to_owned() + &image_width.to_string() + " " + &image_height.to_string() + "\n255\n";
    file.write_all(header.as_bytes())?;
    for i in (0..image_height as i32).rev() {
        println!("Processing: {}%", 100.0 - ((i as f32 / image_height) * 100.0));
        for j in 0..image_width as i32 {
            let r = j as f32 / image_width;
            let g = i as f32 / image_height;
            let b = 0.2;
            let vec = raytracer::vec3::Vec3::new(r, g, b);
            let _ = vec.write_to_file(&mut file);
        }
    }
    println!("Done!");
    let duration = start_time.elapsed();
    println!("Took: {:?}", duration);

    Ok(())
}