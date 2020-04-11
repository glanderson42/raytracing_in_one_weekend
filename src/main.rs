extern crate rand;
extern crate termion;

mod raytracer;

use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};
use std::f32;
use std::boxed::Box;

use rand::Rng;

use raytracer::*;

fn ray_color(r: &mut ray::Ray, world: &dyn hitable::Hitable) -> vec3::Vec3 {
    let mut rec = hit_record::HitRecord::origin();
    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + vec3::Vec3::new(1.0, 1.0, 1.0));
    } else {
        let unit_direction = r.direction.to_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * vec3::Vec3::new(1.0, 1.0, 1.0) + t * vec3::Vec3::new(0.5, 0.7, 1.0);
    }
}

fn print_process(process: f32) {
    let num_of_sign = ((100.0 / 2.0) * process / 100.0) as i32;
    let num_of_spaces = 50 - num_of_sign;
    let mut bar = String::new();
    for _ in 0..num_of_sign {
        bar.push('=');
    }
    bar.push('>');
    for _ in 0..num_of_spaces {
        bar.push(' ');
    }

    print!("{}", termion::clear::All);
    if num_of_sign != 50 {
        println!("[{}] {}%", bar, process);
    } else {
        bar = bar.replace(">", "");
        println!("[{}] {}%", bar, process);
    }
}

fn main() -> std::io::Result<()> {
    let image_width: f32 = 1920.0;
    let image_height: f32 = 1080.0;
    let samples_per_pixel: i32 = 100;
    
    let start_time = Instant::now();

    let mut file = File::create("raytacing.ppm")?;
    let header = "P3\n".to_owned() + &image_width.to_string() + " " + &image_height.to_string() + "\n255\n";
    file.write_all(header.as_bytes())?;

    let mut world = hit_table_list::HitTableList::new();
    world.new_object(Box::new(sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.new_object(Box::new(sphere::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = camera::Camera::new();
    let mut rng = rand::thread_rng();

    for i in (0..image_height as i32).rev() {
        // println!("Processing: {}%", 100.0 - ((i as f32 / image_height) * 100.0));
        print_process(100.0 - ((i as f32 / image_height) * 100.0));
        let mut color = vec3::Vec3::origin();
        for j in 0..image_width as i32 {
            for _ in 0..samples_per_pixel as i32 {
                let u = (j as f32 + rng.gen_range(0.0, 1.0)) / image_width;
                let v = (i as f32 + rng.gen_range(0.0, 1.0)) / image_height;
                let mut r = camera.get_ray(u, v);
                color += ray_color(&mut r, &mut world);
            }

            let _ = color.write_to_file(&mut file, samples_per_pixel);
        }
    }
    println!("Done!");
    let duration = start_time.elapsed();
    println!("Took: {:?}", duration);

    Ok(())
}