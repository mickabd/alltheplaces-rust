use std::io::Write;

use geo::Point;
use log::{debug, error, info};
use postgres::{Client, NoTls};

use crate::model::POI;

pub fn get_client(
    host: String,
    user: String,
    password: String,
    port: String,
    dbname: String,
) -> Client {
    debug!("attempting to connect to database at {}:{}", host, port);

    let connection_string = format!(
        "host={} user={} password={} port={} dbname={}",
        host, user, password, port, dbname
    );

    match Client::connect(connection_string.as_str(), NoTls) {
        Ok(client) => {
            info!("successfully connected to database '{}'", dbname);
            client
        }
        Err(err) => {
            error!("failed to connect to database: {}", err);
            panic!("error while creating the sql client: {}", err);
        }
    }
}

pub fn truncate_table(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    debug!("attempting to truncate poi table");

    let mut transaction = client.transaction()?;
    let query = "truncate table poi;";

    debug!("executing query: {}", query);
    transaction.execute(query, &[])?;

    debug!("committing transaction");
    transaction.commit()?;

    info!("successfully truncated poi table");
    Ok(())
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
        street_name,
        country_code
    ) FROM STDIN";
    let mut writer = transaction.copy_in(query)?;

    // Create a single buffer for all POIs
    // Preallocate a reasonable size
    let mut buffer = String::with_capacity(pois.len() * 256);

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
        buffer.push('\t');
        buffer.push_str(&escape_field(&poi.country_code));
        buffer.push('\n');
    }

    // Remove any null characters from the buffer
    buffer.retain(|c| c != '\u{0000}');

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
