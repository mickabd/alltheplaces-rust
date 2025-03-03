pub mod files;

extern crate geojson;
extern crate reqwest;
extern crate zip;

use files::{download_atp_data, get_file_url, get_valid_files, unzip};
use geojson::GeoJson;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_url = get_file_url().unwrap();
    let output_path = String::from("temp/output.zip");
    let output_directory = String::from("temp/");

    download_atp_data(file_url, &output_path)?;
    unzip(output_path, output_directory);

    let valid_files = get_valid_files(String::from("temp/output/"))?;
    println!("{}", valid_files.len());

    let first_file = valid_files.get(0).unwrap(); // It's safe to unwrap because
    let contents = fs::read_to_string(first_file)?.parse::<GeoJson>()?;
    println!("{}", contents.to_string());
    Ok(())
}
