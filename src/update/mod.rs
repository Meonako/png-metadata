use colored::Colorize;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION:      &str = env!("CARGO_PKG_VERSION");

const LATEST_VERSION_URL:   &str = "https://github.com/Meonako/png-metadata/releases/latest";
const VERSION_URL_PREFIX:   &str = "https://github.com/Meonako/png-metadata/releases/tag/v";
const DOWNLOAD_URL_PREFIX:  &str = "https://github.com/Meonako/png-metadata/releases/latest/download";

pub fn check_for_update(cli: &reqwest::blocking::Client) {
    let response = match cli
        .get(LATEST_VERSION_URL)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            println!("{}\n{}", "Cannot check for new version.".red(), e);
            return;
        }
    };

    let url = response.url().to_string();

    if url.starts_with(VERSION_URL_PREFIX) {
        let version = url.replace(VERSION_URL_PREFIX, "");

        if version != VERSION {
            println!(
                "There is a new version!\n\
                \tCurrent Version: {}\n\
                \tLatest Version: {}\n\
                Download: {}",
                VERSION.red(),
                version.bright_green(),
                format!("{DOWNLOAD_URL_PREFIX}/{PACKAGE_NAME}.exe").cyan()
            );
        } else {
            println!("Already Running Latest version ({})", VERSION.bright_green());
        }
    } else {
        println!("{} {}", "Unknown version received:".red(), url.cyan())
    }
}
