use std::io::{BufRead, BufReader};

extern crate gl;

const ICON_PADDING_TOP: i32 = 20;
const ICON_PADDING_LEFT: i32 = 40;
const ICON_PADDING_RIGHT: i32 = 40;
const ICON_PADDING_BOTTOM: i32 = 35;

#[derive(Debug)]
pub struct DesktopEntry {
    label: String,
    icon_path: String,
}

impl DesktopEntry {
    pub fn draw_area_rect(&self, x: i32, y: i32, size: i32) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(
                x + ICON_PADDING_LEFT,
                y + ICON_PADDING_BOTTOM,
                size - ICON_PADDING_RIGHT - ICON_PADDING_LEFT,
                size - ICON_PADDING_TOP - ICON_PADDING_BOTTOM,
            );

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }
}

pub fn get_desktop_entries() -> Vec<DesktopEntry> {
    let home_dir = std::env::var("HOME").unwrap();
    let desktop_dir = home_dir + "/Desktop";
    let desktop_files = std::fs::read_dir(desktop_dir).unwrap();

    let mut entries = Vec::<DesktopEntry>::new();

    for file in desktop_files.map(std::result::Result::unwrap) {
        let file_path = file.path();
        if file_path.is_file() {
            println!("inside {:?}", file_path);
            let f = std::fs::File::open(file_path).unwrap();
            let buf = BufReader::new(f);

            for line in buf.lines() {
                let line = line.unwrap();
                if line.starts_with("Icon=") {
                    let entry = DesktopEntry {
                        label: file.file_name().into_string().unwrap(),
                        icon_path: line[5..].to_string(),
                    };
                    entries.push(entry);
                }
            }
        }
    }

    println!("{:?}", entries);
    entries
}
