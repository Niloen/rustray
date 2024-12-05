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

        // Combine calculations for the checker index
        let checker = ((u * self.scale).floor() as i32 + (v * self.scale).floor() as i32) & 1;

        // Use the result of the bitwise AND to select the surface
        if checker == 0 {
            self.surface1
        } else {
            self.surface2
        }
    }


    fn clone_box(&self) -> Box<dyn Texture> {
        return Box::new(self.clone());
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::scene::Color;
    use crate::scene::material::Material;
    use super::*;
    
    #[test]
    fn bench() {
        let s1 = Surface::new(Color::from([1.0, 0.0, 0.0]), Material::DEFAULT);
        let s2 = Surface::new(Color::from([0.0, 1.0, 0.0]), Material::DEFAULT);
        let t = CheckerboardTexture::new(s1, s2, 1.0);
        
        let time = Instant::now();
        for _ in 0..100000000 {
            t.surface_at((0.5, 0.5));
        }
        println!("{:?}", time.elapsed());
    }
}