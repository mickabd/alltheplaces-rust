use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Properties {
    pub r#ref: Option<String>,
    #[serde(rename = "@spider")]
    pub spider_id: String,
    #[serde(rename = "@source_uri")]
    pub source_uri: String,
    pub branch: Option<String>,
    pub name: Option<String>,
    pub brand: String,
    #[serde(rename = "@brand:wikidata")]
    pub brand_wikidata_id: Option<String>,
    pub operator: Option<String>,
    #[serde(rename = "@operator:wikidata")]
    pub operator_wikidata_id: Option<String>,
    #[serde(rename = "@addr:full")]
    pub address_full: Option<String>,
    #[serde(rename = "@addr:housenumber")]
    pub address_housenumber: Option<String>,
    #[serde(rename = "@addr:street")]
    pub address_street: Option<String>,
    #[serde(rename = "@addr:street_address")]
    pub address_street_address: Option<String>,
    #[serde(rename = "@addr:city")]
    pub address_city: Option<String>,
    #[serde(rename = "@addr:state")]
    pub address_state: Option<String>,
    #[serde(rename = "@addr:postcode")]
    pub address_postcode: Option<String>,
    #[serde(rename = "@addr:country")]
    pub address_country: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "@contact:twitter")]
    pub contact_twitter: Option<String>,
    #[serde(rename = "@contact:facebook")]
    pub contact_facebook: Option<String>,
    pub opening_hours: Option<String>,
    pub image: Option<String>,
    pub located_in: Option<String>,
    #[serde(rename = "@located_in:wikidata")]
    pub located_in_wikidata_id: Option<String>,
    pub nsi_id: Option<String>,
    pub end_date: Option<String>,
}

pub struct Coordinates<'a> {
    pub longitude: &'a f64,
    pub latitude: &'a f64,
}

pub struct Address<'a> {
    pub city: &'a Option<String>,
    pub zipcode: &'a Option<String>,
    pub house_number: &'a Option<String>,
    pub street_address: &'a Option<String>,
    pub country: &'a Option<String>,
    pub state: &'a Option<String>,
    pub full_address: &'a Option<String>,
    pub street_name: &'a Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

#[derive(Deserialize, Debug)]
pub struct Feature {
    pub r#type: String,
    pub id: String,
    pub properties: Properties,
    pub geometry: Geometry,
}
pub struct POI {
    pub spider_id: String,
}
