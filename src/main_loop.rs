use std::io::Write;
use std::fs::File;

use colored::Colorize;

use crate::TEMP_FILE_NAME;
use crate::utils::*;

pub fn start(client: reqwest::blocking::Client, mut args: Vec<String>) {
    loop {
        println!("-----------------------------------------------------------------");
        print!("{}", "Command: ".bright_magenta());
        std::io::stdout().flush().unwrap();

        let mut target = String::new();

        if args.is_empty() {
            std::io::stdin().read_line(&mut target).unwrap();

            if target.contains(',') {
                args = target.rsplit(',')
                    .map(|s| s.to_string())
                    .collect();
                target = args.pop().unwrap().trim().to_string()
            }
        } else {
            target = args.pop().unwrap().trim().to_string();
            println!("{}", target)
        }

        println!("-----------------------------------------------------------------");

        let new_file = 
            match target.to_lowercase().trim() {
                t if t.starts_with("http") => {
                    if let Some(file) = download_file(&client, t) {
                        file
                    } else {
                        continue;
                    }
                }
                t if t.starts_with("file:") => {
                    if let Some(file) = open_file(t) {
                        file
                    } else {
                        continue;
                    }
                }
                "quit" | "stop" => {
                    remove_temp_file();
                    break;
                }
                _ => {
                    println!("{}", "Unknown command".red());
                    continue;
                }
            };

        let decoder = png::Decoder::new(new_file);
        let reader = match decoder.read_info() {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "{}\n{}{}{}",
                    e.to_string().red(),
                    "Note: If \"".yellow(),
                    "Invalid PNG signature".red(),
                    "\", mostly because the image is not in PNG format".yellow()
                );
                continue;
            }
        };
        let png_info = reader.info();

        let all_text = get_avaiable_text(png_info);

        if !all_text.is_empty() {
            for (key, value) in all_text {
                println!("{}: {}", key.cyan(), value.green())
            }
        } else {
            println!("{}", "No Text Metadata found!".red())
        }

        remove_temp_file();
    }
}

fn download_file(client: &reqwest::blocking::Client, url: &str) -> Option<File> {
    let mut req = client.get(url);

    req = apply_required_header(req, url);

    let response = match req.send() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "{}\n{}",
                e.to_string().red(),
                "Sometimes, \"reqwest\"/\"hyper\" just randomly spit out error".yellow()
            );
            return None;
        }
    };

    if !response.status().is_success() {
        println!(
            "Status: {0}\nData: {2}\nHeaders: {1:#?}",
            response.status().to_string().red(),
            response.headers().clone(),
            response.text().unwrap(),
        );
        return None;
    }

    let image_data = response.bytes().unwrap();

    File::create(TEMP_FILE_NAME)
        .unwrap()
        .write_all(&image_data)
        .unwrap();

    Some(File::open(TEMP_FILE_NAME).unwrap())
}

fn open_file(file_name: &str) -> Option<File> {
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