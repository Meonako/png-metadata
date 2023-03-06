use colored::Colorize;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_update(cli: &reqwest::blocking::Client) {
    let response = match cli
        .get("https://github.com/Meonako/png-metadata/releases/latest")
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            println!("{}\n{}", "Cannot check for new version.".red(), e);
            return;
        }
    };

    let url = response.url().to_string();

    if url.starts_with("https://github.com/Meonako/png-metadata/releases/tag/v") {
        let version = url.replace("https://github.com/Meonako/png-metadata/releases/tag/v", "");

        if version != VERSION {
            println!(
                "There is a new version!\n\
                \tCurrent Version: {}\n\
                \tLatest Version: {}\n\
                Download: {}",
                VERSION.red(),
                version.bright_green(),
                format!("https://github.com/Meonako/png-metadata/releases/latest/download/{}.exe", PACKAGE_NAME).cyan()
            );
        } else {
            println!("Already Running Latest version ({})", VERSION.bright_green());
        }
    } else {
        println!("{} {}", "Unknown version received:".red(), url.cyan())
    }
}
