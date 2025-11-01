use crate::domain::entities::{glyph::Glyph, glyph_rendering_engine::GlyphRenderingEngine};

#[derive(Debug)]
pub(crate) struct GlyphRow {
    children: Vec<Box<dyn Glyph>>,
}

impl GlyphRow {
    pub fn empty() -> Self {
        GlyphRow {
            children: Vec::new(),
        }
    }

    pub fn new(children: Vec<Box<dyn Glyph>>) -> Self {
        Self { children }
    }
}

impl Glyph for GlyphRow {
    fn add_child(&mut self, child: Box<dyn Glyph>) {
        self.children.push(child);
    }

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
        self.children.get_mut(_index)
    }

    fn as_text(&self, engine: &dyn GlyphRenderingEngine) -> String {
        self.children
            .iter()
            .map(|c| c.as_text(engine))
            .fold("".to_string(), |acc: String, e: String| format!("{acc}{e}"))
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::glyph_rendering_engine::GlyphRenderingEngine;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn no_children() {
        let row: GlyphRow = GlyphRow::empty();
        let engine: DummyEngine = DummyEngine {};

        assert_eq!(row.as_text(&engine), String::new());
    }

    #[test]
    fn one_child() {
        let engine: DummyEngine = DummyEngine {};
        let mut row: GlyphRow = GlyphRow::empty();

        row.add_child(Box::new(DummyGlyph {
            text: "mock".to_string(),
        }));

        assert_eq!(row.as_text(&engine), "mock");
    }

    #[test]
    fn multiple_children() {
        let engine: DummyEngine = DummyEngine {};
        let row: GlyphRow = GlyphRow::new(vec![
            Box::new(DummyGlyph {
                text: "A".to_string(),
            }),
            Box::new(DummyGlyph {
                text: "B".to_string(),
            }),
            Box::new(DummyGlyph {
                text: "C".to_string(),
            }),
        ]);

        assert_eq!(row.as_text(&engine), "ABC");
    }

    struct DummyEngine;

    impl GlyphRenderingEngine for DummyEngine {
        fn get_char_based_on_luma_value(&self, _: u8) -> char {
            panic!("Unexpected call")
        }
    }

    #[derive(Debug)]
    struct DummyGlyph {
        text: String,
    }

    impl Glyph for DummyGlyph {
        fn as_text(&self, _: &dyn GlyphRenderingEngine) -> String {
            self.text.clone()
        }

        fn add_child(&mut self, _child: Box<dyn Glyph>) {}

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
            None
        }
    }
}
