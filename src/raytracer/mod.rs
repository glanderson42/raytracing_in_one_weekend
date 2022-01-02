mod vec3;
mod ray;
mod hit_record;
mod hit_table_list;
mod sphere;
mod hitable;
mod camera;

pub type Vec3 = vec3::Vec3;
pub type Ray = ray::Ray;
pub type HitRecord = hit_record::HitRecord;
pub type HitTableList = hit_table_list::HitTableList;
pub type Sphere = sphere::Sphere;
pub type Hitable = dyn hitable::Hitable;
pub type Camera = camera::Camera;