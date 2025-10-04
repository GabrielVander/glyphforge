use crate::domain::entities::{
    ascii_col::AsciiCol, ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer,
    ascii_row::AsciiRow,
};

#[derive(Debug)]
struct AsciiGrid {
    main_col: AsciiCol,
    elements_count: usize,
    width: usize,
    height: usize,
}

impl AsciiGrid {
    fn new(width: usize, height: usize) -> Self {
        let mut rows: Vec<Box<dyn AsciiRenderable>> = Vec::with_capacity(height);

        for _ in 0..width {
            rows.push(Box::new(AsciiRow::new(Vec::with_capacity(width))));
        }
        Self {
            main_col: AsciiCol::new(rows),
            elements_count: 0,
            width,
            height,
        }
    }

    fn is_finished(&self) -> bool {
        self.elements_count >= self.width * self.height
    }

    fn add_glyph(&mut self, new_gyph: Box<dyn AsciiRenderable>) {
        let col_index: usize = self.elements_count / self.width;

        if let Some(i) = self.main_col.get_child_mut(col_index) {
            i.add_child(new_gyph)
        }

        self.elements_count += 1;
    }
}

impl AsciiRenderable for AsciiGrid {
    fn to_ascii(&self, renderer: &dyn AsciiRenderer) -> String {
        println!("Grid to_ascii");
        self.main_col.to_ascii(renderer)
    }

    fn add_child(&mut self, child: Box<dyn AsciiRenderable>) {
        if self.is_finished() {
            return;
        }

        self.add_glyph(child);
    }

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::domain::entities::{
        ascii_grid::AsciiGrid, ascii_renderable::AsciiRenderable, ascii_renderer::AsciiRenderer,
    };

    #[test]
    fn image_to_ascii() {
        let expected_output: &str = "XX\nXX";

        let renderer: DummyRenderer = DummyRenderer;
        let mut grid: AsciiGrid = AsciiGrid::new(2, 2);

        for _ in 0..3 {
            grid.add_child(Box::new(DummyGlyph));
            assert!(!grid.is_finished());
        }

        grid.add_child(Box::new(DummyGlyph));

        assert!(grid.is_finished());
        println!("{:?}", grid);
        let result: String = grid.to_ascii(&renderer);

        assert_eq!(expected_output, result);
    }

    #[derive(Debug)]
    struct DummyGlyph;

    impl AsciiRenderable for DummyGlyph {
        fn to_ascii(&self, _renderer: &dyn AsciiRenderer) -> String {
            "X".to_string()
        }

        fn add_child(&mut self, _child: Box<dyn AsciiRenderable>) {
            todo!()
        }

        fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
            todo!()
        }
    }

    struct DummyRenderer;

    impl AsciiRenderer for DummyRenderer {
        fn render_luma(&self, _luma: u8) -> char {
            panic!("Should not be called");
        }
    }
}
