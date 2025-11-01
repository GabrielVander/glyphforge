use glyphforge_core::domain::entities::{Glyph, GlyphRenderingEngine};

use crate::domain::entities::rgb_pixel::RgbPixel;

#[derive(Debug, PartialEq)]
pub struct LumaPixel {
    pub y: u8,
}

impl LumaPixel {
    pub fn new(y: u8) -> Self {
        Self { y }
    }

    pub(crate) fn from_rgb(rgb: RgbPixel) -> Self {
        let (r, g, b) = (rgb.r, rgb.g, rgb.b);

        let luma_value: u8 = ((2126 * r as u32 + 7152 * g as u32 + 722 * b as u32) / 10000) as u8;
        Self { y: luma_value }
    }
}

impl Glyph for LumaPixel {
    fn as_text(&self, renderer: &dyn GlyphRenderingEngine) -> String {
        renderer.get_char_based_on_luma_value(self.y).to_string()
    }

    fn add_child(&mut self, _child: Box<dyn Glyph>) {}

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::{luma_pixel::LumaPixel, rgb_pixel::RgbPixel};

    use glyphforge_core::domain::entities::{Glyph, GlyphRenderingEngine};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_rgb_values_to_luma_correctly() {
        let rgb_values: Vec<RgbPixel> = vec![
            RgbPixel::new(0, 0, 0),       // Black
            RgbPixel::new(255, 255, 255), // White
            RgbPixel::new(255, 0, 0),     // Red
            RgbPixel::new(0, 255, 0),     // Green
            RgbPixel::new(0, 0, 255),     // Blue
            RgbPixel::new(128, 128, 128), // Gray
            RgbPixel::new(255, 255, 0),   // Yellow
            RgbPixel::new(0, 255, 255),   // Cyan
            RgbPixel::new(255, 0, 255),   // Magenta
        ];
        let expected: Vec<LumaPixel> = vec![0, 255, 54, 182, 18, 128, 236, 200, 72]
            .into_iter()
            .map(LumaPixel::new)
            .collect();

        let results: Vec<LumaPixel> = rgb_values.into_iter().map(LumaPixel::from_rgb).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn should_behave_as_leaf_glyph() {
        let mut pixel: LumaPixel = LumaPixel::new(128);

        pixel.add_child(Box::new(LumaPixel::new(0)));
        assert!(pixel.get_child_mut(0).is_none());
    }

    #[test]
    fn should_use_engine_to_represent_text() {
        let luma_values: Vec<u8> = vec![0, 255, 54, 182, 18, 128, 236, 200, 72];

        for y in luma_values {
            let engine: MockEngine = MockEngine { expected_y: y };

            let pixel: LumaPixel = LumaPixel::new(y);

            assert_eq!("X", pixel.as_text(&engine));
        }
    }

    struct MockEngine {
        expected_y: u8,
    }

    impl GlyphRenderingEngine for MockEngine {
        fn get_char_based_on_luma_value(&self, y: u8) -> char {
            assert_eq!(self.expected_y, y);
            'X'
        }
    }
}
