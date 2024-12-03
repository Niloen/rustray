use crate::algebra::{Point3, Vector3};
use crate::render::trace::camera::Camera;
use crate::render::trace::world::World;
use crate::render::{RenderListener, Renderer};
use crate::algebra::Ray;
use crate::scene::Scene;
use image::RgbImage;
use std::time::Instant;

pub mod world;
pub mod camera;

pub struct TraceRenderer {
    config: TraceRenderConfig
}

pub struct TraceRenderConfig {
    pub parallel: bool,
}
impl TraceRenderer {
    pub fn new(config: TraceRenderConfig) -> TraceRenderer {
        TraceRenderer {
            config
        }
    }
}
impl Renderer for TraceRenderer {
    fn render(&self, scene: &Scene, width: u32, height: u32, tx: impl RenderListener) -> RgbImage {
        let world = World::from_scene(scene);
        let camera_base = Ray::normalized(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.1, 1.0).normalize());
        let camera = Camera::new(camera_base, width, height, 50.0);

        let photo_start_time = Instant::now();
        let image = camera.take_photo(&world, tx, self.config.parallel);
        let photo_duration = photo_start_time.elapsed();
        println!("Photo generation completed in: {:?}", photo_duration);
        image
    }
}