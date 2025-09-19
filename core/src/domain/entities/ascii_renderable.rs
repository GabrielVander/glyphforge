use crate::domain::entities::ascii_renderer::AsciiRenderer;

pub(crate) trait AsciiRenderable {
    fn to_ascii(&self, renderer: &impl AsciiRenderer) -> String;
}
