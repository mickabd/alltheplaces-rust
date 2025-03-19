use geojson::GeoJson;
use log::error;
use std::error::Error;
use std::fs::{self, File};

use std::path::Path;
use walkdir::DirEntry;

use crate::model::POI;

pub fn is_file_empty(entry: &DirEntry) -> bool {
    let display = entry.path().display();
    let metadata = match entry.metadata() {
        Err(why) => {
            error!("error reading metadata for {}, {}", display, why);
            return true;
        }
        Ok(value) => value,
    };
    metadata.is_file() && metadata.len() == 0
}

pub fn read_geojson(entry: &DirEntry) -> Result<GeoJson, Box<dyn Error>> {
    let display = entry.path().display();

    let string_value = match fs::read_to_string(display.to_string()) {
        Err(why) => return Err(why.into()),
        Ok(value) => value,
    };
    match serde_json::from_str(string_value.as_str()) {
        Err(why) => Err(why.into()),
        Ok(value) => Ok(value),
    }
}

pub fn write_to_csv(pois: Vec<POI>, output_file: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(output_file);
    fs::create_dir_all(path.parent().unwrap())?;
    let file = match File::create(output_file) {
        Err(why) => panic!("error while creating the file {}: {}", output_file, why),
        Ok(value) => value,
    };
    let mut wtr = csv::Writer::from_writer(file);
    for poi in pois {
        wtr.serialize(poi)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;
    use walkdir::WalkDir;

    #[test]
    fn test_empty_file() {
        let dir = TempDir::new("directory").expect("Failed to create temp dir");
        let file_path = dir.path().join("empty.txt");
        File::create(&file_path).expect("Failed to create file");
        let entry = WalkDir::new(dir.path())
            .into_iter()
            .filter_map(Result::ok)
            .find(|e| e.path().is_file())
            .expect("No file found in temp dir");
        assert!(is_file_empty(&entry));
    }

    #[test]
    fn test_non_empty_file() {
        let dir = TempDir::new("directory").expect("Failed to create temp dir");
        let file_path = dir.path().join("non_empty.txt");
        let mut file = File::create(&file_path).expect("Failed to create file");
        writeln!(file, "Hello").expect("Failed to write to file");
        let entry = WalkDir::new(dir.path())
            .into_iter()
            .filter_map(Result::ok)
            .find(|e| e.path().is_file())
            .expect("No file found in temp dir");
        assert!(!is_file_empty(&entry));
    }

    #[test]
    fn test_read_geojson_valid() {
        let temp_dir = TempDir::new("geojson_test").expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("valid.geojson");

        let geojson_content = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": [102.0, 0.5]
                    },
                    "properties": {
                        "name": "Sample Point"
                    }
                }
            ]
        }"#;

        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(geojson_content.as_bytes())
            .expect("Failed to write to test file");

        for entry in WalkDir::new(temp_dir.path())
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.path().is_file() {
                let result = read_geojson(&entry);
                assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
            }
        }
    }

    #[test]
    fn test_read_geojson_invalid() {
        let temp_dir = TempDir::new("geojson_test").expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("invalid.geojson");

        let invalid_geojson_content = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "geometry": {
                        "type": "INVALID_TYPE",
                        "coordinates": [102.0, 0.5]
                    },
                    "properties": {
                        "name": "Sample Point"
                    }
                }
            ]
        }"#;

        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(invalid_geojson_content.as_bytes())
            .expect("Failed to write to test file");

        for entry in WalkDir::new(temp_dir.path())
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.path().is_file() {
                let result = read_geojson(&entry);
                assert!(result.is_err(), "Expected Err, got Ok: {:?}", result);
            }
        }
    }

    #[test]
    fn test_read_geojson_non_json() {
        let temp_dir = TempDir::new("geojson_test").expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("not_json.txt");

        let non_json_content = "This is not a JSON file";

        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(non_json_content.as_bytes())
            .expect("Failed to write to test file");

        for entry in WalkDir::new(temp_dir.path())
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.path().is_file() {
                let result = read_geojson(&entry);
                assert!(result.is_err(), "Expected Err, got Ok: {:?}", result);
            }
        }
    }
}
