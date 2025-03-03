use core::panic;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use regex::Regex;
use walkdir::WalkDir;

pub fn get_file_url() -> Result<String, Box<dyn Error>> {
    let atp_base_url = String::from("https://data.alltheplaces.xyz/runs/latest/info_embed.html");
    let body = reqwest::blocking::get(atp_base_url)?.text()?;
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']")?;
    let captures = match re.captures(&body) {
        Some(captures) => captures,
        None => return Err("could not find the latest URL download link!".into()),
    };
    // The first captures is always the full string containing the match.
    let url = match captures.get(1) {
        Some(url) => url.as_str(),
        None => return Err("capture group not found".into()),
    };
    Ok(url.to_string())
}

pub fn download_atp_data(url: String, output_path: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&output_path);
    let mut file = File::create(path)?;
    println!("Getting the zip file from {}", url);
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(120, 0)))
        .build()?;
    let req = client.get(&url).build().unwrap();
    let resp = client.execute(req).unwrap().bytes().unwrap();
    println!("Got file from {}", url);
    file.write_all(&resp)?;
    Ok(())
}

pub fn unzip(file_path: String, output_directory: String) {
    let file = File::open(&file_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    match archive.extract(&output_directory) {
        Err(why) => {
            panic!(
                "couldn't extract {} to {}, error: {}",
                file_path, output_directory, why
            )
        }
        Ok(_) => println!(
            "{} successfully extracted to {}",
            file_path, output_directory
        ),
    };
}

pub fn get_valid_files(directory: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<String> = vec![];
    for entry in WalkDir::new(directory).into_iter().filter_map(|f| f.ok()) {
        if entry.metadata().unwrap().is_file() && entry.metadata().unwrap().len() > 0 {
            files.push(entry.path().display().to_string());
        }
    }
    if files.len() == 0 {
        return Err("No files found".into());
    }
    files.sort();
    Ok(files)
}
