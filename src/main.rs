use std::{fs::File, io::Write};

use colored::Colorize;
use reqwest::blocking::Client;

fn main() {
    let client = Client::builder()
        .user_agent("PNG Meta Grabber")
        .build()
        .expect("build reqwest client");

    println!(
        "Path example:\n\t{}\n\t{}",
        "file:C:/Image/background.png".yellow(),
        "https://i.pximg.net/img-original/img/2023/03/04/00/42/34/105888900_p1.png".yellow()
    );

    loop {
        print!("Command: ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        println!("-----------------------------------------------------------------");

        let new_file = match user_input.to_lowercase().trim() {
            str if str.starts_with("http") => {
                let mut req = client.get(str);

                if str.contains("i.pximg.net") {
                    req = req.header("Referer", "https://www.pixiv.net/")
                }

                let response = match req.send() {
                    Ok(x) => x,
                    Err(e) => {
                        println!("{}\n{}", e.to_string().red(), "Sometimes, \"reqwest\"/\"hyper\" just randomly spit out error"
                                .yellow());
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

                File::create("temp.img.png")
                    .unwrap()
                    .write_all(&image_data)
                    .unwrap();

                File::open("temp.img.png").unwrap()
            }
            str if str.starts_with("file:") => {
                let path_string = str.replace("file:", "");
                let path = std::path::Path::new(&path_string);

                if !path.exists() {
                    println!("{}{}{}", "File (".red(), path_string.cyan(), ") not found!".red());
                    continue;
                }

                File::open(path).unwrap()
            },
            "quit" | "stop" => {
                let temp_file = std::path::Path::new("temp.img.png");

                if temp_file.exists() {
                    std::fs::remove_file(temp_file).expect("delete file");
                }

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
                    "If ".yellow(),
                    "\"Invalid PNG signature\"".red(),
                    ", mostly because the image is not in PNG format".yellow()
                );
                println!("-----------------------------------------------------------------");
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

        println!("-----------------------------------------------------------------");
    }
}

fn get_avaiable_text(info: &png::Info) -> Vec<(String, String)> {
    let mut result = Vec::new();

    if !info.compressed_latin1_text.is_empty() {
        for text in &info.compressed_latin1_text {
            result.push((text.keyword.clone(), text.get_text().unwrap()))
        }
    }

    if !info.uncompressed_latin1_text.is_empty() {
        for text in &info.uncompressed_latin1_text {
            result.push((text.keyword.clone(), text.text.clone()))
        }
    }

    if !info.utf8_text.is_empty() {
        for text in &info.utf8_text {
            result.push((text.keyword.clone(), text.get_text().unwrap()))
        }
    }

    result
}
