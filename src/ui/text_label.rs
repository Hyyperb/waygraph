use crate::{AppData, renderer::Rendering};

pub struct TextLabel {
    pub text: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Rendering for TextLabel {
    fn render(&self, state: &AppData) {
        state
            .app
            .font
            .font_renderer
            .as_ref()
            .unwrap()
            .render_text(state, self.text, self.x, self.y);
    }
}
