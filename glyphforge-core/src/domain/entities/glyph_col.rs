use crate::domain::entities::{glyph::Glyph, glyph_rendering_engine::GlyphRenderingEngine};

#[derive(Debug)]
pub(crate) struct GlyphCol {
    children: Vec<Box<dyn Glyph>>,
}

impl GlyphCol {
    pub fn empty() -> Self {
        GlyphCol {
            children: Vec::new(),
        }
    }

    pub fn new(children: Vec<Box<dyn Glyph>>) -> Self {
        Self { children }
    }
}

impl Glyph for GlyphCol {
    fn as_text(&self, engine: &dyn GlyphRenderingEngine) -> String {
        self.children
            .iter()
            .map(|c| c.as_text(engine))
            .fold("".to_string(), |acc: String, e: String| {
                format!("{acc}{e}\n")
            })
            .trim_end()
            .to_string()
    }

    fn add_child(&mut self, child: Box<dyn Glyph>) {
        self.children.push(child);
    }

    fn get_child_mut(&mut self, index: usize) -> Option<&mut Box<dyn Glyph>> {
        self.children.get_mut(index)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::{
        glyph::Glyph, glyph_col::GlyphCol, glyph_rendering_engine::GlyphRenderingEngine,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn no_children() {
        let engine: DummyEngine = DummyEngine {};
        let col: GlyphCol = GlyphCol::empty();

        assert_eq!(col.as_text(&engine), "")
    }

    #[test]
    fn single_child() {
        let engine: DummyEngine = DummyEngine {};
        let mut col: GlyphCol = GlyphCol::empty();

        col.add_child(Box::new(DummyGlyphA));

        assert_eq!(col.as_text(&engine), "A");
    }

    #[test]
    fn multiple_children() {
        let engine: DummyEngine = DummyEngine {};
        let mut col: GlyphCol = GlyphCol::empty();

        let glyphs: Vec<Box<dyn Glyph>> = vec![
            Box::new(DummyGlyphA),
            Box::new(DummyGlyphB),
            Box::new(DummyGlyphC),
        ];

        for e in glyphs {
            col.add_child(e);
        }

        assert_eq!(col.as_text(&engine), "A\nB\nC");
    }

    struct DummyEngine;

    impl GlyphRenderingEngine for DummyEngine {
        fn get_char_based_on_luma_value(&self, y: u8) -> char {
            panic!("Unexpected call with y={}", y)
        }
    }

    #[derive(Clone, Debug)]
    struct DummyGlyphA;

    impl Glyph for DummyGlyphA {
        fn as_text(&self, _engine: &dyn GlyphRenderingEngine) -> String {
            "A".to_string()
        }

        fn add_child(&mut self, _child: Box<dyn Glyph>) {
            todo!()
        }

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
            todo!()
        }
    }

    #[derive(Clone, Debug)]
    struct DummyGlyphB;

    impl Glyph for DummyGlyphB {
        fn as_text(&self, _engine: &dyn GlyphRenderingEngine) -> String {
            "B".to_string()
        }

        fn add_child(&mut self, _child: Box<dyn Glyph>) {
            todo!()
        }

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
            todo!()
        }
    }

    #[derive(Clone, Debug)]
    struct DummyGlyphC;

    impl Glyph for DummyGlyphC {
        fn as_text(&self, _engine: &dyn GlyphRenderingEngine) -> String {
            "C".to_string()
        }

        fn add_child(&mut self, _child: Box<dyn Glyph>) {
            todo!()
        }

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
            todo!()
        }
    }
}
