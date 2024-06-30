use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rbook::{Ebook, Epub};
use serde::{Deserialize, Serialize};
use xx_hash::xx_hash64;
use std::io::{self, Write};
use rbook::xml::Element;  // Ensure the Write trait is in scope
#[derive(Serialize, Deserialize)]
pub struct epub {
    section_hrefs: Vec<String>,
    contents: Vec<String>,
    style_sheets: Vec<(String, String)>,
    hash: String,
    toc: Vec<TocElem>,
    section_index: u32,
    elem_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TocElem  {name: String,  value: String,  children: Vec<TocElem>, collapsed: bool}
#[derive(Serialize, Deserialize)]
pub struct Book {title: String, path: String,   cover: String }

#[tauri::command]
pub fn get_book_content(book_path: &str, library_path: &str) -> epub{
    let epub        = Epub::new(book_path).unwrap();
    let reader      = epub.reader();
    let contents    = reader.iter().map(|x| x.unwrap().to_string()).collect();
    let style_sheets = get_stylesheets(&epub);
    let hash        = xx_hash64(&fs::read(Path::new(book_path)).unwrap()).to_string();
    let toc         = get_toc(&epub);
    let section_hrefs = get_spine(&epub);
    let pos_dir     = format!("{library_path}/.bookrium/positions");
    let pos_path    = format!("{pos_dir}/{hash}");
    fs::create_dir_all(&pos_dir).unwrap();
    let position    = match fs::read_to_string(pos_path.clone()) {
        Ok(value)   => { value.split(":").map(|pos| pos.to_string().parse::<u32>().unwrap()).collect() }
        Err(_)      => { File::create(pos_path).unwrap();
            write_book_position(library_path, hash.clone(), 0, 0);
            vec![0,0]
        }
    };
    update_last_read(library_path, hash.to_string());



    let path_dir    = format!("{library_path}/.bookrium/book_paths");
    let write_path  = format!("{path_dir}/{hash}.txt");
    fs::create_dir_all(&path_dir).unwrap();
    fs::write(write_path, book_path).unwrap();
    
    epub {contents, style_sheets, hash, toc, section_hrefs, section_index: position[0], elem_index: position[1]}
}

fn get_stylesheets(epub: &Epub) -> Vec<(String, String)> {
    epub.manifest().all_by_media_type("text/css").into_iter()
        .map(|elem| (elem.value().to_string(), epub.read_file(elem.value()).unwrap())).collect()
}

pub fn update_last_read(library_path: &str, hash: String) {
    let last_read_path  = format!("{library_path}/.bookrium/last_read.txt");
    let reader          = BufReader::new(File::open(&last_read_path).unwrap());
    let mut hashes: Vec<String> = reader.lines().map(|e| e.unwrap()).take(20)
        .filter(|pos| pos != &hash).collect();
    hashes.insert(0, hash);
    fs::write(last_read_path, hashes.join("\n")).unwrap();
}

#[tauri::command]
pub async fn get_images(book_path: String) -> Result<Vec<(String, String)>, ()>{
    let epub        = Epub::new(book_path).unwrap();
    let mut images: Vec<(String,String)> = Vec::with_capacity(epub.manifest().images().len());
    for img_element in epub.manifest().images() {
        let img_bytes   = epub.read_bytes_file(img_element.value()).unwrap();
        let img_base64  = format!("data:image/jpeg;base64,{}", BASE64_STANDARD.encode(img_bytes.to_vec()));
        images.push((img_element.value().to_string(), img_base64));
    }
    Ok(images)
}

#[tauri::command]
pub async fn get_book_cover(path: String) -> Result<Book, String> {
    let epub = Epub::new(&path).unwrap();
    let title = match epub.metadata().title() {
        None => {path.split("/").last().unwrap().to_string()}
        Some(title) => {title.value().to_string()}
    };
    let cover = get_cover(&epub);
    Ok(Book {title, cover, path})
}

#[tauri::command]
pub fn write_book_position(lib_path: &str, hash: String, section_index: u32, elem_index: u32) {
    let write_path  = format!("{}/.bookrium/positions/{}", lib_path, hash.to_string());
    let position    = format!("{}:{}", section_index, elem_index);
    fs::write(write_path, position).unwrap()
}

fn get_cover(epub:&Epub) -> String{
    let cover_img = match epub.cover_image() {
        Some(image) => {epub.read_bytes_file(image.value()).unwrap_or_default()}
        None        => {
            let mut cover_img = Vec::new();
            for img_element in epub.manifest().images() {
                if img_element.name().to_lowercase().contains("cover") || img_element.value().to_lowercase().contains("cover") {
                    cover_img =  epub.read_bytes_file(img_element.value()).unwrap_or_default();
                }
            }
            cover_img
        }
    };
    return format!("data:image/jpeg;base64,{}", BASE64_STANDARD.encode(cover_img.to_vec()));
}

fn get_toc(epub: &Epub) -> Vec<TocElem>{
    let toc = epub.toc();
    toc.elements().into_iter().map(convert_toc_element).collect()
}

fn get_spine(epub: &Epub) -> Vec<String> {
    let spine = epub.spine().elements();
    let mut hrefs = Vec::new();
    for element in spine {
        let value = element.attributes().get(0).unwrap().value();
        let href = epub.manifest().by_id(value).unwrap().value().split("/").last().unwrap();
        hrefs.push(href.to_string())
    }
    hrefs
}

fn convert_toc_element(element: &Element) -> TocElem {
    TocElem {
        name: element.name().to_string(),
        value: element.value().to_string().split("/").last().unwrap().to_string(),
        children: element.children().into_iter()
            .map(|child| convert_toc_element(child)).collect(),
        collapsed: true
    }
}

mod tests {
    use rbook::{Ebook, Epub};
    use crate::epub::{get_spine, get_stylesheets, get_toc};

    #[test]
    fn test_toc(){
        let epub = Epub::new("/home/johan/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub").unwrap();
        let toc = get_toc(&epub);
        //println!("{toc}");
    }
    
    #[test]
    fn test_spine() {
        let epub = Epub::new("/home/johan/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub").unwrap();
        let spine = get_spine(&epub);
        println!("{:#?}", spine)
    }

    #[test]
    fn test_stylesheet() {
        let epub = Epub::new("/home/johan/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub").unwrap();
        let stylesheets = get_stylesheets(&epub);
        for style in stylesheets {
            println!("{}\n{}", style.0, style.1);
        }
    }
}