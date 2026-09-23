use std::io::{BufRead, BufReader};

use crate::{AppData, renderer::Rendering};

extern crate gl;

const ICON_PADDING_TOP: i32 = 20;
const ICON_PADDING_LEFT: i32 = 40;
const ICON_PADDING_RIGHT: i32 = 40;
const ICON_PADDING_BOTTOM: i32 = 35;

#[derive(Debug, Default)]
pub struct DesktopItem {
    pub label: String,
    pub icon_path: String,
    pub exec: Option<String>,
    pub size: i32,
    pub x: i32,
    pub y: i32,
}

impl Rendering for DesktopItem {
    fn render(&self, state: &AppData) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(
                self.x + ICON_PADDING_LEFT,
                state.app.desktop.height as i32 - 180 - self.y + ICON_PADDING_BOTTOM,
                self.size - ICON_PADDING_RIGHT - ICON_PADDING_LEFT,
                self.size - ICON_PADDING_TOP - ICON_PADDING_BOTTOM,
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

    let size = 140;

    for file in desktop_files.map(std::result::Result::unwrap) {
        let file_path = file.path();
        if file_path.is_file() && file_path.extension().unwrap() == "desktop" {
            let f = std::fs::File::open(file_path).unwrap();
            let buf = BufReader::new(f);

            let mut icon_path: Option<String> = None;
            let mut exec: Option<String> = None;

            for line in buf.lines() {
                let line = line.unwrap();
                // TODO:
                // if line.starts_with("[Desktop Entry]") {
                //
                // }
                if line.starts_with("Icon=") && icon_path.is_none() {
                    icon_path = Some(line.strip_prefix("Icon=").unwrap().to_string());
                } else if line.starts_with("Exec=") && exec.is_none() {
                    let mut exec_match = line.strip_prefix("Exec=").unwrap().to_string();
                    if exec_match.starts_with("\"") && exec_match.ends_with("\"") {
                        exec_match = exec_match.strip_prefix("\"").unwrap().to_string();
                    }
                    exec = Some(exec_match);
                }
            }

            if icon_path.is_none() {
                continue;
            }

            let item = DesktopItem {
                label: file.file_name().into_string().unwrap(),
                icon_path: icon_path.unwrap(),
                exec,
                size,
                x: 0,
                y: 0,
            };
            desktop_items.push(item);
        }
    }

    desktop_items
}
