// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::State;
use crate::epub::{get_book_content, get_book_cover, get_images, write_book_position};
use crate::home::{create_libraries, delete_library, get_last_read_books, get_libraries};
use crate::library::{get_books, get_dirs, get_library};

mod epub;
mod home;
mod library;// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_libraries, get_libraries, delete_library,
            get_library, get_books, get_dirs, 
            get_book_content, get_images, get_book_cover, write_book_position])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
