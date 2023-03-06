use clap::Parser;
use colored::Colorize;
use reqwest::blocking::Client;

mod structs;
use structs::Arguments;

mod utils;
mod main_loop;

mod update;

const TEMP_FILE_NAME: &str = "temp.img.png";

fn main() {
    let client = 
        Client::builder()
            .user_agent("PNG Meta Grabber")
            .build()
            .expect("build reqwest client");

    let argument = 
        Arguments::parse()
            .path
            .into_iter()
            .rev()
            .collect::<Vec<String>>();

    println!(
        "Path example: \n\
        \t{}\n \
        \t{}\n \
        \t{}\n \
        \t{}\n \
        \"{}\" / \"{}\" to exit",
        "dir:images/example".yellow(),
        "file:My Image/background.png".yellow(),
        "file:\"C:/Pictures/my pic.png\"".yellow(),
        "https://i.pximg.net/img-original/img/2023/03/04/00/42/34/105888900_p1.png".yellow(),
        "quit".bright_blue(),
        "stop".bright_blue()
    );

    main_loop::start(client, argument);
}
