pub mod db;
pub mod download;
pub mod files;
pub mod model;
pub mod poi;
pub mod unzip;

extern crate dotenv;

use db::{get_client, to_db};
use dotenv::dotenv;
use download::download_atp_data;
use poi::extract_features_from_files;
use std::env;
use unzip::unzip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let host = env::var("DBHOST")?;
    let user = env::var("DBUSER")?;
    let password = env::var("DBPASSWORD")?;
    let port = env::var("DBPORT")?;
    let dbname = env::var("DBNAME")?;

    let output_path = String::from("temp/output.zip");
    let unzip_directory = String::from("temp/");
    let files_directory = String::from("temp/output/");
    let output_directory = String::from("temp/curated");
    download_atp_data(&output_path);
    unzip(output_path, unzip_directory);
    extract_features_from_files(&files_directory, &output_directory);

    let client = get_client(host, user, password, port, dbname);
    let _ = to_db(client);
    Ok(())
}
