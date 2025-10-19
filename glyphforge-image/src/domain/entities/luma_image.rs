use glyphforge_core::domain::entities::{Glyph, GlyphGrid, GlyphRenderingEngine};

#[derive(Debug)]
pub struct LumaImage {
    width: usize,
    height: usize,
    pixels: GlyphGrid,
}

impl LumaImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: GlyphGrid::new(width, height),
        }
    }

    pub fn is_full(&self) -> bool {
        self.pixels.elements_count >= self.width * self.height
    }
}

impl Glyph for LumaImage {
    fn as_text(&self, engine: &dyn GlyphRenderingEngine) -> String {
        self.pixels.as_text(engine)
    }

    fn add_child(&mut self, child: Box<dyn Glyph>) {
        self.pixels.add_child(child);
    }

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use glyphforge_core::domain::entities::{Glyph, GlyphRenderingEngine};
    use pretty_assertions::assert_eq;

    use crate::domain::entities::{luma_image::LumaImage, luma_pixel::LumaPixel};

    #[test]
    fn should_operate_as_glyph_composite() {
        let engine: DummyEngine = DummyEngine;
        let mut image: LumaImage = LumaImage::new(1, 1);

        image.add_child(Box::new(LumaPixel::new(255)));
        image.add_child(Box::new(LumaPixel::new(255)));
        image.add_child(Box::new(LumaPixel::new(255)));

        assert_eq!(image.as_text(&engine), "X");
    }

    struct DummyEngine;

    impl GlyphRenderingEngine for DummyEngine {
        fn get_char_based_on_luma_value(&self, _y: u8) -> char {
            'X'
        }
    }
}
