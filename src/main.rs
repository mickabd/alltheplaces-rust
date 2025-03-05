pub mod files;

use files::{download_atp_data, get_file_url, remove_not_usable_files, unzip};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_url = get_file_url().unwrap();
    let output_path = String::from("temp/output.zip");
    let output_directory = String::from("temp/");
    let files_directory = String::from("temp/output/");

    download_atp_data(file_url, &output_path)?;
    unzip(output_path, output_directory);
    remove_not_usable_files(files_directory);
    Ok(())
}
