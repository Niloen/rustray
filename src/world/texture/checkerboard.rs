use crate::world::surface::Surface;
use crate::world::texture::{Texture, TextureCoords};

#[derive(Clone)]
pub struct CheckerboardTexture {
    surface1: Surface,
    surface2: Surface,
    scale: f64, // Number of checkers per unit area
}

impl CheckerboardTexture {
    pub fn new(surface1: Surface, surface2: Surface, scale: f64) -> Self {
        Self { surface1, surface2, scale }
    }
}

impl<'a> Texture<'a> for CheckerboardTexture {
    fn surface_at(&self, coords: TextureCoords) -> Surface {
        let (u, v) = coords;
        let checker = ((u * self.scale).floor() + (v * self.scale).floor()) as i32;
        if checker % 2 == 0 {
            self.surface1.clone() // Ensure Surface implements Clone or manually clone if needed
        } else {
            self.surface2.clone()
        }
    }

    fn clone_box(&self) -> Box<dyn Texture<'a> + 'a> {
        return Box::new(self.clone());
    }
}
