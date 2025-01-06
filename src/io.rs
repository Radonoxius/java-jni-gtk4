/*use std::fs::OpenOptions;
use std::io::{Read, Write};

pub(crate) fn write_to_file(path: &str, content: &str) {
    let file = OpenOptions::new().write(true).open(path);
    if let Err(_) = file {
        eprintln!("Oops! File not found at {}!", path);
    }
    else {
        file.unwrap().write_all(content.as_bytes()).unwrap();
    }
}

pub(crate) fn get_file_content(path: &str) -> String {
    let file = OpenOptions::new().read(true).open(path);
    let mut content = String::new();
    if let Err(_) = file {
        eprintln!("Oops! File not found at {}!", path);
    }
    else {
        file.unwrap().read_to_string(&mut content).unwrap();
    }
    content
}*/
