use geozero::{ToWkt, wkb};
use serde::{Serialize, ser::SerializeStruct};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Poi {
    pub id: i32,
    pub spider_id: String,
    pub poi_name: Option<String>,
    pub brand: Option<String>,
    pub brand_wikidata_id: Option<String>,
    pub website: Option<String>,
    pub opening_hours: Option<String>,
    pub phone: Option<String>,
    pub point: wkb::Decode<geo_types::Geometry<f64>>,
    pub city: Option<String>,
    pub zipcode: Option<String>,
    pub house_number: Option<String>,
    pub street_address: Option<String>,
    pub country: Option<String>,
    pub country_code: String,
    pub state: Option<String>,
    pub full_address: Option<String>,
    pub street_name: Option<String>,
}

impl Serialize for Poi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("POI", 18)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("spider_id", &self.spider_id)?;
        state.serialize_field("poi_name", &self.poi_name)?;
        state.serialize_field("brand", &self.brand)?;
        state.serialize_field("brand_wikidata_id", &self.brand_wikidata_id)?;
        state.serialize_field("website", &self.website)?;
        state.serialize_field("opening_hours", &self.opening_hours)?;
        state.serialize_field("phone", &self.phone)?;
        state.serialize_field(
            "point",
            &self
                .point
                .geometry
                .as_ref()
                .and_then(|value| value.to_wkt().ok()),
        )?;
        state.serialize_field("city", &self.city)?;
        state.serialize_field("zipcode", &self.zipcode)?;
        state.serialize_field("house_number", &self.house_number)?;
        state.serialize_field("street_address", &self.street_address)?;
        state.serialize_field("country", &self.country)?;
        state.serialize_field("country_code", &self.country_code)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("full_address", &self.full_address)?;
        state.serialize_field("street_name", &self.street_name)?;
        state.end()
    }
}
