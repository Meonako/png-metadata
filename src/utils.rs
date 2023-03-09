use crate::TEMP_FILE_NAME;

pub fn remove_temp_file() {
    let temp_file = std::path::Path::new(TEMP_FILE_NAME);

    if temp_file.exists() {
        std::fs::remove_file(temp_file).expect("delete file");
    }
}

pub fn get_avaiable_text(info: &png::Info) -> Vec<(String, String)> {
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

pub fn apply_required_header(
    req: reqwest::blocking::RequestBuilder,
    url: &str,
) -> reqwest::blocking::RequestBuilder {
    if url.contains("i.pximg.net") {
        req.header("referer", "https://www.pixiv.net/")
    } else {
        req
    }
}

// Split function from ChatGPT xD
pub fn split_ignore_quotes<T: ToString>(s: T) -> Vec<String> {
    let s = s.to_string();

    let mut result = Vec::new();
    let mut inside_quotes = false;
    let mut start = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '"' {
            inside_quotes = !inside_quotes;
        } else if c == ',' && inside_quotes {
            continue;
        } else if i == s.len() - 1 {
            result.push(s[start..=i].to_string());
        } else if c == ',' {
            result.push(s[start..i].to_string());
            start = i + 1;
        }
    }

    result
}
