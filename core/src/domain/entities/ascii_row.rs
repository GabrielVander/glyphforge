use crate::domain::entities::{ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer};

pub(crate) struct AsciiRow<'a> {
    children: Vec<&'a dyn AsciiRenderable>,
}

impl<'a> AsciiRow<'a> {
    pub fn empty() -> Self {
        AsciiRow {
            children: Vec::new(),
        }
    }

    pub fn new(children: Vec<&'a dyn AsciiRenderable>) -> Self {
        Self { children }
    }
}

impl<'a> AsciiRenderable for AsciiRow<'a> {
    fn to_ascii(&self, renderer: &dyn AsciiRenderer) -> String {
        self.children
            .iter()
            .map(|c| c.to_ascii(renderer))
            .fold("".to_string(), |acc: String, e: String| format!("{acc}{e}"))
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::ascii_renderer::AsciiRenderer;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn to_ascii_no_children() {
        let row: AsciiRow = AsciiRow::empty();
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};

        assert_eq!(row.to_ascii(&renderer), String::new());
    }

    #[test]
    fn to_ascii_one_child() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let child: AsciiRenderableMock = AsciiRenderableMock {
            ascii_representation: "mock".to_string(),
        };
        let row: AsciiRow = AsciiRow::new(vec![&child]);

        assert_eq!(row.to_ascii(&renderer), "mock");
    }

    #[test]
    fn to_ascii_multiple_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let child1: AsciiRenderableMock = AsciiRenderableMock {
            ascii_representation: "A".to_string(),
        };
        let child2: AsciiRenderableMock = AsciiRenderableMock {
            ascii_representation: "B".to_string(),
        };
        let child3: AsciiRenderableMock = AsciiRenderableMock {
            ascii_representation: "C".to_string(),
        };
        let row: AsciiRow = AsciiRow::new(vec![&child1, &child2, &child3]);

        assert_eq!(row.to_ascii(&renderer), "ABC");
    }

    struct AsciiRendererPanicImpl;

    impl AsciiRenderer for AsciiRendererPanicImpl {
        fn render_luma(&self, _: u8) -> char {
            panic!("Unexpected call")
        }
    }

    struct AsciiRenderableMock {
        ascii_representation: String,
    }

    impl AsciiRenderable for AsciiRenderableMock {
        fn to_ascii(&self, _: &dyn AsciiRenderer) -> String {
            self.ascii_representation.clone()
        }
    }
}
