extern crate reqwest;
extern crate zip;

use core::panic;
use std::fs::File;

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
