CREATE TABLE poi (
    id SERIAL PRIMARY KEY,
    spider_id VARCHAR(255) NOT NULL,
    poi_name VARCHAR(255),
    brand VARCHAR(255),
    brand_wikidata_id VARCHAR(255),
    website VARCHAR(255),
    opening_hours TEXT,
    phone VARCHAR(255),
    point GEOMETRY(POINT, 4326),
    city VARCHAR(255),
    zipcode VARCHAR(255),
    house_number VARCHAR(255),
    street_address VARCHAR(255),
    country VARCHAR(255),
    state VARCHAR(255),
    full_address VARCHAR(255),
    street_name VARCHAR(255),
    country_code VARCHAR(15),
);
