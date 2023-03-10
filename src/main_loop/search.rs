use std::fs::{read_dir as list_dir, ReadDir};

use colored::Colorize;

pub fn search_dir(file_name: &str) -> Option<Vec<String>> {
    let file_name = file_name.replace("all:", "");
    let path_string = file_name.trim().trim_matches('"');

    let path = std::path::Path::new(path_string);

    if !path.exists() {
        println!(
            "{}{}{}",
            "Directory (\"".red(),
            path_string.cyan(),
            "\") not found!".red()
        );
        return None;
    }

    if !path.is_dir() {
        println!("\'{}\' {}", path_string.cyan(), "is not a directory!".red());
        None
    } else {
        let files_list = match list_dir(path) {
            Ok(entries) => filter_png_and_read_subfolder(entries),
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
            );
            None
        } else {
            Some(files_list.into_iter().rev().collect())
        }
    }
}

fn filter_png_and_read_subfolder(entries: ReadDir) -> Vec<String> {
    let mut png_list = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let entry_full_path = entry.path();

        if entry_full_path.is_file() {
            let file_name = entry.path().to_string_lossy().to_string();

            if file_name.ends_with(".png") {
                png_list.push(file_name)
            }
        } else if entry_full_path.is_dir() {
            let result = search_dir(&entry_full_path.to_string_lossy().to_string());

            if let Some(res) = result {
                png_list.extend(res.into_iter().rev());
            }
        }
    }

    png_list
}
