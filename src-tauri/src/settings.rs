use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;

const SETTINGS_PATH: &str = "/home/johan/.local/share/bookrium/settings";
const EPUB_SETTINGS_PATH: &str = "/home/johan/.local/share/bookrium/settings/epub-style.txt";

#[tauri::command]
pub fn get_reader_style() -> HashMap<String, String> {
    let css_path = format!{"{SETTINGS_PATH}/epub-style.txt"};
    println!("{css_path}");
    OpenOptions::new().create(true).append(true).open(&css_path).unwrap();
    let settings = fs::read_to_string(css_path).unwrap();
    let style_settings = settings.lines();
    let mut settings_map = HashMap::new();
    for style_setting in style_settings {
        let parts: Vec<&str> = style_setting.trim().split(":").collect();
        settings_map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
    }
    settings_map

}

#[tauri::command]
pub async fn set_setting(setting: String, value: String) {
    let mut settings = get_reader_style();
    settings.insert(setting, value);
    set_settings(settings);
}

fn set_settings(settings: HashMap<String, String>) {
    let mut settings_output = String::new();
    for (key, value) in settings {
        settings_output = format!("{settings_output}{key}\t:\t{value}\n");
    }
    fs::write(EPUB_SETTINGS_PATH, settings_output).unwrap();
}

#[tauri::command]
pub fn set_reader_style(css: &str) {
    let css_path = format!{"{SETTINGS_PATH}/epub_reader.css"};
    let file = OpenOptions::new().create(true).append(true).open(&css_path).unwrap();
    fs::write(css_path, css).unwrap()
}

