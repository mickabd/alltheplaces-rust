CREATE TABLE brand (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    wikidata_id VARCHAR(255),
    CONSTRAINT brand_name UNIQUE (name)
);

CREATE INDEX idx_brand_id ON brand (id);

CREATE TABLE poi (
    id SERIAL PRIMARY KEY,
    spider_id VARCHAR(255) NOT NULL,
    poi_name TEXT,
    brand_id INTEGER REFERENCES brand(id),
    website VARCHAR(255),
    opening_hours TEXT,
    phone VARCHAR(255),
    point GEOMETRY(POINT, 4326),
    city VARCHAR(255),
    zipcode VARCHAR(255),
    house_number VARCHAR(255),
    street_address TEXT,
    country VARCHAR(255),
    state VARCHAR(255),
    full_address TEXT,
    street_name TEXT,
    country_code VARCHAR(15)
);

CREATE INDEX idx_poi_point ON poi USING GIST (point);
CREATE INDEX idx_poi_brand_id ON poi (brand_id);
