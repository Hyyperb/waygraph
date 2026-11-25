// use std::io::{BufRead, BufReader};

use std::path::PathBuf;

use glam::Vec2;
use xdgkit::desktop_entry::DesktopEntry;

use crate::{AppData, renderer::Rendering};

extern crate gl;

const ICON_PADDING_TOP: f32 = 20.0;
const ICON_PADDING_LEFT: f32 = 40.0;
const ICON_PADDING_RIGHT: f32 = 40.0;
const ICON_PADDING_BOTTOM: f32 = 35.0;

const PADDING: f32 = 40.0;

const UNKNOWN_ICON_PATH: &str = "assets/application-x-generic.png";

#[derive(Debug, Default)]
pub struct DesktopItem {
    pub data: DesktopEntry,
    pub size: f32,
    pub pos: Vec2,
    pub icon_texture: u32,
}

impl Rendering for DesktopItem {
    fn render(&self, state: &AppData) {
        state.app.desktop.icon_renderer.as_ref().unwrap().render(
            state,
            self.icon_texture,
            self.pos.x + PADDING,
            self.pos.y + PADDING,
            self.size - PADDING * 2.0,
            self.size - PADDING * 2.0,
        );
        // unsafe {
        //     gl::Enable(gl::SCISSOR_TEST);
        //     gl::Scissor(
        //         (self.pos.x + ICON_PADDING_LEFT) as i32,
        //         (state.app.desktop.height as f32 - 180.0 - self.pos.y + ICON_PADDING_BOTTOM) as i32,
        //         (self.size - ICON_PADDING_RIGHT - ICON_PADDING_LEFT) as i32,
        //         (self.size - ICON_PADDING_TOP - ICON_PADDING_BOTTOM) as i32,
        //     );
        //
        //     gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        //     gl::Clear(gl::COLOR_BUFFER_BIT);
        //     gl::Disable(gl::SCISSOR_TEST);
        // }
    }
}

pub fn get_desktop_entries() -> Vec<DesktopItem> {
    let home_dir = std::env::var("HOME").unwrap();
    let desktop_dir = format!("{home_dir}/Desktop");
    let desktop_files = std::fs::read_dir(desktop_dir).unwrap();

    let mut desktop_items = Vec::<DesktopItem>::new();

    let size = 140.0;

    for file in desktop_files.map(std::result::Result::unwrap) {
        let file_path = file.path();
        if file_path.is_file() && file_path.extension().unwrap() == "desktop" {
            let data = xdgkit::desktop_entry::DesktopEntry::new(file.path().into_string().unwrap());

            let icon_name = data.icon.as_ref().unwrap();

            let icon_path = xdgkit::icon_finder::find_icon(icon_name.to_owned(), 128, 1)
                .unwrap_or_else(|| PathBuf::from(icon_name))
                .into_string()
                .unwrap();

            let icon_texture = upload_icon_texture(&icon_path);

            let item = DesktopItem {
                data,
                size,
                icon_texture,
                ..Default::default()
            };
            desktop_items.push(item);
        }
    }
    desktop_items
}

pub fn upload_icon_texture(path: &str) -> u32 {
    let mut texture: u32 = 0;
    let img = image::open(path).unwrap_or_else(|_| image::open(UNKNOWN_ICON_PATH).unwrap());

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.to_rgba8().as_ptr().cast(),
        );
    }
    texture
}
