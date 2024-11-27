use std::sync::Arc;
use image::Rgb;
use crate::scene::light::Light;

pub mod geometry;
pub mod texture;
pub mod light;
pub mod material;
pub mod surface;
mod transform;
pub mod object;
pub mod ray;

pub type ColorPart = f32;
pub type Color = Rgb<ColorPart>;

pub struct Scene {
    objects: Vec<Arc<object::Object>>,
    lights: Vec<Light>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![], lights: vec![] }
    }
    
    pub fn add(&mut self, object: object::Object) {
        self.objects.push(Arc::new(object));
    }
    
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    
    pub fn iter_objects(&self) -> std::slice::Iter<Arc<object::Object>> {
        self.objects.iter()
    }
    
    pub fn iter_lights(&self) -> std::slice::Iter<Light> {
        self.lights.iter()
    }
}