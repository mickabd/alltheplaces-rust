extern crate reqwest;
extern crate zip;

use std::fs::File;

pub fn unzip(file_path: String, output_directory: String) {
    let file = File::open(&file_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive
        .extract(&output_directory)
        .expect(format!("couldn't extract {} to {}", file_path, output_directory).as_str());
    println!(
        "{} successfully extracted to {}",
        file_path, output_directory
    );
}
