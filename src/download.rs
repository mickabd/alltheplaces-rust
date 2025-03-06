use std::io::Write;
use std::{fs::File, path::Path, time::Duration};

use regex::Regex;

fn get_file_url() -> String {
    let atp_base_url = String::from("https://data.alltheplaces.xyz/runs/latest/info_embed.html");
    let request = match reqwest::blocking::get(&atp_base_url) {
        Err(why) => panic!("not able to reach the url {}, {}", atp_base_url, why),
        Ok(value) => value,
    };
    let body = match request.text() {
        Err(why) => panic!("not able to parse the body of the request, {}", why),
        Ok(value) => value,
    };
    // unwrap is ok because it's a harcoded value
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']").unwrap();
    let captures = match re.captures(&body) {
        Some(captures) => captures,
        None => panic!("could not find the latest URL download link!"),
    };
    // The first captures is always the full string containing the match.
    let url = match captures.get(1) {
        Some(url) => url.as_str(),
        None => panic!("capture group not found"),
    };
    url.to_string()
}

pub fn download_atp_data(output_path: &String) {
    let url = get_file_url();
    let path = Path::new(&output_path);
    let mut file = match File::create(path) {
        Err(why) => panic!("not able to create the file {}: {}", output_path, why),
        Ok(value) => value,
    };
    println!("Getting the zip file from {}", url);
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(120, 0)))
        .build()
        .unwrap();

    let req = match client.get(&url).build() {
        Err(why) => panic!(
            "error when building the request to get the zip file at {}: {}",
            url, why
        ),
        Ok(value) => value,
    };

    let resp = match client.execute(req) {
        Err(why) => panic!("error requesting the zip file at {}: {}", url, why),
        Ok(value) => value,
    }
    .bytes()
    .unwrap();
    println!("Got file from {}", url);
    file.write_all(&resp).unwrap();
}
