pub mod download;
pub mod files;
pub mod model;
pub mod poi;
pub mod unzip;
use download::download_atp_data;
use poi::extract_features_from_files;
use unzip::unzip;

fn main() {
    let output_path = String::from("temp/output.zip");
    let unzip_directory = String::from("temp/");
    let files_directory = String::from("temp/output/");
    let output_directory = String::from("temp/curated");
    download_atp_data(&output_path);
    unzip(output_path, unzip_directory);
    extract_features_from_files(&files_directory, &output_directory);
}
