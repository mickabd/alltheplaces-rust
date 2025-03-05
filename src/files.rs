extern crate reqwest;
extern crate zip;

use core::panic;
use std::error::Error;
use std::fs::{read_to_string, remove_file, File};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use geojson::GeoJson;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

pub fn get_file_url() -> String {
    let atp_base_url = String::from("https://data.alltheplaces.xyz/runs/latest/info_embed.html");
    let request = match reqwest::blocking::get(&atp_base_url) {
        Err(why) => panic!("not able to reach the url {}, {}", atp_base_url, why),
        Ok(value) => value,
    };
    let body = match request.text() {
        Err(why) => panic!("not able to parse the body of the request, {}", why),
        Ok(value) => value,
    };
    // unwrap is ok because it's a harcoded value
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']").unwrap();
    let captures = match re.captures(&body) {
        Some(captures) => captures,
        None => panic!("could not find the latest URL download link!"),
    };
    // The first captures is always the full string containing the match.
    let url = match captures.get(1) {
        Some(url) => url.as_str(),
        None => panic!("capture group not found"),
    };
    url.to_string()
}

pub fn download_atp_data(url: String, output_path: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&output_path);
    let mut file = File::create(path)?;
    println!("Getting the zip file from {}", url);
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(120, 0)))
        .build()?;
    let req = client.get(&url).build()?;
    let resp = client.execute(req)?.bytes()?;
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

fn is_file_empty(entry: &DirEntry) -> bool {
    let display = entry.path().display().to_string();
    let metadata = match entry.metadata() {
        Err(why) => {
            println!("error reading metadata for {}, {}", display, why);
            return true;
        }
        Ok(value) => value,
    };
    let is_empty = metadata.is_file() && metadata.len() == 0;
    if is_empty {
        println!("{} is empty.", display);
    }
    is_empty
}

fn is_file_broken(entry: &DirEntry) -> bool {
    let display = entry.path().display().to_string();
    println!("trying to parse {} into a geojson", display);
    let string_value = match read_to_string(&display) {
        Err(why) => {
            println!("error reading {}, {}", display, why);
            return true;
        }
        Ok(value) => value,
    };
    match string_value.parse::<GeoJson>() {
        Err(why) => {
            println!("error parsing {}, {}", display, why);
            true
        }
        Ok(_) => {
            println!("successfully parsed {} into a geojson", display);
            false
        }
    }
}

pub fn remove_not_usable_files(directory: String) {
    for entry in WalkDir::new(directory)
        .max_depth(1)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| e.path().is_file() && (is_file_empty(e) || is_file_broken(e)))
    {
        let display = entry.path().display().to_string();
        println!("starting deletion of {}.", display);
        match remove_file(&display) {
            Err(why) => panic!("not able to delete the file: {}", why),
            Ok(_) => (),
        };
        println!("{} has been deleted.", display);
    }
}
