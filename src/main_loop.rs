use std::{io::Write, fs::File};

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
                    let mut req = client.get(str);

                    if str.contains("i.pximg.net") {
                        req = req.header("Referer", "https://www.pixiv.net/")
                    }

                    let response = match req.send() {
                        Ok(x) => x,
                        Err(e) => {
                            println!(
                                "{}\n{}",
                                e.to_string().red(),
                                "Sometimes, \"reqwest\"/\"hyper\" just randomly spit out error"
                                    .yellow()
                            );
                            continue;
                        }
                    };

                    if !response.status().is_success() {
                        println!(
                            "Status: {}\nData: {}",
                            response.status().as_str().red(),
                            response.text().unwrap()
                        );
                        continue;
                    }

                    let image_data = response.bytes().unwrap();

                    File::create(TEMP_FILE_NAME)
                        .unwrap()
                        .write_all(&image_data)
                        .unwrap();

                    File::open(TEMP_FILE_NAME).unwrap()
                }
                str if str.starts_with("file:") => {
                    let path_string = str.replace("file:", "");
                    let path = std::path::Path::new(&path_string);

                    if !path.exists() {
                        println!(
                            "{}{}{}",
                            "File (".red(),
                            path_string.cyan(),
                            ") not found!".red()
                        );
                        continue;
                    }

                    File::open(path).unwrap()
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