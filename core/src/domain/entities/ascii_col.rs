use crate::domain::entities::ascii_renderable::AsciiRenderable;

struct AsciiCol<'a> {
    children: Vec<&'a dyn AsciiRenderable>,
}
impl<'a> AsciiCol<'a> {
    fn empty() -> Self {
        AsciiCol {
            children: Vec::new(),
        }
    }

    fn new(children: Vec<&'a dyn AsciiRenderable>) -> Self {
        Self { children }
    }
}

impl<'a> AsciiRenderable for AsciiCol<'a> {
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
        let child: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "A".to_string(),
        };
        let col: AsciiCol = AsciiCol::new(vec![&child]);

        assert_eq!(col.to_ascii(&renderer), "A");
    }

    #[test]
    fn to_ascii_multiple_children() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let child1: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "A".to_string(),
        };
        let child2: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "B".to_string(),
        };
        let child3: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "C".to_string(),
        };
        let col: AsciiCol = AsciiCol::new(vec![&child1, &child2, &child3]);

        assert_eq!(col.to_ascii(&renderer), "A\nB\nC");
    }

    struct AsciiRendererPanicImpl;

    impl AsciiRenderer for AsciiRendererPanicImpl {
        fn render_luma(&self, y: u8) -> char {
            panic!("Unexpected call with y={}", y)
        }
    }

    struct AsciiRenderableConstant {
        ascii_representation: String,
    }

    impl AsciiRenderable for AsciiRenderableConstant {
        fn to_ascii(&self, _: &dyn AsciiRenderer) -> String {
            self.ascii_representation.clone()
        }
    }
}
