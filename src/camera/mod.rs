extern crate image;

use async_channel::Sender;
use crate::vector::Vector3;
use crate::world::ray::Ray;
use crate::world::World;
use image::{Rgb, RgbImage};

pub struct Camera {
    base: Ray,
    width: u32,
    height: u32,
    fov: f64
}

impl Camera {

    fn ray_at(&self, coord: (u32, u32)) -> Ray {
        let (x, y) = coord;
        let aspect_ratio = self.width as f64 / self.height as f64;
        let fov_radians = (self.fov.to_radians()) / 2.0;
        let scale = fov_radians.tan();

        // Convert pixel coordinates (x, y) to normalized device coordinates (NDC)
        let pixel_ndc_x = (x as f64 + 0.5) / self.width as f64;   // Add 0.5 to center the ray in the pixel
        let pixel_ndc_y = (y as f64 + 0.5) / self.height as f64;

        // Convert NDC to screen space [-1, 1]
        let pixel_screen_x = 2.0 * pixel_ndc_x - 1.0;
        let pixel_screen_y = 1.0 - 2.0 * pixel_ndc_y; // Flip the y-axis

        // Apply aspect ratio and scaling
        let pixel_camera_x = pixel_screen_x * aspect_ratio * scale;
        let pixel_camera_y = pixel_screen_y * scale;

        // Create the local direction vector in the camera space
        let local_direction = Vector3::new(pixel_camera_x, pixel_camera_y, 1.0).normalize(); // Assuming -Z is forward

        // Use the camera's base direction and up vector to create the world space direction
        let forward = self.base.direction.normalize();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(&forward).normalize(); // Assuming an up vector of (0, 1, 0)
        let up = forward.cross(&right).normalize();

        // Transform local direction to world space
        let world_direction = right * local_direction.x + up * local_direction.y + forward * local_direction.z;

        // Create the ray
        Ray {
            origin: self.base.origin,  // Camera position
            direction: world_direction.normalize(),  // Transformed direction
        }
    }

    fn trace_pixel(&self, world: &World, x: u32, y: u32) -> Rgb<u8> {
        let ray = self.ray_at((x,y));
        match world.closest_along(&ray) {
            Some(obj) => {
                let info = obj.hit(&ray).unwrap();

                let light = ray.direction.cos_angle(info.normal).abs();

                Rgb(info.color.0.map(|x| (x * light * 255.0) as u8))
            }
            None => Rgb([0,0,0])
        }
    }
    
    pub fn take_photo(&self, world: &World, on_trace: Sender<(u32, u32, Rgb<u8>)>) -> RgbImage {
        RgbImage::from_par_fn(self.width, self.height, |x, y| {
            let rgb = self.trace_pixel(world, x, y);
            match on_trace.send_blocking((x, y, rgb)) {
                Err(e) => {
                    println!("Error sending trace: {:?}", e);
                }
                Ok(_) => {}
            }
            rgb
        })
    }

    pub fn new(base: Ray, width: u32, height: u32, fov: f64) -> Self {
        Self { base, width, height, fov }
    }
}