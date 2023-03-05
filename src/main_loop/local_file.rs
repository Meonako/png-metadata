use std::fs::File;

use colored::Colorize;

pub fn open_file(file_name: &str) -> Option<File> {
    let mut path_string = file_name.replace("file:", "");

    if path_string.starts_with("\"") && path_string.ends_with("\"") {
        // Remove first and last double quote
        // I don't know which one is more efficient, so...
        //
        // path_string = path_string[1..path_string.len() - 1].to_string()
        //
        path_string = path_string.replace("\"", "")
    }

    let path = std::path::Path::new(&path_string);

    if !path.is_file() {
        println!(
            "{} {}",
            path_string.cyan(),
            "is not a file!"
        );
        return None;
    }

    if !path.exists() {
        println!(
            "{}{}{}",
            "File (".red(),
            path_string.cyan(),
            ") not found!".red()
        );
        None
    } else {
        Some(File::open(path).unwrap())
    }
}