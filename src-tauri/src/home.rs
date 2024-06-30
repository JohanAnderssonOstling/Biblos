use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;
use csv::{Reader, WriterBuilder};
use serde::{Deserialize, Serialize};

const CSV_PATH: &str = "/home/johan/.local/share/bookrium/home.csv";
#[derive(Serialize, Deserialize)]
pub struct Library { path: String, book_paths: Vec<String> }

#[tauri::command]
pub fn create_libraries(paths: Vec<String>) {
    let file = OpenOptions::new().create(true).append(true).open(CSV_PATH).unwrap();
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
    writer.serialize(paths).unwrap();
    writer.flush().unwrap();
}

#[tauri::command]
pub fn delete_library(library_path: &str) {
    let delete_path     = format!("{library_path}/.bookrium");
    let mut reader      = Reader::from_path(CSV_PATH).unwrap();
    let libs: Vec<String> = reader.deserialize().map(|result| result.unwrap()).collect();
    let mut writer      = WriterBuilder::new().has_headers(true).from_writer(OpenOptions::new()
        .create(true).truncate(true).write(true).open(CSV_PATH).unwrap());
    
    writer.serialize("path").unwrap();
    for library in libs {
        if library_path == library  { fs::remove_dir_all(&delete_path).unwrap() }
        else                        {writer.serialize(library).unwrap()}
    }
    writer.flush().unwrap()
}

#[tauri::command]
pub fn get_libraries() -> Vec<Library> {
    Reader::from_path(CSV_PATH).unwrap().deserialize()
        .map    (|res: Result<String, _>| res.unwrap())
        .filter (|path| Path::new(path).exists())
        .map    (|path| Library {path: path.clone(), book_paths: get_last_read_books(&path)})
        .collect()
}

pub fn get_last_read_books(library_path: &str) -> Vec<String>{
    let lib_path = format!("{library_path}/.bookrium");
    let last_read_path      = format!("{lib_path}/last_read.txt");
    fs::create_dir_all(&lib_path).unwrap_or_default();
    let file = OpenOptions::new().read(true).write(true).create(true).open(last_read_path).unwrap();
    BufReader::new(file).lines()
        .map        (|hash| format!("{library_path}/.bookrium/book_paths/{}.txt", hash.unwrap()))
        .filter     (|path| Path::new(path).exists())
        .filter_map (|path| get_book(path, &library_path))
        .collect()
}

fn get_book(path: String, library_path: &str) -> Option<String> {
    let book_path = fs::read_to_string(&path).unwrap();
    if Path::new(&book_path).exists() {return Some(book_path) }
    let book_name = book_path.split("/").last().unwrap();
    for entry in WalkDir::new(library_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == book_name {
            return Some(path.display().to_string());
        }
    }
    None
}