use std::fs::File;
use std::io::Write;

use colored::Colorize;

use crate::TEMP_FILE_NAME;
use crate::utils::apply_required_header;

pub fn download(client: &reqwest::blocking::Client, url: &str) -> Option<File> {
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