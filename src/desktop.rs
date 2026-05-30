use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct DesktopEntry {
    label: String,
    icon_path: String,
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
