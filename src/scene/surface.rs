use crate::scene::Color;
use crate::scene::material::Material;

#[derive(Debug, Copy, Clone)]
pub struct Surface {
    pub color: Color,
    pub material: Material,
}

impl Surface {
    pub fn new(color: Color, material: Material) -> Self {
        Self { color, material: material }
    }
}
