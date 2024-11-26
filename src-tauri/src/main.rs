#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use crate::epub::{get_book_content, get_book_cover, get_images, write_book_position};
use crate::home::{create_libraries, delete_library, get_last_read_books, get_libraries};
use crate::library::{get_books, get_dirs, get_library};
use crate::settings::{get_reader_style, set_reader_style, set_setting};
mod epub;
mod home;
mod library;
mod settings; // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command



fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            create_libraries, get_libraries, delete_library,
            get_library, get_books, get_dirs, 
            get_book_content, get_images, get_book_cover, write_book_position,
            get_reader_style, set_reader_style, set_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}
*/