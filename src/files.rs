extern crate geojson;
extern crate url;

use geojson::GeoJson;
use std::error::Error;
use std::fs::{self, File};

use std::path::Path;
use walkdir::DirEntry;

use crate::model::POI;

pub fn is_file_empty(entry: &DirEntry) -> bool {
    let display = entry.path().display();
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

pub fn read_geojson(entry: &DirEntry) -> Result<GeoJson, Box<dyn Error>> {
    let display = entry.path().display();
    println!("trying to parse {} into a geojson", display);
    let string_value = match fs::read_to_string(&display.to_string()) {
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
