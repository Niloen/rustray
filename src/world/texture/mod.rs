mod checkerboard;
pub use checkerboard::CheckerboardTexture;
use crate::world::geometry::TextureCoords;
use crate::world::surface::Surface;

pub trait Texture: Send + Sync {
    /// Returns a `Surface` at the given `TextureCoords` on the object's surface.
    fn surface_at(&self, coords: TextureCoords) -> Surface;

    fn clone_box(&self) -> Box<dyn Texture>;
}

impl Texture for Surface {
    fn surface_at(&self, _coords: TextureCoords) -> Surface {
        self.clone()
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}