use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::epub::write_book_position;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dir { path: String, book_paths: Vec<String>}

#[tauri::command]
pub fn get_library(path: &str) -> (Vec<String>, Vec<Dir>) {
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
pub fn get_dirs(path: &str) -> Vec<Dir>{
    fs::read_dir(path).unwrap()
        .map    (|res|  res.unwrap().path())
        .filter (|path| path.is_dir() && !path.to_str().unwrap().contains("."))
        .map    (|path| path.to_str().unwrap().to_string())
        .map    (|path| Dir {book_paths: get_books(&path), path})
        .collect()
}

#[tauri::command]
pub fn search_books(path: &str) -> Vec<String> {
    todo!()
}

pub fn get_book_position(library_path: &str, hash: &str) -> (u32, u32) {
    let pos_dir     = format!("{library_path}/.bookrium/positions");
    let pos_path    = format!("{pos_dir}/{hash}");
    fs::create_dir_all(&pos_dir).unwrap();
    match fs::read_to_string(pos_path.clone()) {
        Ok(value)   => { value.split(":").map(|pos| pos.to_string().parse::<u32>().unwrap()).next_tuple().unwrap() }
        Err(_)      => {
            File::create(pos_path).unwrap();
            write_book_position(library_path, hash, 0, 0);
            (0,0)
        }
    }
}

pub fn update_book_path(library_path: &str, hash: &str, book_path: &str) {
    let path_dir    = format!("{library_path}/.bookrium/book_paths");
    let write_path  = format!("{path_dir}/{hash}.txt");
    fs::create_dir_all(&path_dir).unwrap();
    fs::write(write_path, book_path).unwrap();
}

pub fn update_last_read(library_path: &str, hash: String) {
    let last_read_path  = format!("{library_path}/.bookrium/last_read.txt");
    let reader          = BufReader::new(File::open(&last_read_path).unwrap());
    let mut hashes: Vec<String> = reader.lines()
        .map(|e| e.unwrap()).take(20)
        .filter(|pos| pos != &hash).collect();
    hashes.insert(0, hash);
    fs::write(last_read_path, hashes.join("\n")).unwrap();
}