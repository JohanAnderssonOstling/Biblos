use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rbook::{Ebook, Epub};
use rbook::xml::{Element, Find};
use serde::{Deserialize, Serialize};
use xx_hash::xx_hash64;

use crate::library::{get_book_position, update_book_path, update_last_read};
use crate::settings::get_reader_style;

#[derive(Serialize, Deserialize)]
pub struct epub {
    title           : String,
    section_hrefs   : Vec<String>,
    contents        : Vec<String>,
    style_sheets    : Vec<String>,
    user_style      : HashMap<String, String>,
    hash            : String,
    toc             : Vec<TocElem>,
    section_index   : u32,
    elem_index      : u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct TocElem  {name: String,  value: String,  children: Vec<TocElem>, collapsed: bool, current: bool}
#[derive(Serialize, Deserialize)]
pub struct Book {title: String, path: String,   cover: String }

#[tauri::command]
pub fn get_book_content(book_path: &str, library_path: &str) -> epub{
    let epub        = Epub::new(book_path).unwrap();
    let title       = epub.metadata().title().unwrap().value().to_string();
    let contents    = epub.reader().iter().map(|x| x.unwrap().to_string()).collect();
    let toc         = epub.toc().elements().into_iter().map(convert_toc_element).collect();
    let user_style  = get_reader_style();
    let hash        = xx_hash64(&fs::read(Path::new(book_path)).unwrap()).to_string();
    let (section_index, elem_index) = get_book_position(library_path, &hash);

    let style_sheets = epub.manifest().all_by_media_type("text/css").into_iter()
        .map    (|elem| epub.read_file(elem.value()).unwrap()).collect();
    let section_hrefs = epub.spine().elements().iter()
        .map    (|elem| elem.attributes().get(0).unwrap().value())
        .map    (|value| epub.manifest().by_id(value).unwrap().value().split("/").last().unwrap().to_string())
        .collect();

    update_last_read(library_path, hash.to_string());
    update_book_path(library_path, &hash, book_path);
    epub {title, contents, style_sheets, user_style, hash, toc, section_hrefs, section_index, elem_index}
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
    let epub        = Epub::new(&path).unwrap();
    let title       = match epub.metadata().title() {
        None            => {path.split("/").last().unwrap().to_string()}
        Some(title)     => {title.value().to_string()}
    };
    let cover       = get_cover(&epub);
    Ok(Book {title, cover, path})
}

#[tauri::command]
pub fn write_book_position(lib_path: &str, hash: &str, section_index: u32, elem_index: u32) {
    let write_path  = format!("{}/.bookrium/positions/{}", lib_path, hash);
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

fn convert_toc_element(element: &Element) -> TocElem {
    TocElem {
        name: element.name().to_string(),
        value: element.value().to_string().split("/").last().unwrap().to_string(),
        children: element.children().into_iter()
            .map(|child| convert_toc_element(child)).collect(),
        collapsed: true,
        current: false,
    }
}

mod tests {
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use std::time::Instant;

    use rbook::{Ebook, Epub};
    use xx_hash::xx_hash64;
    use zip::ZipArchive;
    use crate::epub::get_cover;

    #[test]
    fn test_meta() {
        let time = Instant::now();
        let epub = Epub::new("/home/johan/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub").unwrap();
        //let author = epub.metadata().creators();
        let image = epub.cover_image().unwrap();
        let img = epub.read_bytes_file(image.value());
        
        let elapsed = time.elapsed();
        println!("{:#?}", elapsed);
    }
    #[test]
    fn test_zip_vs_hash() {
        let now = Instant::now();
        let zipfile = File::open("/home/johan/Hem/Programmering/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub").unwrap();
        let zip = ZipArchive::new(zipfile);
        
        let epub_time = now.elapsed();
        let now = Instant::now();
        let hash        = xx_hash64(&fs::read(Path::new("/home/johan/Hem/Programmering/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub")).unwrap()).to_string();
        let hash_time = now.elapsed();
        println!("{:#?}\t{:#?}", epub_time, hash_time);
    }

    #[test]
    fn test_cover() {
        let now = Instant::now();
        let epub = Epub::new("/home/johan/Hem/Programmering/RustroverProjects/Svelte-Epub/testdata/A Concise History of Switzerland.epub").unwrap();
        let epub_time = now.elapsed();
        let now = Instant::now();
        let cover = get_cover(&epub);
        let base64_time = now.elapsed();
        let now = Instant::now();
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
        let img_time = now.elapsed();
        println!("{:#?}\t{:#?}\t{:#?}", base64_time, img_time, epub_time);
    }

}