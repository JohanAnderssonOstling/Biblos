use std::fs;

#[tauri::command]
pub fn get_library(path: &str) -> (Vec<String>, Vec<String>) {
    (get_books(path), get_dirs(path))
}

#[tauri::command]
pub fn get_books(path: &str) -> Vec<String>{
    fs::read_dir(path).unwrap()
        .map    (|res|  res.unwrap().path().to_str().unwrap().to_string())
        .filter (|path| path.ends_with(".epub"))
        .collect()
}

#[tauri::command]
pub fn get_dirs(path: &str) ->Vec<String>{
    fs::read_dir(path).unwrap()
        .map    (|res|  res.unwrap().path())
        .filter (|path| path.is_dir() && !path.to_str().unwrap().contains("."))
        .map    (|path| path.to_str().unwrap().to_string())
        .collect()
}



