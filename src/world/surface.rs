use image::Rgb;
use crate::world::material::Material;

#[derive(Debug)]
pub struct Surface {
    pub color: Rgb<f64>,
    pub material: Box<dyn Material>,
}

impl Surface {
    pub fn new(color: Rgb<f64>, material: &dyn Material) -> Self {
        Self { color, material: material.clone_box() }
    }
}

impl<'a> Clone for Surface {
    fn clone(&self) -> Self {
        Surface {
            material: self.material.clone_box(),
            color: self.color,
        }
    }
}
