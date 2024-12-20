mod checkerboard;
pub use checkerboard::CheckerboardTexture;
use crate::scene::geometry::TextureCoords;
use crate::scene::surface::Surface;

pub trait Texture: Send + Sync {
    /// Returns a `Surface` at the given `TextureCoords` on the object's surface.
    fn surface_at(&self, coords: TextureCoords) -> Surface;

    fn clone_box(&self) -> Box<dyn Texture>;
}

impl Texture for Surface {
    fn surface_at(&self, _coords: TextureCoords) -> Surface {
        *self
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}