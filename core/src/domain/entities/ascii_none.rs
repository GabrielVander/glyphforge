use crate::domain::entities::{ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer};

#[derive(Debug)]
pub(crate) struct AsciiNone;

impl AsciiNone {
    pub fn new() -> Self {
        Self
    }
}

impl AsciiRenderable for AsciiNone {
    fn to_ascii(&self, _: &dyn AsciiRenderer) -> String {
        String::new()
    }

    fn add_child(&mut self, _child: Box<dyn AsciiRenderable>) {}

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::{
        ascii_none::AsciiNone, ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn to_ascii_should_render_nothing() {
        let renderer: DummyRenderer = DummyRenderer {};
        let glyph: AsciiNone = AsciiNone::new();

        assert_eq!(glyph.to_ascii(&renderer), "");
    }

    struct DummyRenderer;

    impl AsciiRenderer for DummyRenderer {
        fn render_luma(&self, _luma: u8) -> char {
            panic!("Should not be called");
        }
    }
}
