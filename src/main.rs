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

    print_help();

    main_loop::start(client, argument);
}

pub fn print_help() {
    println!("\
        {} will read from a directory if is dir\n\
        {} will read from a file if is file\n\
        Multiple Commands: {} will work like 2 above in order \n\
        {} will ignore comma (,) in file name\n\
        {} will download the image from the URL and read it\n\
        ------------------------------\n\
        \"{}\" / \"{}\" to clear terminal screen\n\
        ------------------------------\n\
        \"{}\" to check for update\n\
        ------------------------------\n\
        \"{}\" / \"{}\" / \"{}\" to exit",

        "images/example".yellow(),
        "My Image/background.png".yellow(),
        "images/example, My Image/background.png".yellow(),
        "\"C:/Pictures/my pic, family.png\"".yellow(),
        "https://mydomain.dom/img/example.png".yellow(),

        "cls".bright_blue(), "clear".bright_blue(),

        "update".bright_blue(),

        "quit".bright_blue(), "stop".bright_blue(), "exit".bright_blue(),
    );
}