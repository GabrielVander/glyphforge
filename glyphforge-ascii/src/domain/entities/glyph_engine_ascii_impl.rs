use glyphforge_core::domain::entities::GlyphRenderingEngine;

pub(crate) struct GlyphEngineAsciiImpl {
    charset: [char; 10],
}

impl GlyphEngineAsciiImpl {
    pub fn new() -> Self {
        Self {
            charset: [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
        }
    }
}

impl GlyphRenderingEngine for GlyphEngineAsciiImpl {
    fn get_char_based_on_luma_value(&self, y: u8) -> char {
        let index: usize = (y as usize * self.charset.len()) / 256;

        self.charset[index]
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn black_luma_value() {
        let engine: GlyphEngineAsciiImpl = GlyphEngineAsciiImpl::new();

        let result: char = engine.get_char_based_on_luma_value(0);

        assert_eq!(result, ' ');
    }

    #[test]
    fn white_luma_value() {
        let engine: GlyphEngineAsciiImpl = GlyphEngineAsciiImpl::new();

        let result: char = engine.get_char_based_on_luma_value(255);

        assert_eq!(result, '@');
    }

    #[test]
    fn mid_gray_luma_value() {
        let engine: GlyphEngineAsciiImpl = GlyphEngineAsciiImpl::new();

        let result: char = engine.get_char_based_on_luma_value(128);

        assert_eq!(result, '+');
    }
}
