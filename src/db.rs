extern crate postgres;

use std::io::Write;

use geo::Point;
use postgres::{Client, NoTls};

use crate::model::POI;

pub fn get_client(
    host: String,
    user: String,
    password: String,
    port: String,
    dbname: String,
) -> Client {
    match Client::connect(
        format!(
            "host={} user={} password={} port={} dbname={}",
            host, user, password, port, dbname
        )
        .as_str(),
        NoTls,
    ) {
        Err(why) => panic!("error while creating the sql client: {}", why),
        Ok(value) => value,
    }
}

pub fn ingest_into_db(
    client: &mut Client,
    pois: Vec<POI>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = client.transaction()?;
    let query = "
    COPY poi (
        spider_id,
        poi_name,
        brand,
        brand_wikidata_id,
        website,
        opening_hours,
        phone,
        point,
        city,
        zipcode,
        house_number,
        street_address,
        country,
        state,
        full_address,
        street_name
    ) FROM STDIN";
    let mut writer = transaction.copy_in(query)?;

    // Create a single buffer for all POIs
    let mut buffer = String::with_capacity(pois.len() * 256); // Preallocate a reasonable size

    // Process all POIs and build the complete buffer
    for poi in pois {
        // Format each field with proper escaping and tab separation
        buffer.push_str(&poi.spider_id);
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.poi_name.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.brand.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.brand_wikidata_id.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.website.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.opening_hours.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.phone.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&point_to_string(&poi.point));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.city.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.zipcode.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.house_number.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.street_address.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.country.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.state.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.full_address.unwrap_or_default()));
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.street_name.unwrap_or_default()));
        buffer.push('\n');
    }

    // Write the entire buffer at once
    writer.write_all(buffer.as_bytes())?;

    // Finish the COPY operation
    writer.finish()?;

    // Commit the transaction
    transaction.commit()?;
    Ok(())
}

fn point_to_string(point: &Option<Point>) -> String {
    match point {
        None => "\\N".to_string(),
        Some(point) => format!("Point({} {})", point.x(), point.y()),
    }
}

// Helper function to escape fields for PostgreSQL COPY
fn escape_field(field: &str) -> String {
    if field.is_empty() {
        "\\N".to_string() // PostgreSQL NULL representation
    } else {
        field
            .replace("\\", "\\\\")
            .replace("\t", "\\t")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
    }
}

pub fn to_db(client: &mut Client) {
    let rows = match client.query("select count(1) from poi;", &[]) {
        Err(why) => panic!("error reading the table {}", why),
        Ok(value) => value,
    };
    for row in rows {
        let value: i64 = row.get("count");
        println!("row: {}", value);
    }
}
