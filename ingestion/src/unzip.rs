use std::fs::File;

use log::info;

pub fn unzip(file_path: String, output_directory: String) {
    let file = File::open(&file_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive
        .extract(&output_directory)
        .unwrap_or_else(|_| panic!("couldn't extract {} to {}", file_path, output_directory));
    info!(
        "{} successfully extracted to {}",
        file_path, output_directory
    );
}
