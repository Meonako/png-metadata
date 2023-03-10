use std::{fs::File, io::Write};

use colored::Colorize;

use crate::utils::*;

mod download_file;
use download_file::download;

mod dir;
use dir::read_dir;

mod search;
use search::search_dir;

pub fn start(client: reqwest::blocking::Client, mut args: Vec<String>) {
    'main: loop {
        remove_temp_file();
        println!("-----------------------------------------------------------------");
        print!("{}", "Command: ".bright_magenta());
        std::io::stdout().flush().unwrap();

        let mut target = String::new();

        if args.is_empty() {
            std::io::stdin().read_line(&mut target).unwrap();

            if target.contains(',') {
                args = split_ignore_quotes(target);
                target = args.pop().unwrap().trim().to_string();
            }
        } else {
            target = args.pop().unwrap().trim().to_string();
            println!("{}", target);
        }

        println!("-----------------------------------------------------------------");

        let new_file = match target.trim() {
            t if t.starts_with("http") => {
                // Soft check for PNG from HTTP
                if !t.ends_with(".png") {
                    'confirm: loop {
                        print!(
                            "{} doesn't ends with {}. Do you want to proceed? (y/n)",
                            t.cyan(),
                            "\".png\"".bright_green()
                        );
                        std::io::stdout().flush().unwrap();
                        let mut confirm = String::new();
                        std::io::stdin().read_line(&mut confirm).unwrap();

                        match confirm.to_lowercase().trim() {
                            "y" | "yes" | "sure" => break 'confirm,
                            "n" | "no" | "stop" => continue 'main,
                            _ => continue 'confirm,
                        }
                    }
                }

                // Fugg idiomatic. I'm doing this my way.
                let file = download(&client, t);
                if file.is_none() {
                    continue 'main;
                }

                file.unwrap()
            }
            t if t.starts_with("all:") => {
                // Fugg idiomatic. I'm doing this my way.
                let file_list = search_dir(t);
                if file_list.is_none() {
                    continue 'main;
                }

                let file_list = file_list.unwrap();
                println!(
                    "{} {} {}",
                    "Found".green(),
                    file_list.len(),
                    "PNG image(s)".green()
                );
                args = file_list;
                continue 'main;
            }
            cmd => match cmd.to_lowercase().as_str() {
                "cls" | "clear" => {
                    print!("\x1Bc");
                    std::io::stdout().flush().unwrap();
                    continue 'main;
                }
                "quit" | "stop" | "exit" => {
                    remove_temp_file();
                    break 'main;
                }
                "update" => {
                    crate::update::check_for_update(&client);
                    continue 'main;
                }
                "help" => {
                    crate::print_help();
                    continue 'main;
                }
                _ => {
                    // Not receiving from match because it's lowercase. Not cool when display to the user.
                    let path = std::path::Path::new(cmd.trim_matches('"'));

                    if !path.exists() {
                        println!("{}{}{}", "\"".red(), cmd.cyan(), "\" not found!".red());
                        continue 'main;
                    }

                    if path.is_file() {
                        File::open(path).unwrap()
                    } else if path.is_dir() {
                        // Fugg idiomatic. I'm doing this my way.
                        let file_list = read_dir(path);
                        if file_list.is_none() {
                            continue 'main;
                        }

                        let file_list = file_list.unwrap();
                        println!(
                            "{} {} {}",
                            "Found".green(),
                            file_list.len(),
                            "PNG image(s)".green()
                        );
                        args = file_list;

                        continue 'main;
                    } else {
                        println!("Unknow path: {}", cmd.red());
                        continue 'main;
                    }
                }
            },
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
                println!("{}: {}", key.cyan(), value.green());
            }
        } else {
            println!("{}", "No Text Metadata found!".red());
        }
    }
}
