pub mod db;
pub mod download;
pub mod files;
pub mod model;
pub mod poi;
pub mod unzip;

use db::{get_client, ingest_brand_into_db, ingest_poi_into_db, truncate_table};
use dotenv::dotenv;
use download::{download_atp_data, get_file_url};
use log::{debug, info};
use poi::extract_features;
use std::env;
use unzip::unzip;
use walkdir::WalkDir;

const ATP_BASE_URL: &str = "https://data.alltheplaces.xyz/runs/latest/info_embed.html";

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

    let url = get_file_url(ATP_BASE_URL);
    download_atp_data(&output_path, &url)?;
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
            Some(value) => {
                let brand_id = ingest_brand_into_db(&mut client, value.brand).unwrap();
                ingest_poi_into_db(&mut client, value.pois, brand_id).unwrap();
            }
            None => continue,
        };
        info!("File {} successfuly ingested", display);
    }
    Ok(())
}
