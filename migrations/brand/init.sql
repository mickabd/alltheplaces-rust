CREATE TABLE brand (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    wikidata_id VARCHAR(255),
    CONSTRAINT brand_name UNIQUE (name)
);

CREATE INDEX idx_brand_id ON brand (id);
