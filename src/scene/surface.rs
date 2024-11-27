use crate::scene::Color;
use crate::scene::material::Material;

#[derive(Debug)]
pub struct Surface {
    pub color: Color,
    pub material: Box<dyn Material>,
}

impl Surface {
    pub fn new(color: Color, material: &dyn Material) -> Self {
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
