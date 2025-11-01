use crate::domain::entities::{
    glyph::Glyph, glyph_col::GlyphCol, glyph_rendering_engine::GlyphRenderingEngine,
    glyph_row::GlyphRow,
};

#[derive(Debug)]
pub struct GlyphGrid {
    main_col: GlyphCol,
    pub elements_count: usize,
    width: usize,
    height: usize,
}

impl GlyphGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rows: Vec<Box<dyn Glyph>> = Vec::with_capacity(height);

        for _ in 0..width {
            rows.push(Box::new(GlyphRow::new(Vec::with_capacity(width))));
        }
        Self {
            main_col: GlyphCol::new(rows),
            elements_count: 0,
            width,
            height,
        }
    }

    fn is_finished(&self) -> bool {
        self.elements_count >= self.width * self.height
    }

    fn add_glyph(&mut self, new_gyph: Box<dyn Glyph>) {
        let col_index: usize = self.elements_count / self.width;

        if let Some(i) = self.main_col.get_child_mut(col_index) {
            i.add_child(new_gyph)
        }

        self.elements_count += 1;
    }
}

impl Glyph for GlyphGrid {
    fn as_text(&self, engine: &dyn GlyphRenderingEngine) -> String {
        self.main_col.as_text(engine)
    }

    fn add_child(&mut self, child: Box<dyn Glyph>) {
        if self.is_finished() {
            return;
        }

        self.add_glyph(child);
    }

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::domain::entities::{
        glyph::Glyph, glyph_grid::GlyphGrid, glyph_rendering_engine::GlyphRenderingEngine,
    };

    #[test]
    fn grid_operations() {
        let expected_output: &str = "XX\nXX";

        let engine: DummyEngine = DummyEngine;
        let mut grid: GlyphGrid = GlyphGrid::new(2, 2);

        for _ in 0..3 {
            grid.add_child(Box::new(DummyGlyph));
            assert!(!grid.is_finished());
        }

        grid.add_child(Box::new(DummyGlyph));

        assert!(grid.is_finished());
        println!("{:?}", grid);
        let result: String = grid.as_text(&engine);

        assert_eq!(expected_output, result);
    }

    #[derive(Debug)]
    struct DummyGlyph;

    impl Glyph for DummyGlyph {
        fn as_text(&self, _engine: &dyn GlyphRenderingEngine) -> String {
            "X".to_string()
        }

        fn add_child(&mut self, _child: Box<dyn Glyph>) {
            todo!()
        }

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn Glyph>> {
            todo!()
        }
    }

    struct DummyEngine;

    impl GlyphRenderingEngine for DummyEngine {
        fn get_char_based_on_luma_value(&self, _luma: u8) -> char {
            panic!("Should not be called");
        }
    }
}
