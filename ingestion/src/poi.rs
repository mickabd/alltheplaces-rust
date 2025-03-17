use crate::files::{is_file_empty, read_geojson};
use crate::model::{Feature, Geometry, POI};
use country_boundaries::{BOUNDARIES_ODBL_360X180, CountryBoundaries, LatLon};
use geo::Point;
use geojson::JsonValue;
use lazy_static::lazy_static;
use log::warn;
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
            warn!(
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
            warn!("error parsing the feature {}", why);
            return None;
        }
        Ok(value) => value,
    };
    let poi_name = parse_poi_name(&feature.properties.brand, &feature.properties.name);
    let website = parse_url(
        &feature.properties.website,
        &Some(feature.properties.source_uri.clone()),
    );
    let point = parse_coordinates(&feature.geometry);
    let country_code = match reverse_geocode(&point) {
        Some(value) => value,
        None => return None,
    };

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

fn parse_poi_name(brand: &Option<String>, name: &Option<String>) -> Option<String> {
    match name {
        Some(name) => return Some(name.clone()),
        None => match brand {
            Some(value) => Some(value.clone()),
            None => None,
        },
    }
}

fn parse_url(website: &Option<String>, source_uri: &Option<String>) -> Option<String> {
    // Try website first, then fall back to source_uri
    let urls_to_try = [website, source_uri];

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

fn parse_coordinates(geometry: &Option<Geometry>) -> Option<Point> {
    match geometry {
        Some(value) => return Some(Point::new(value.coordinates[0], value.coordinates[1])),
        None => None,
    }
}

fn reverse_geocode(point: &Option<Point>) -> Option<String> {
    let (longitude, latitude) = match point {
        Some(value) => (value.x(), value.y()),
        None => return None,
    };
    let latlong = match LatLon::new(latitude, longitude) {
        Err(why) => {
            warn!("error parsing the lat/long for Point: {:#?}", point);
            warn!("error: {}", why);
            return None;
        }
        Ok(value) => value,
    };
    let ids = BOUNDARIES.ids(latlong);
    // We get the last one to get the biggest one.
    match ids.last() {
        None => return None,
        Some(value) => Some(value.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_parse_poi_name_with_name() {
        let result = parse_poi_name(&Some(String::from("mickael")), &Some(String::from("jules")));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("jules"))
    }

    #[test]
    fn test_parse_poi_name_without_name() {
        let result = parse_poi_name(&Some(String::from("mickael")), &None);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("mickael"))
    }

    #[test]
    fn test_parse_poi_name_without_none() {
        let result = parse_poi_name(&None, &None);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_url_with_website() {
        let result = parse_url(
            &Some(String::from("https://doc.rust-lang.org/")),
            &Some(String::from("https://calendar.google.com/calendar/")),
        );
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("doc.rust-lang.org"));
    }

    #[test]
    fn test_parse_url_with_wrong_website() {
        let result = parse_url(
            &Some(String::from("..doc.rust-lang.org/")),
            &Some(String::from("https://calendar.google.com/calendar/")),
        );
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("calendar.google.com"));
    }

    #[test]
    fn test_parse_url_with_none() {
        let result = parse_url(&None, &None);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_coordinates() {
        let result = parse_coordinates(&Some(Geometry {
            r#type: "Point".to_string(),
            coordinates: [-1.0, 1.0],
        }));
        assert!(result.is_some());
        assert_eq!(result.unwrap().x(), -1.0);
        assert_eq!(result.unwrap().y(), 1.0);
    }

    #[test]
    fn test_parse_coordinates_none() {
        let result = parse_coordinates(&None);
        assert!(result.is_none());
    }

    #[test]
    fn test_reverse_geocode_fr() {
        let result = reverse_geocode(&Some(Point::new(2.3276581, 48.8805374)));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("FR"));
    }

    #[test]
    fn test_reverse_geocode_en() {
        let result = reverse_geocode(&Some(Point::new(-0.14405508452768728, 51.4893335)));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("GB"));
    }

    #[test]
    fn test_reverse_geocode_us() {
        let result = reverse_geocode(&Some(Point::new(-74.0060152, 40.7127281)));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("US"));
    }

    #[test]
    fn test_reverse_geocode_none() {
        let result = reverse_geocode(&None);
        assert!(result.is_none());
    }

    #[test]
    fn test_reverse_geocode_water() {
        let result = reverse_geocode(&Some(Point::new(3.864293, 54.375721)));
        assert!(result.is_none());
    }

    #[test]
    fn test_build_poi_valid_feature() {
        let feature = serde_json::json!({
            "id": "uuid",
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [-74.0060152, 40.7127281]
            },
            "properties": {
                "name": "Test POI",
                "brand": "Test Brand",
                "website": "http://example.com",
                "@source_uri": "http://example.com",
                "@brand:wikidata": "Q12345",
                "@spider": "spider_1",
                "opening_hours": "24/7",
                "phone": "+123456789",
                "addr:full": "123 Test St, Test City, Test Country",
                "addr:housenumber": "123",
                "addr:street": "Test St",
                "addr:street_address": "123 Test St",
                "addr:city": "Test City",
                "addr:postcode": "12345",
                "addr:state": "Test State",
                "addr:country": "Test Country"
            }
        });

        let result = build_poi(&feature);
        assert!(result.is_some());
        let poi = result.unwrap();
        assert_eq!(poi.poi_name, Some("Test POI".to_string()));
        assert_eq!(poi.brand, Some("Test Brand".to_string()));
        assert_eq!(poi.website, Some("example.com".to_string()));
        assert_eq!(poi.brand_wikidata_id, Some("Q12345".to_string()));
        assert_eq!(poi.spider_id, "spider_1".to_string());
        assert_eq!(poi.opening_hours, Some("24/7".to_string()));
        assert_eq!(poi.phone, Some("+123456789".to_string()));
        assert_eq!(
            poi.full_address,
            Some("123 Test St, Test City, Test Country".to_string())
        );
        assert_eq!(poi.house_number, Some("123".to_string()));
        assert_eq!(poi.street_name, Some("Test St".to_string()));
        assert_eq!(poi.street_address, Some("123 Test St".to_string()));
        assert_eq!(poi.city, Some("Test City".to_string()));
        assert_eq!(poi.zipcode, Some("12345".to_string()));
        assert_eq!(poi.state, Some("Test State".to_string()));
        assert_eq!(poi.country, Some("Test Country".to_string()));
        assert_eq!(poi.country_code, "US".to_string());
        assert_eq!(poi.point, Some(Point::new(-74.0060152, 40.7127281)));
    }

    #[test]
    fn test_build_poi_invalid_feature() {
        let feature = serde_json::json!({
            "id": "uuid",
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [5, 0]
            },
            "properties": {
                "name": "Test POI",
                "brand": "Test Brand",
                "website": "http://example.com",
                "@source_uri": "http://example.com",
                "@brand:wikidata": "Q12345",
                "@spider": "spider_1",
                "opening_hours": "24/7",
                "phone": "+123456789",
                "addr:full": "123 Test St, Test City, Test Country",
                "addr:housenumber": "123",
                "addr:street": "Test St",
                "addr:street_address": "123 Test St",
                "addr:city": "Test City",
                "addr:postcode": "12345",
                "addr:state": "Test State",
                "addr:country": "Test Country"
            }
        });

        let result = build_poi(&feature);
        assert!(result.is_none());
    }

    #[test]
    fn test_build_poi_missing_properties() {
        let feature = serde_json::json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [102.0, 0.5]
            },
            "properties": {}
        });

        let result = build_poi(&feature);
        assert!(result.is_none());
    }

    #[test]
    fn test_build_poi_missing_geometry() {
        let feature = json!({
            "id": "uuid",
            "type": "Feature",
            "properties": {
                "name": "Test POI",
                "brand": "Test Brand",
                "website": "http://example.com",
                "@source_uri": "http://example.com",
                "@brand:wikidata": "Q12345",
                "@spider": "spider_1",
                "opening_hours": "24/7",
                "phone": "+123456789",
                "addr:full": "123 Test St, Test City, Test Country",
                "addr:housenumber": "123",
                "addr:street": "Test St",
                "addr:street_address": "123 Test St",
                "addr:city": "Test City",
                "addr:postcode": "12345",
                "addr:state": "Test State",
                "addr:country": "Test Country"
            }
        });

        let result = build_poi(&feature);
        assert!(result.is_none());
    }
}
