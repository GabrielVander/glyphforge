use crate::domain::entities::ascii_renderable::AsciiRenderable;

pub(crate) struct AsciiCol {
    children: Vec<Box<dyn AsciiRenderable>>,
}

impl AsciiCol {
    fn empty() -> Self {
        AsciiCol {
            children: Vec::new(),
        }
    }

    pub fn new(children: Vec<Box<dyn AsciiRenderable>>) -> Self {
        Self { children }
    }
}

impl AsciiRenderable for AsciiCol {
    fn to_ascii(&self, renderer: &dyn super::ascii_renderer::AsciiRenderer) -> String {
        self.children
            .iter()
            .map(|c| c.to_ascii(renderer))
            .fold("".to_string(), |acc: String, e: String| {
                format!("{acc}{e}\n")
            })
            .trim_end()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::ascii_renderer::AsciiRenderer;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn to_ascii_no_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let col: AsciiCol = AsciiCol::empty();

        assert_eq!(col.to_ascii(&renderer), "")
    }

    #[test]
    fn to_ascii_single_child() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let col: AsciiCol = AsciiCol::new(vec![Box::new(FakeAsciiRenderable {
            ascii_representation_to_be_returned: "A".to_string(),
        })]);

        assert_eq!(col.to_ascii(&renderer), "A");
    }

    #[test]
    fn to_ascii_multiple_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let col: AsciiCol = AsciiCol::new(vec![
            Box::new(FakeAsciiRenderable {
                ascii_representation_to_be_returned: "A".to_string(),
            }),
            Box::new(FakeAsciiRenderable {
                ascii_representation_to_be_returned: "B".to_string(),
            }),
            Box::new(FakeAsciiRenderable {
                ascii_representation_to_be_returned: "C".to_string(),
            }),
        ]);

        assert_eq!(col.to_ascii(&renderer), "A\nB\nC");
    }

    struct AsciiRendererPanicImpl;

    impl AsciiRenderer for AsciiRendererPanicImpl {
        fn render_luma(&self, y: u8) -> char {
            panic!("Unexpected call with y={}", y)
        }
    }

    struct FakeAsciiRenderable {
        ascii_representation_to_be_returned: String,
    }

    impl AsciiRenderable for FakeAsciiRenderable {
        fn to_ascii(&self, _: &dyn AsciiRenderer) -> String {
            self.ascii_representation_to_be_returned.clone()
        }
    }
}
