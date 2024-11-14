mod checkerboard;
pub use checkerboard::CheckerboardTexture;

use crate::world::surface::Surface;
pub type TextureCoords = (f64, f64);

pub trait Texture<'a>: Send + Sync {
    /// Returns a `Surface` at the given `TextureCoords` on the object's surface.
    fn surface_at(&self, coords: TextureCoords) -> Surface;

    fn clone_box(&self) -> Box<dyn Texture<'a> + 'a>;
}

impl<'a> Texture<'a> for Surface<'a> {
    fn surface_at(&self, _coords: TextureCoords) -> Surface {
        self.clone()
    }

    fn clone_box(&self) -> Box<dyn Texture<'a> + 'a> {
        Box::new(self.clone())
    }
}