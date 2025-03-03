# alltheplaces-rust

This is a Rust application that uses the [alltheplaces](https://github.com/alltheplaces/alltheplaces) data.
The objective of this application is to extract the data from their website, clean and reorganise it.

## How to run

1. Clone the repository
2. Run `cargo run` in the root directory

A `temp` folder will be created.
It will contains the original zip file from Alltheplaces website.
A folder called `output` will contains all the data extracted from the zip file in the original `geojson` format.
