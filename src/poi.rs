extern crate country_boundaries;
extern crate geojson;
extern crate lazy_static;
extern crate url;

use crate::files::{is_file_empty, read_geojson};
use crate::model::{Feature, POI};
use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use geo::Point;
use geojson::JsonValue;
use lazy_static::lazy_static;
use std::path::Display;
use url::Url;
use walkdir::DirEntry;

lazy_static! {
    static ref BOUNDARIES: CountryBoundaries =
        CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)
            .expect("error while initializing the country boundaries");
}

pub fn extract_features(input_path: DirEntry) -> Option<Vec<POI>> {
    let display = input_path.path().display();
    if is_file_empty(&input_path) {
        println!("the file {} is empty, skipping it", display);
        return None;
    }
    let content = match read_geojson(&input_path) {
        Err(why) => {
            println!(
                "the file {} is broken, skipping it. Error is: {}",
                display, why
            );
            return None;
        }
        Ok(value) => value.to_json_value(),
    };
    Some(build_pois(content, &display))
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
    let point = parse_coordinates(&feature);
    let country_code = reverse_geocode(&feature);
    // let phone_number = parsed_phone_number(&feature, &country_code);

    Some(POI {
        poi_name,
        brand: feature.properties.brand,
        website,
        brand_wikidata_id: feature.properties.brand_wikidata_id,
        spider_id: feature.properties.spider_id,
        opening_hours: feature.properties.opening_hours,
        phone: feature.properties.phone,
        full_address: feature.properties.address_full,
        house_number: feature.properties.address_housenumber,
        street_name: feature.properties.address_street,
        street_address: feature.properties.address_street_address,
        city: feature.properties.address_city,
        zipcode: feature.properties.address_postcode,
        state: feature.properties.address_state,
        country: feature.properties.address_country,
        country_code,
        point,
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
            }
        }
    }
    None
}

fn parse_coordinates(feature: &Feature) -> Option<Point> {
    match &feature.geometry {
        Some(value) => return Some(Point::new(value.coordinates[0], value.coordinates[1])),
        None => None,
    }
}

fn reverse_geocode(feature: &Feature) -> Option<String> {
    let (longitude, latitude) = match &feature.geometry {
        Some(value) => (value.coordinates[0], value.coordinates[1]),
        None => return None,
    };
    let latlong = match LatLon::new(latitude, longitude) {
        Err(why) => {
            println!("error parsing the lat/long for feature: {:#?}", feature);
            println!("error: {}", why);
            return None;
        }
        Ok(value) => value,
    };
    let ids = BOUNDARIES.ids(latlong);
    // We get the last one to get the biggest one.
    match ids.last() {
        None => {
            println!("the latlong was not mapped to a country :O");
            return None;
        }
        Some(value) => Some(value.to_string()),
    }
}
