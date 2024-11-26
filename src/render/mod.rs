mod trace;

use image::{Rgb, RgbImage};
use crate::scene::Scene;
pub use trace::{TraceRenderer, TraceRenderConfig};

pub trait RenderListener: Fn((u32, u32, Rgb<u8>)) + Send + Sync {}
impl<T> RenderListener for T where T: Fn((u32, u32, Rgb<u8>)) + Send + Sync {}

pub trait Renderer {
    fn render(&self, scene: &Scene, width: u32, height: u32, tx: impl RenderListener) -> RgbImage;
}
