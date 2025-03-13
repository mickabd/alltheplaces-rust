# alltheplaces-rust

This is a Rust application that uses the [alltheplaces](https://github.com/alltheplaces/alltheplaces) data.
The objective of this application is to extract and process data from the alltheplaces website. The processed data is then stored in a local PostgreSQL database. The PostgreSQL database has the PostGIS extension installed. A small async backend is also available to query the data.

The repository is set up using rust workspace: `ingestion` and `backend` are the two crates.

## Ingestion

The data is extracted from the zip file available on their [website](https://data.alltheplaces.xyz/runs/latest/info_embed.html).
The data is then cleaned and reorganised in a more readable format. The output structure can be found [here](ingestion/src/model.rs).

We're parsing and building attributes from original geojson files:

- `poi_name`: from the brand and the poi name.
- `website`: we parsed the provided url to extract the host only.
- `point`: we're using the `geometry` field.
- `country_code`: we reverse geocode the point to get the country code.

Finally, the data is stored in a postgresql database. The database schema can be found [here](ingestion/src/db.rs)

## Backend

The backend is a small async server that allows to query the data stored in the database. The server is using the [actix-web](https://actix.rs/) framework.

The server is exposing endpoints that can be found [here](backend/src/main.rs).

## How to run the ingestion

1. run `docker compose up postgres` to start the postgresql database.
2. run `cargo run --bin ingestion` to start the ingestion process.

The ingestion process will download the zip file, extract the data, clean it and store it in the database.

## How to run the backend

1. run `docker compose up postgres` to start the postgresql database.
2. run `cargo run --bin backend` or `docker compose up backend` to start the backend server.
3. The server will be available at `http://localhost:8080`
