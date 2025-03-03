pub mod files;

extern crate reqwest;
extern crate zip;

use files::{download_atp_data, get_file_url, get_valid_files, unzip};

fn main() {
    let file_url = get_file_url().unwrap();
    let output_path = String::from("temp/output.zip");
    let output_directory = String::from("temp/");
    download_atp_data(file_url, &output_path);
    unzip(output_path, output_directory);
    let valid_files = get_valid_files(String::from("temp/output/"));
    println!("{}", valid_files.len());
}
