use crate::algebra::Distance;
use crate::scene::geometry::TextureCoords;
use crate::scene::surface::Surface;
use crate::scene::texture::Texture;

#[derive(Clone)]
pub struct CheckerboardTexture {
    surface1: Surface,
    surface2: Surface,
    scale: Distance, // Number of checkers per unit area
}

impl CheckerboardTexture {
    pub fn new(surface1: Surface, surface2: Surface, scale: Distance) -> Self {
        Self { surface1, surface2, scale }
    }
}

impl Texture for CheckerboardTexture {
    fn surface_at(&self, coords: TextureCoords) -> Surface {
        let (u, v) = coords;
        let checker = ((u * self.scale).floor() + (v * self.scale).floor()) as i32;
        if checker % 2 == 0 {
            self.surface1.clone() // Ensure Surface implements Clone or manually clone if needed
        } else {
            self.surface2.clone()
        }
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        return Box::new(self.clone());
    }
}
