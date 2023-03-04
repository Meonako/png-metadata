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

        let mut user_input = String::new();

        if args.is_empty() {
            std::io::stdin().read_line(&mut user_input).unwrap();
        } else {
            user_input = args.pop().unwrap();
            println!("{}", user_input)
        }

        println!("-----------------------------------------------------------------");

        let new_file = 
            match user_input.to_lowercase().trim() {
                str if str.starts_with("http") => {
                    if let Some(file) = download_file(&client, str) {
                        file
                    } else {
                        continue;
                    }
                }
                str if str.starts_with("file:") => {
                    if let Some(file) = open_file(str) {
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
                    "If \"".yellow(),
                    "Invalid PNG signature".red(),
                    "\", mostly because the image is not in PNG format".yellow()
                );
                continue;
            }
        };
        let png_info = reader.info();

        let data = get_avaiable_text(png_info);

        if !data.is_empty() {
            for (key, value) in data {
                println!("{}: {}", key.cyan(), value.green())
            }
        } else {
            println!("{}", "Not found anything!".red())
        }

        remove_temp_file();
    }
}

fn download_file(client: &reqwest::blocking::Client, url: &str) -> Option<File> {
    let mut req = client.get(url);

    if url.contains("i.pximg.net") {
        req = req.header("Referer", "https://www.pixiv.net/")
    }

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
            "Status: {}\nData: {}",
            response.status().as_str().red(),
            response.text().unwrap()
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
        path_string = path_string[1..path_string.len() - 1].to_string()
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