use crate::AppData;
use crate::desktop::DesktopItem;
use crate::fonts::FontRenderer;
use crate::layout::Layout;
use crate::renderer::Rendering;

pub struct SimpleGridLayout {
    desktop_item_size: f32,
    font_renderer: FontRenderer,
    width: f32,
    height: f32,
}

impl SimpleGridLayout {
    pub fn new(
        width: f32,
        height: f32,
        font_name: &'static str,
        font_size: u8,
        desktop_item_size: f32,
    ) -> SimpleGridLayout {
        let font_renderer = FontRenderer::new(font_name, font_size);
        SimpleGridLayout {
            desktop_item_size,
            font_renderer,
            width,
            height,
        }
    }
}

impl SimpleGridLayout {
    fn resolve_position(&self, grid_x: f32, grid_y: f32) -> (f32, f32) {
        let y = self.desktop_item_size * grid_x;
        let x = self.desktop_item_size * grid_y;
        (x, y)
    }

    fn snap(&self, x: f32, y: f32) -> (i32, i32) {
        (
            (x / self.desktop_item_size) as i32,
            (y / self.desktop_item_size) as i32,
        )
    }
}

impl Layout for SimpleGridLayout {
    fn arrange_items(&self, desktop_items: &mut Vec<DesktopItem>) {
        let row_count = (self.height / self.desktop_item_size).floor();

        for (i, desktop_item) in desktop_items.iter_mut().enumerate() {
            let (x, y) =
                self.resolve_position(i as f32 % row_count, (i as f32 / row_count).floor());
            desktop_item.pos.x = x;
            desktop_item.pos.y = y;
            desktop_item.size = self.desktop_item_size;
        }
    }

    fn get_index_of_item_under_position(
        &self,
        desktop_items: &[DesktopItem],
        x: f32,
        y: f32,
    ) -> Option<usize> {
        let (ix, iy) = self.snap(x, y);
        let row_count = (self.height / self.desktop_item_size) as i32;

        let i = ix * row_count + iy;

        if i < desktop_items.len() as i32 {
            Some(i as usize)
        } else {
            None
        }
    }
}

impl Rendering for SimpleGridLayout {
    fn render(&self, state: &AppData) {
        for desktop_item in state.app.desktop.items.iter() {
            desktop_item.render(state);
            let text_x = desktop_item.pos.x + self.desktop_item_size / 2.0;
            let text_y = desktop_item.pos.y + desktop_item.size - 10.0;

            self.font_renderer.render_text(
                state,
                desktop_item.data.name.as_ref().unwrap().as_str(),
                text_x,
                text_y,
            );
        }
    }
}
