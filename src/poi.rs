extern crate geojson;
use crate::model::{Address, Coordinates, Feature, POI};
use geojson::{GeoJson, JsonValue};
use std::{error::Error, fs::read_to_string};
use walkdir::{DirEntry, WalkDir};

pub fn extract_features_from_files(input_path: &String, output_path: &String) {
    for entry in WalkDir::new(input_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| e.path().is_file())
    {
        let display = entry.path().display().to_string();
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
        let pois = build_pois(content);
    }
}

fn is_file_empty(entry: &DirEntry) -> bool {
    let display = entry.path().display().to_string();
    let metadata = match entry.metadata() {
        Err(why) => {
            println!("error reading metadata for {}, {}", display, why);
            return true;
        }
        Ok(value) => value,
    };
    let is_empty = metadata.is_file() && metadata.len() == 0;
    if is_empty {
        println!("{} is empty.", display);
    }
    is_empty
}

fn read_geojson(entry: &DirEntry) -> Result<GeoJson, Box<dyn Error>> {
    let display = entry.path().display().to_string();
    println!("trying to parse {} into a geojson", display);
    let string_value = match read_to_string(&display) {
        Err(why) => return Err(why.into()),
        Ok(value) => value,
    };
    match string_value.parse::<GeoJson>() {
        Err(why) => return Err(why.into()),
        Ok(value) => {
            println!("successfully parsed {} into a geojson", display);
            Ok(value)
        }
    }
}

fn build_pois(content: JsonValue) -> Option<Vec<POI>> {
    let mut pois: Vec<POI> = vec![];
    // this will either assign the value or stop the function and returns None
    let features = match content["features"].as_array() {
        Some(value) => value,
        None => return None,
    };

    for feature in features {
        let poi = match build_poi(feature) {
            Some(value) => value,
            None => {
                println!("file data is incomplete");
                return None;
            }
        };
        pois.push(poi)
    }
    Some(pois)
}

fn build_poi(feature: &JsonValue) -> Option<POI> {
    let feature: Feature = match serde_json::from_str(&feature.to_string()) {
        Err(why) => {
            println!("error parsing the feature {}", why);
            return None;
        }
        Ok(value) => value,
    };
    let coordinates = parse_coordinates(&feature);
    let (name, brand) = parse_names(&feature);
    let address = parse_geographical_information(&feature);
    Some(POI {
        spider_id: "value".to_string(),
    })
}

fn parse_coordinates(feature: &Feature) -> Coordinates {
    Coordinates {
        longitude: &feature.geometry.coordinates[0],
        latitude: &feature.geometry.coordinates[1],
    }
}

fn parse_names(feature: &Feature) -> (&String, &String) {
    let brand = &feature.properties.brand;
    let name = &feature.properties.name;
    match name {
        Some(value) => return (value, brand),
        None => return (brand, brand),
    }
}

fn parse_geographical_information(feature: &Feature) -> Address {
    let city = &feature.properties.address_city;
    let zipcode = &feature.properties.address_postcode;
    let house_number = &feature.properties.address_housenumber;
    let street_address = &feature.properties.address_street;
    let country = &feature.properties.address_country;
    let state = &feature.properties.address_state;
    let full_address = &feature.properties.address_full;
    let street_name = &feature.properties.address_street;

    Address {
        full_address,
        house_number,
        street_name,
        street_address,
        zipcode,
        city,
        state,
        country,
    }
}
