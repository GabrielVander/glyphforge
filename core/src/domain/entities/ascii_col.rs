use crate::domain::entities::{ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer};

#[derive(Debug)]
pub(crate) struct AsciiCol {
    children: Vec<Box<dyn AsciiRenderable>>,
}

impl AsciiCol {
    pub fn empty() -> Self {
        AsciiCol {
            children: Vec::new(),
        }
    }

    pub fn new(children: Vec<Box<dyn AsciiRenderable>>) -> Self {
        Self { children }
    }
}

impl AsciiRenderable for AsciiCol {
    fn to_ascii(&self, renderer: &dyn AsciiRenderer) -> String {
        println!("AsciiCol to_ascii");
        self.children
            .iter()
            .map(|c| c.to_ascii(renderer))
            .fold("".to_string(), |acc: String, e: String| {
                format!("{acc}{e}\n")
            })
            .trim_end()
            .to_string()
    }

    fn add_child(&mut self, child: Box<dyn AsciiRenderable>) {
        self.children.push(child);
    }

    fn get_child_mut(&mut self, index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
        self.children.get_mut(index)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::{
        ascii_col::AsciiCol, ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn to_ascii_no_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let col: AsciiCol = AsciiCol::empty();

        assert_eq!(col.to_ascii(&renderer), "")
    }

    #[test]
    fn to_ascii_single_child() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let mut col: AsciiCol = AsciiCol::empty();

        col.add_child(Box::new(FakeAsciiRenderable {
            ascii_representation_to_be_returned: "A".to_string(),
        }));

        assert_eq!(col.to_ascii(&renderer), "A");
    }

    #[test]
    fn to_ascii_multiple_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let mut col: AsciiCol = AsciiCol::empty();

        let glyphs: Vec<FakeAsciiRenderable> = vec![
            FakeAsciiRenderable {
                ascii_representation_to_be_returned: "A".to_string(),
            },
            FakeAsciiRenderable {
                ascii_representation_to_be_returned: "B".to_string(),
            },
            FakeAsciiRenderable {
                ascii_representation_to_be_returned: "C".to_string(),
            },
        ];

        for e in glyphs {
            col.add_child(Box::new(e));
        }

        assert_eq!(col.to_ascii(&renderer), "A\nB\nC");
    }

    struct AsciiRendererPanicImpl;

    impl AsciiRenderer for AsciiRendererPanicImpl {
        fn render_luma(&self, y: u8) -> char {
            panic!("Unexpected call with y={}", y)
        }
    }

    #[derive(Clone, Debug)]
    struct FakeAsciiRenderable {
        ascii_representation_to_be_returned: String,
    }

    impl AsciiRenderable for FakeAsciiRenderable {
        fn to_ascii(&self, _: &dyn AsciiRenderer) -> String {
            self.ascii_representation_to_be_returned.clone()
        }

        fn add_child(&mut self, _child: Box<dyn AsciiRenderable>) {
            todo!()
        }

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
            todo!()
        }
    }
}
