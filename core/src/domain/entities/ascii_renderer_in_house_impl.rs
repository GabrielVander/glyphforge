use crate::domain::entities::ascii_renderer::AsciiRenderer;

struct AsciiRendererInHouseImpl {
    charset: [char; 10],
}

impl AsciiRendererInHouseImpl {
    fn new() -> Self {
        Self {
            charset: [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
        }
    }
}

impl AsciiRenderer for AsciiRendererInHouseImpl {
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
        let renderer: AsciiRendererInHouseImpl = AsciiRendererInHouseImpl::new();

        let result: char = renderer.render_luma(0);

        assert_eq!(result, ' ');
    }

    #[test]
    fn render_white_luma_value() {
        let renderer: AsciiRendererInHouseImpl = AsciiRendererInHouseImpl::new();

        let result: char = renderer.render_luma(255);

        assert_eq!(result, '@');
    }

    #[test]
    fn render_mid_gray_luma_value() {
        let renderer: AsciiRendererInHouseImpl = AsciiRendererInHouseImpl::new();

        let result: char = renderer.render_luma(128);

        assert_eq!(result, '+');
    }
}
