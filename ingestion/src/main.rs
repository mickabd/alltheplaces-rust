pub mod db;
pub mod download;
pub mod files;
pub mod model;
pub mod poi;
pub mod unzip;

use db::{get_client, ingest_brand_into_db, ingest_poi_into_db, truncate_table};
use download::{download_atp_data, get_file_url};
use log::{debug, info};
use poi::extract_features;
use std::env;
use unzip::unzip;
use walkdir::WalkDir;

const ATP_BASE_URL: &str = "https://data.alltheplaces.xyz/runs/latest/info_embed.html";
const POSTGRES_POI_DB_URL: &str = "postgres://admin:example@localhost:5432/poi";
const POSTGRES_BRAND_DB_URL: &str = "postgres://admin:example@localhost:5433/brand";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    debug!("Logger Initialized");
    debug!("Reading environment variables");
    debug!("Creating db client");
    let mut client_poi = get_client(POSTGRES_POI_DB_URL);
    let mut client_brand = get_client(POSTGRES_BRAND_DB_URL);

    let output_path = String::from("temp/output.zip");
    let unzip_directory = String::from("temp/");
    let files_directory = String::from("temp/output/");

    let url = get_file_url(ATP_BASE_URL);
    download_atp_data(&output_path, &url)?;
    unzip(output_path, unzip_directory);

    truncate_table(&mut client_poi, "poi")?;
    truncate_table(&mut client_brand, "brand")?;

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
                let brand_id = ingest_brand_into_db(&mut client_brand, value.brand).unwrap();
                ingest_poi_into_db(&mut client_poi, value.pois, brand_id).unwrap();
            }
            None => continue,
        };
        info!("File {} successfuly ingested", display);
    }
    Ok(())
}
