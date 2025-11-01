pub trait GlyphRenderingEngine {
    fn get_char_based_on_luma_value(&self, y: u8) -> char;
}
