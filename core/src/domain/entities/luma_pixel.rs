use crate::domain::entities::{ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer};

struct LumaPixel {
    y: u8,
}

impl AsciiRenderable for LumaPixel {
    fn to_ascii(&self, renderer: Box<dyn AsciiRenderer>) -> String {
        renderer.render_luma(self.y).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_ascii_uses_y_value() {
        let random_y_value: u8 = 81;
        let renderer: Box<MockAsciiRenderer> = Box::new(MockAsciiRenderer {
            expected_luma_y_value: random_y_value,
        });
        let pixel: LumaPixel = LumaPixel { y: random_y_value };

        assert_eq!(pixel.to_ascii(renderer), "X");
    }

    struct MockAsciiRenderer {
        expected_luma_y_value: u8,
    }

    impl AsciiRenderer for MockAsciiRenderer {
        fn render_luma(&self, y: u8) -> char {
            assert_eq!(self.expected_luma_y_value, y);

            'X'
        }
    }
}
