use crate::{AppData, fonts::FontRenderer, renderer::Rendering};

pub struct TextLabel {
    renderer: FontRenderer,
    text: &'static str,
    x: f32,
    y: f32,
}

impl Rendering for TextLabel {
    fn render(&self, state: &AppData) {
        self.renderer.render_text(state, self.text, self.x, self.y);
    }
}
