// use std::io::{BufRead, BufReader};

use glam::Vec2;
use xdgkit::desktop_entry::DesktopEntry;

use crate::{AppData, renderer::Rendering};

extern crate gl;

const ICON_PADDING_TOP: f32 = 20.0;
const ICON_PADDING_LEFT: f32 = 40.0;
const ICON_PADDING_RIGHT: f32 = 40.0;
const ICON_PADDING_BOTTOM: f32 = 35.0;

#[derive(Debug, Default)]
pub struct DesktopItemOld {
    pub label: String,
    pub icon_path: String,
    pub exec: Option<String>,
    pub size: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default)]
pub struct DesktopItem {
    pub data: DesktopEntry,
    pub size: f32,
    pub pos: Vec2,
}

impl Rendering for DesktopItem {
    fn render(&self, state: &AppData) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(
                (self.pos.x + ICON_PADDING_LEFT) as i32,
                (state.app.desktop.height as f32 - 180.0 - self.pos.y + ICON_PADDING_BOTTOM) as i32,
                (self.size - ICON_PADDING_RIGHT - ICON_PADDING_LEFT) as i32,
                (self.size - ICON_PADDING_TOP - ICON_PADDING_BOTTOM) as i32,
            );

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::SCISSOR_TEST);
        }
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
            let ent = xdgkit::desktop_entry::DesktopEntry::new(file.path().into_string().unwrap());
            let item = DesktopItem {
                data: ent,
                size,
                ..Default::default()
            };
            desktop_items.push(item);
        }
        //     let f = std::fs::File::open(file_path).unwrap();
        //     let buf = BufReader::new(f);
        //
        //     let mut icon_path: Option<String> = None;
        //     let mut exec: Option<String> = None;
        //
        //     for line in buf.lines() {
        //         let line = line.unwrap();
        //         if line.starts_with("Icon=") && icon_path.is_none() {
        //             icon_path = Some(line.strip_prefix("Icon=").unwrap().to_string());
        //         } else if line.starts_with("Exec=") && exec.is_none() {
        //             let mut exec_match = line.strip_prefix("Exec=").unwrap().to_string();
        //             if exec_match.starts_with("\"") && exec_match.ends_with("\"") {
        //                 exec_match = exec_match.strip_prefix("\"").unwrap().to_string();
        //             }
        //             exec = Some(exec_match);
        //         }
        //     }
        //
        //     if icon_path.is_none() {
        //         continue;
        //     }
        //
        //     let item = DesktopItem {
        //         label: file.file_name().into_string().unwrap(),
        //         icon_path: icon_path.unwrap(),
        //         exec,
        //         size,
        //         x: 0,
        //         y: 0,
        //     };
        //    desktop_items.push(item);
    }
    desktop_items
}
