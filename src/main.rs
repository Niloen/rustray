use crate::camera::Camera;
use crate::vector::Vector3;
use crate::visualize::show;
use crate::world::ray::Ray;
use crate::world::sphere::Sphere;
use crate::world::{BaseMaterial, Light, World};
use image::Rgb;
use std::time::Instant;

mod world;
mod vector;
mod camera;
mod visualize;


fn generate_image(width: u32, height: u32, tx: impl Fn((u32, u32, Rgb<u8>)) + Send + Sync) {
    let world = create_world();

    let camera_base = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.1, 1.0).normalize());
    let camera = Camera::new(camera_base, width, height, 50.0);

    let photo_start_time = Instant::now();
    let image = camera.take_photo(&world, tx);
    let photo_duration = photo_start_time.elapsed();
    println!("Photo generation completed in: {:?}", photo_duration);
    image.save("output.png").unwrap();

    println!("Generated image");
}

fn create_world<'a>() -> World<'a> {
    create_world2()
}

fn create_world1<'a>() -> World<'a> {
    const MAT: BaseMaterial = BaseMaterial::DEFAULT;
    let mut world = World::new();
    world.add_light(Light::new(Ray::new(Vector3::new(0.0, 100.0, 100.0), Vector3::new(0.0, -1.0, 0.0)), Rgb([1.0, 1.0, 1.0])));
    world.add_light(Light::new(Ray::new(Vector3::new(-100.0, 0.0, 80.0), Vector3::new(0.0, -1.0, 0.0)), Rgb([0.0, 0.0, 1.0])));
    world.add(Sphere::new(Vector3::new(0.0, 25.0, 100.0), 20.0, Rgb([1.0, 1.0, 1.0]), &MAT));
    for i in 1..=1000 {
        let ifl = i as f64;

        world.add(Sphere::new(Vector3::new(20.0 + ifl, 0.5, 200.0 - ifl * 3.0), 50.0, Rgb([1.0, 1.0, 1.0]), &MAT));
    }
    world
}

fn create_world2<'a>() -> World<'a> {
    let mat: BaseMaterial = BaseMaterial::DEFAULT;
    let mirror: BaseMaterial = BaseMaterial {
        reflectivity: 0.7,
        ..mat
    };

    let white = Rgb([1.0, 1.0, 1.0]);
    let em: BaseMaterial = BaseMaterial {
        emission: white,
        ..mat
    };

    let mut world = World::new();
    world.add_light(Light::new(Ray::new(Vector3::new(0.0, 100.0, 100.0), Vector3::new(0.0, -1.0, 0.0)), white));
    world.add(Sphere::new(Vector3::new(0.0, 0.0, 100.0), 20.0, white, &mirror));
    world.add(Sphere::new(Vector3::new(200.0, 0.0, 100.0), 100.0, Rgb([1.0, 0.0, 0.0]), &mat));
    world
}

fn main() {
    let visualize = true;
    let width: u32 = 3820;
    let height: u32 = 1920;
    if visualize {
        show(width as i32, height as i32, move |tx| {
            generate_image(width, height, |m|tx.send_blocking(m).unwrap());
        })
    } else {
        generate_image(width, height, |_m| {})
    }
}