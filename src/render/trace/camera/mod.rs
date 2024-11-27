extern crate image;

use crate::algebra::{Distance, Vector3};
use crate::scene::ray::RayCaster;
use crate::algebra::Ray;
use image::{Rgb, RgbImage};

pub struct Camera {
    base: Ray,
    width: u32,
    height: u32,
    pixel_step_x: Vector3,
    pixel_step_y: Vector3,
    corner: Vector3, // Precomputed top-left corner of the camera plane
}

impl Camera {

    pub fn new(base: Ray, width: u32, height: u32, fov: Distance) -> Self {
        let aspect_ratio = width as Distance / height as Distance;
        let fov_radians = (fov.to_radians()) / 2.0;
        let scale = fov_radians.tan();

        // Compute camera basis vectors
        let forward = base.direction.normalize();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(&forward).normalize(); // Assuming up = (0, 1, 0)
        let up = forward.cross(&right).normalize();

        // Calculate pixel step sizes
        let pixel_width = 2.0 * aspect_ratio * scale / width as Distance;
        let pixel_height = 2.0 * scale / height as Distance;

        // Precompute pixel step vectors
        let pixel_step_x = right * pixel_width;
        let pixel_step_y = -up * pixel_height; // Negative because y-axis is flipped in screen space

        // Compute the top-left corner of the camera plane
        let half_width = width as Distance / 2.0;
        let half_height = height as Distance / 2.0;
        let corner = forward
            - pixel_step_x * half_width
            - pixel_step_y * half_height;

        Self {
            base,
            width,
            height,
            pixel_step_x,
            pixel_step_y,
            corner,
        }
    }

    fn ray_at(&self, coord: (u32, u32)) -> Ray {
        let (x, y) = coord;

        // Compute the world direction for the current pixel
        let world_direction = self.corner
            + self.pixel_step_x * x as Distance
            + self.pixel_step_y * y as Distance;

        Ray::new(self.base.origin, world_direction)
    }

    fn trace_pixel(&self, caster: &impl RayCaster, x: u32, y: u32) -> Rgb<u8> {
        let ray = self.ray_at((x,y));        

        let color = caster.cast(&ray, 5);

        Rgb(color.0.map(|x| (x * 255.0) as u8))
    }

    pub fn take_photo(&self, caster: &impl RayCaster, on_trace: impl Fn((u32, u32, Rgb<u8>)) + Send + Sync, parallel: bool) -> RgbImage {
        (if parallel { RgbImage::from_par_fn } else { RgbImage::from_fn })(self.width, self.height, |x, y| {
            let rgb = self.trace_pixel(caster, x, y);
            on_trace((x, y, rgb));
            rgb
        })
    }
}