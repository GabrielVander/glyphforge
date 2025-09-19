use crate::domain::entities::{ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer};

struct AsciiArt<'a> {
    children: Vec<&'a dyn AsciiRenderable>,
}

impl<'a> AsciiArt<'a> {
    fn new(children: Vec<&'a dyn AsciiRenderable>) -> Self {
        Self { children }
    }
}

impl<'a> AsciiRenderable for AsciiArt<'a> {
    fn to_ascii(&self, renderer: &dyn AsciiRenderer) -> String {
        self.children
            .iter()
            .map(|c| c.to_ascii(renderer))
            .fold("".to_string(), |acc: String, e: String| format!("{acc}{e}"))
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::{
        ascii_col::AsciiCol, ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer,
        ascii_row::AsciiRow,
    };

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn to_ascii_grid() {
        let renderer: AsciiRendererPanicImpl = AsciiRendererPanicImpl {};
        let aa: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "AA".to_string(),
        };
        let ab: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "AB".to_string(),
        };
        let ac: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "AC".to_string(),
        };
        let ba: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "BA".to_string(),
        };
        let bb: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "BB".to_string(),
        };
        let bc: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "BC".to_string(),
        };
        let ca: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "CA".to_string(),
        };
        let cb: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "CB".to_string(),
        };
        let cc: AsciiRenderableConstant = AsciiRenderableConstant {
            ascii_representation: "CC".to_string(),
        };
        let a_row: AsciiRow = AsciiRow::new(vec![&aa, &ab, &ac]);
        let b_row: AsciiRow = AsciiRow::new(vec![&ba, &bb, &bc]);
        let c_row: AsciiRow = AsciiRow::new(vec![&ca, &cb, &cc]);
        let main_col: AsciiCol = AsciiCol::new(vec![&a_row, &b_row, &c_row]);
        let grid: AsciiArt = AsciiArt::new(vec![&main_col]);

        assert_eq!(grid.to_ascii(&renderer), "AAABAC\nBABBBC\nCACBCC");
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
