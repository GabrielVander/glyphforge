use crate::domain::entities::{ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer};

#[derive(Debug)]
pub(crate) struct AsciiRow {
    children: Vec<Box<dyn AsciiRenderable>>,
}

impl AsciiRow {
    pub fn empty() -> Self {
        AsciiRow {
            children: Vec::new(),
        }
    }

    pub fn new(children: Vec<Box<dyn AsciiRenderable>>) -> Self {
        Self { children }
    }
}

impl AsciiRenderable for AsciiRow {
    fn add_child(&mut self, child: Box<dyn AsciiRenderable>) {
        self.children.push(child);
    }

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
        self.children.get_mut(_index)
    }

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
        let mut row: AsciiRow = AsciiRow::empty();

        row.add_child(Box::new(AsciiRenderableMock {
            ascii_representation: "mock".to_string(),
        }));

        assert_eq!(row.to_ascii(&renderer), "mock");
    }

    #[test]
    fn to_ascii_multiple_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let row: AsciiRow = AsciiRow::new(vec![
            Box::new(AsciiRenderableMock {
                ascii_representation: "A".to_string(),
            }),
            Box::new(AsciiRenderableMock {
                ascii_representation: "B".to_string(),
            }),
            Box::new(AsciiRenderableMock {
                ascii_representation: "C".to_string(),
            }),
        ]);

        assert_eq!(row.to_ascii(&renderer), "ABC");
    }

    struct AsciiRendererPanicImpl;

    impl AsciiRenderer for AsciiRendererPanicImpl {
        fn render_luma(&self, _: u8) -> char {
            panic!("Unexpected call")
        }
    }

    #[derive(Debug)]
    struct AsciiRenderableMock {
        ascii_representation: String,
    }

    impl AsciiRenderable for AsciiRenderableMock {
        fn to_ascii(&self, _: &dyn AsciiRenderer) -> String {
            self.ascii_representation.clone()
        }

        fn add_child(&mut self, _child: Box<dyn AsciiRenderable>) {}

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
            None
        }
    }
}
