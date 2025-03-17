pub mod db;
pub mod download;
pub mod files;
pub mod model;
pub mod poi;
pub mod unzip;

extern crate dotenv;

use db::{get_client, ingest_into_db, truncate_table};
use dotenv::dotenv;
use download::download_atp_data;
use log::debug;
use poi::extract_features;
use std::env;
use unzip::unzip;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("Logger Initialized");
    debug!("Reading environment variables");
    dotenv().ok();
    let host = env::var("DBHOST")?;
    let user = env::var("DBUSER")?;
    let password = env::var("DBPASSWORD")?;
    let port = env::var("DBPORT")?;
    let dbname = env::var("DBNAME")?;
    debug!("Creating db client");
    let mut client = get_client(host, user, password, port, dbname);

    let output_path = String::from("temp/output.zip");
    let unzip_directory = String::from("temp/");
    let files_directory = String::from("temp/output/");

    download_atp_data(&output_path);
    unzip(output_path, unzip_directory);
    truncate_table(&mut client)?;
    for entry in WalkDir::new(files_directory)
        .max_depth(1)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| e.path().is_file())
    {
        let display = entry.path().display().to_string();
        let pois = extract_features(entry);
        match pois {
            Some(value) => ingest_into_db(&mut client, value).unwrap(),
            None => continue,
        };
        println!("File {} successfuly ingested", display);
    }
    Ok(())
}
