use regex::Regex;
use std::error::Error;
extern crate reqwest;

fn main() {
    let file_url = get_file_url().unwrap();
    println!("{}", file_url)
}

fn get_file_url() -> Result<String, Box<dyn Error>> {
    let atp_base_url = String::from("https://data.alltheplaces.xyz/runs/latest/info_embed.html");
    let body = reqwest::blocking::get(atp_base_url)?.text()?;
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']").unwrap();
    let captures = match re.captures(&body) {
        Some(captures) => captures,
        None => return Err("Could not find the latest URL download link!".into()),
    };
    // The first captures is always the full string containing the match.
    let url = match captures.get(1) {
        Some(url) => url.as_str(),
        None => return Err("Capture group not found".into()),
    };
    Ok(url.to_string())
}
