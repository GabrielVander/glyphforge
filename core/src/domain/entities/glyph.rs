use crate::domain::entities::glyph_rendering_engine::GlyphRenderingEngine;

pub(crate) trait Glyph: std::fmt::Debug {
    fn as_text(&self, _engine: &dyn GlyphRenderingEngine) -> String;

    fn add_child(&mut self, _child: Box<dyn Glyph>);

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>>;
}
