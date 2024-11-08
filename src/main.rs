use image::codecs::png::PngEncoder;
use crate::camera::Camera;
use crate::vector::Vector3;
use crate::world::ray::Ray;
use crate::world::sphere::Sphere;
use crate::world::World;

mod world;
mod vector;
mod camera;

fn main() {
    let mut world = World::new();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, 100.0), 20.0));

    let camera_base = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    let camera = Camera::new(camera_base, 640, 480, 50.0);

    let image = camera.take_photo(&world);
    
    image.save("output.png").unwrap();
}
