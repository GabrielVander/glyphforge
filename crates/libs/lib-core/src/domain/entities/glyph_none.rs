use crate::domain::entities::{glyph::Glyph, glyph_rendering_engine::GlyphRenderingEngine};

#[derive(Debug)]
pub(crate) struct GlyphNone;

impl GlyphNone {
    pub fn new() -> Self {
        Self
    }
}

impl Glyph for GlyphNone {
    fn as_text(&self, _: &dyn GlyphRenderingEngine) -> String {
        String::new()
    }

    fn add_child(&mut self, _child: Box<dyn Glyph>) {}

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::{
        glyph::Glyph, glyph_none::GlyphNone, glyph_rendering_engine::GlyphRenderingEngine,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_render_nothing() {
        let engine: DummyEngine = DummyEngine {};
        let glyph: GlyphNone = GlyphNone::new();

        assert_eq!(glyph.as_text(&engine), "");
    }

    struct DummyEngine;

    impl GlyphRenderingEngine for DummyEngine {
        fn get_char_based_on_luma_value(&self, _luma: u8) -> char {
            panic!("Should not be called");
        }
    }
}
