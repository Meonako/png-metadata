use std::{
    fs::{read_dir as list_dir, ReadDir},
    path::Path,
};

use colored::Colorize;

pub fn read_dir(path: &Path) -> Option<Vec<String>> {
    let files_list = match list_dir(path) {
        Ok(entries) => filter_png(entries),
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
            path.to_string_lossy().cyan(),
            "directory!".red()
        );
        None
    } else {
        Some(files_list.into_iter().rev().collect())
    }
}

fn filter_png(entries: ReadDir) -> Vec<String> {
    let mut png_list = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.path().to_string_lossy().to_string();

        if file_name.ends_with(".png") {
            png_list.push(file_name)
        }
    }

    png_list
}
