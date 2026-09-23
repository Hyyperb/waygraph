use crate::AppData;
use crate::desktop::DesktopItem;
use crate::fonts::FontRenderer;
use crate::layout::Layout;
use crate::renderer::Rendering;

pub struct SimpleGridLayout {
    desktop_item_size: i32,
    font_renderer: FontRenderer,
    width: i32,
    height: i32,
}

impl SimpleGridLayout {
    pub fn new(
        width: i32,
        height: i32,
        font_name: &'static str,
        font_size: u8,
        desktop_item_size: i32,
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
    fn resolve_position(&self, grid_x: i32, grid_y: i32) -> (i32, i32) {
        let row_count = self.height / self.desktop_item_size;
        let y = self.desktop_item_size * grid_x;
        let x = self.desktop_item_size * grid_y;
        (x, y)
    }

    fn snap(&self, x: f32, y: f32) -> (i32, i32) {
        (
            x as i32 / self.desktop_item_size,
            y as i32 / self.desktop_item_size,
        )
    }
}

impl Layout for SimpleGridLayout {
    fn arrange_items(&self, desktop_items: &mut Vec<DesktopItem>) {
        let row_count = self.height / self.desktop_item_size;

        for i in 0..desktop_items.len() {
            let (x, y) = self.resolve_position(i as i32 % row_count, i as i32 / row_count);
            desktop_items[i].x = x;
            desktop_items[i].y = y;
            desktop_items[i].size = self.desktop_item_size;
        }
    }

    fn get_index_of_item_under_position(
        &self,
        desktop_items: &Vec<DesktopItem>,
        x: f32,
        y: f32,
    ) -> Option<usize> {
        let (ix, iy) = self.snap(x, y);
        let row_count = self.height / self.desktop_item_size;

        let i = ix * row_count + iy;

        Some(i as usize)
    }
}

impl Rendering for SimpleGridLayout {
    fn render(&self, state: &AppData) {
        for desktop_item in state.app.desktop.items.iter() {
            desktop_item.render(state);
            let text_x = (desktop_item.x + self.desktop_item_size / 2) as f32;
            let text_y = (desktop_item.y + desktop_item.size - 10) as f32;

            self.font_renderer
                .render_text(&state, &desktop_item.label, text_x, text_y);
        }
    }
}
