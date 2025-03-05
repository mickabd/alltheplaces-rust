pub mod files;

use files::{download_atp_data, get_file_url, remove_not_usable_files, unzip};

fn main() {
    let file_url = get_file_url();
    let output_path = String::from("temp/output.zip");
    let output_directory = String::from("temp/");
    let files_directory = String::from("temp/output/");

    match download_atp_data(file_url, &output_path) {
        Err(why) => panic!("error downloading the atp zip file: {}", why),
        Ok(_) => (),
    };
    unzip(output_path, output_directory);
    remove_not_usable_files(files_directory);
}
