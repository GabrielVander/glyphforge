struct AsciiRenderer {
    charset: [char; 10],
}

impl AsciiRenderer {
    fn new() -> Self {
        Self {
            charset: [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
        }
    }

    fn render_luma(&self, y: u8) -> char {
        let index: usize = (y as usize * self.charset.len()) / 256;

        self.charset[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_black_luma_value() {
        let renderer: AsciiRenderer = AsciiRenderer::new();

        let result: char = renderer.render_luma(0);

        assert_eq!(result, ' ');
    }

    #[test]
    fn render_white_luma_value() {
        let renderer: AsciiRenderer = AsciiRenderer::new();

        let result: char = renderer.render_luma(255);

        assert_eq!(result, '@');
    }

    #[test]
    fn render_mid_gray_luma_value() {
        let renderer: AsciiRenderer = AsciiRenderer::new();

        let result: char = renderer.render_luma(128);

        assert_eq!(result, '+');
    }
}
