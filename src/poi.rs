extern crate geojson;
extern crate url;

use crate::files::{is_file_empty, read_geojson, write_to_csv};
use crate::model::{Feature, POI};
use geojson::JsonValue;
use std::path::Display;
use url::Url;
use walkdir::WalkDir;

pub fn extract_features_from_files(input_path: &str, output_path: &str) {
    for entry in WalkDir::new(input_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| e.path().is_file())
    {
        let display = entry.path().display();
        if is_file_empty(&entry) {
            println!("the file {} is empty, skipping it", display);
            continue;
        }
        let content: JsonValue = match read_geojson(&entry) {
            Err(why) => {
                println!(
                    "the file {} is broken, skipping it. Error is: {}",
                    display, why
                );
                continue;
            }
            Ok(value) => value.to_json_value(),
        };
        let pois = build_pois(content, &display);
        let file_name = entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0];

        let output_file = format!("{}/{}.csv", output_path, file_name);
        write_to_csv(pois, &output_file)
            .expect(format!("error while writing to {}", output_file).as_str());
    }
}

fn build_pois(content: JsonValue, file_path: &Display) -> Vec<POI> {
    let mut pois: Vec<POI> = vec![];
    // this will either assign the value or stop the function and returns None
    let features = content["features"]
        .as_array()
        .expect(format!("error when parsing the content for the file {}", file_path).as_str());

    for feature in features {
        let poi = match build_poi(feature) {
            Some(value) => value,
            None => {
                println!("file data is incomplete");
                continue;
            }
        };
        pois.push(poi);
    }
    pois
}

fn build_poi(feature: &JsonValue) -> Option<POI> {
    let feature: Feature = match serde_json::from_str(&feature.to_string()) {
        Err(why) => {
            println!("error parsing the feature {}", why);
            return None;
        }
        Ok(value) => value,
    };
    let poi_name = parse_names(&feature);
    let website = parse_url(&feature);
    let (longitude, latitude) = parse_coordinates(&feature);

    Some(POI {
        poi_name,
        brand: feature.properties.brand,
        website,
        brand_wikidata_id: feature.properties.brand_wikidata_id,
        spider_id: feature.properties.spider_id,
        opening_hours: feature.properties.opening_hours,
        full_address: feature.properties.address_full,
        house_number: feature.properties.address_housenumber,
        street_name: feature.properties.address_street,
        street_address: feature.properties.address_street_address,
        city: feature.properties.address_city,
        zipcode: feature.properties.address_postcode,
        state: feature.properties.address_state,
        country: feature.properties.address_country,
        longitude,
        latitude,
    })
}

fn parse_names(feature: &Feature) -> Option<String> {
    let brand = feature.properties.brand.clone();
    let name = feature.properties.name.clone();
    match name {
        Some(name) => return Some(name),
        None => match brand {
            Some(value) => Some(value),
            None => None,
        },
    }
}

fn parse_url(feature: &Feature) -> Option<String> {
    // Try website first, then fall back to source_uri
    let urls_to_try = [
        feature.properties.website.as_ref(),
        Some(&feature.properties.source_uri),
    ];

    for url_opt in urls_to_try {
        if let Some(url_str) = url_opt {
            if let Ok(parsed_url) = Url::parse(url_str) {
                if let Some(host) = parsed_url.host_str() {
                    return Some(host.to_string());
                }
            } else {
                println!("Error parsing URL {}", url_str);
            }
        }
    }
    None
}

fn parse_coordinates(feature: &Feature) -> (Option<f64>, Option<f64>) {
    match &feature.geometry {
        Some(value) => return (Some(value.coordinates[0]), Some(value.coordinates[1])),
        None => return (None, None),
    };
}
