use std::path::PathBuf;

pub fn get_system_fonts(path: Option<String>) -> Vec<PathBuf> {
    let fonts_dir = std::fs::read_dir(path.unwrap_or("/usr/share/fonts/".to_string())).unwrap();

    let mut fonts: Vec<PathBuf> = vec![];

    for file in fonts_dir {
        let file = file.unwrap();
        if file.path().is_dir() {
            fonts.extend(get_system_fonts(
                file.path().to_str().map(|s| s.to_string()),
            ));
        } else {
            fonts.push(file.path());
        }
    }

    fonts
}

pub fn find_font_path(query: &String) -> Option<PathBuf> {
    let fonts = get_system_fonts(None);
    fonts.into_iter().find(|f| {
        f.ends_with(query)
            || f.file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s == query)
    })
}
