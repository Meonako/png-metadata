use std::fs::read_dir as list_dir;

use colored::Colorize;

pub fn read_dir(file_name: &str) -> Option<Vec<String>> {
    let mut path_string = file_name.replace("dir:", "");

    if path_string.starts_with("\"") && path_string.ends_with("\"") {
        // Remove first and last double quote
        // I don't know which one is more efficient, so...
        //
        // path_string = path_string[1..path_string.len() - 1].to_string()
        //
        path_string = path_string.replace("\"", "")
    }

    let path = std::path::Path::new(&path_string);

    if !path.is_dir() {
        println!("\'{}\' {}", path_string.cyan(), "is not a directory!".red());
        return None;
    }

    if !path.exists() {
        println!(
            "{}{}{}",
            "Directory (\"".red(),
            path_string.cyan(),
            "\") not found!".red()
        );
        None
    } else {
        let files_list = match list_dir(path) {
            Ok(entries) => {
                let mut png_list = Vec::new();

                for entry in entries {
                    let entry = entry.unwrap();
                    let file_name = entry.file_name().to_string_lossy().to_string();

                    if file_name.ends_with(".png") {
                        png_list.push(format!("file:{}{}{}", path_string, if path_string.contains('/') { '/' } else { '\\' }, file_name))
                    }
                }

                png_list
            }
            Err(e) => {
                println!(
                    "{}\n{}",
                    e,
                    "Mostly because you lack permission to view content inside directory".yellow()
                );
                return None;
            }
        };

        if files_list.is_empty() {
            println!(
                "{} \'.png\' {} {} {}",
                "No".red(),
                "found in".red(),
                path_string.cyan(),
                "directory!".red()
            )
        }

        Some(files_list.into_iter().rev().collect())
    }
}