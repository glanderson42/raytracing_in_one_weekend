extern crate rand;
extern crate termion;

mod raytracer;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::f32;
use std::boxed::Box;
use pbr::ProgressBar;

use rand::Rng;

use raytracer::{
    Camera,
    HitRecord,
    HitTableList,
    Hitable,
    Ray,
    Sphere,
    Vec3
};

fn ray_color(r: &mut Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord::origin();
    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    } else {
        let unit_direction = r.direction.to_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

#[allow(dead_code)]
enum Resulotion {
    FHD,
    UHD,
    HD,
}

impl Resulotion {
    pub fn get_resolution(res: Resulotion) -> (f32, f32) {
        match res {
            Self::FHD => (1920.0, 1080.0),
            Self::UHD => (3840.0, 2160.0),
            Self::HD => (1280.0, 720.0)
        }
    }
}

fn main() -> std::io::Result<()> {
    let (image_width, image_height) = Resulotion::get_resolution(Resulotion::UHD);
    let samples_per_pixel: i32 = 100;
    
    let start_time = Instant::now();

    let mut file = File::create("raytacing.ppm")?;
    let header = "P3\n".to_owned() + &image_width.to_string() + " " + &image_height.to_string() + "\n255\n";
    file.write_all(header.as_bytes())?;

    let mut world = HitTableList::new();
    world.new_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.new_object(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();
    let mut rng = rand::thread_rng();
    let mut pb = ProgressBar::new(image_height as u64);
    pb.tick_format("\\|/-");
    pb.format("|#--|");
    pb.show_tick = true;

    for i in (0..image_height as i32).rev() {
        pb.inc();

        let mut line = Vec3::origin();
        for j in 0..image_width as i32 {
            pb.tick();
            for _ in 0..samples_per_pixel as i32 {
                let u = (j as f32 + rng.gen_range(0.0..1.0)) / image_width;
                let v = (i as f32 + rng.gen_range(0.0..1.0)) / image_height;
                let mut r = camera.get_ray(u, v);
                line += ray_color(&mut r, &mut world);
            }
            let _ = line.save_ppm(&mut file, samples_per_pixel);
        }
    }

    drop(world.objects);
    pb.finish_print("Done!");
    let duration = start_time.elapsed();
    println!("Took: {:?}", duration);

    Ok(())
}