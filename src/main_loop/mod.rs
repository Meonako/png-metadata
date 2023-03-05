use std::io::Write;

use colored::Colorize;

use crate::utils::*;

mod download_file;
use download_file::download;

mod local_file;
use local_file::open_file;

mod dir;
use dir::read_dir;

pub fn start(client: reqwest::blocking::Client, mut args: Vec<String>) {
    loop {
        println!("-----------------------------------------------------------------");
        print!("{}", "Command: ".bright_magenta());
        std::io::stdout().flush().unwrap();

        let mut target = String::new();

        if args.is_empty() {
            std::io::stdin().read_line(&mut target).unwrap();

            if target.contains(',') {
                args = target.rsplit(',').map(|s| s.to_string()).collect();
                target = args.pop().unwrap().trim().to_string()
            }
        } else {
            target = args.pop().unwrap().trim().to_string();
            println!("{}", target)
        }

        println!("-----------------------------------------------------------------");

        let new_file = match target.trim() {
            t if t.starts_with("http") => {
                if let Some(file) = download(&client, t) {
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
            t if t.starts_with("dir:") => {
                if let Some(entries) = read_dir(t) {
                    println!("{} {} {}", "Found".green(), entries.len(), "PNG image(s)".green());
                    args = entries;
                    continue;
                } else {
                    continue;
                }
            }
            cmd if cmd.to_lowercase().as_str() == "cls"
                || cmd.to_lowercase().as_str() == "clear" =>
            {
                print!("\x1Bc");
                std::io::stdout().flush().unwrap();
                continue;
            }
            cmd if cmd.to_lowercase().as_str() == "quit"
                || cmd.to_lowercase().as_str() == "stop" =>
            {
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
