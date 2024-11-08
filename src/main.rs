use crate::camera::Camera;
use crate::vector::Vector3;
use crate::world::ray::Ray;
use crate::world::sphere::Sphere;
use crate::world::World;
use image::Rgb;

mod world;
mod vector;
mod camera;

fn main() {
    let mut world = World::new();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, 100.0), 20.0, Rgb([1.0, 0.0, 0.0])));
    world.add(Sphere::new(Vector3::new(20.0, 0.5, 200.0), 50.0, Rgb([0.0, 1.0, 0.0])));

    let camera_base = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.1, 1.0).normalize());
    let camera = Camera::new(camera_base, 3800, 1920, 50.0);

    let image = camera.take_photo(&world);
    
    image.save("output.png").unwrap();
}
