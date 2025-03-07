use std::fs;
use std::io::Write;
use std::{fs::File, path::Path, time::Duration};

use regex::Regex;

fn get_file_url() -> String {
    let atp_base_url = String::from("https://data.alltheplaces.xyz/runs/latest/info_embed.html");

    let request = reqwest::blocking::get(&atp_base_url)
        .expect(format!("not able to reach the url {}", atp_base_url).as_str());

    let body = request
        .text()
        .expect("not able to parse the body of the request, {}");

    // unwrap is ok because it's a harcoded value
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']").unwrap();
    let captures = re
        .captures(&body)
        .expect("could not find the latest URL download link!");

    // The first captures is always the full string containing the match.
    let url = captures.get(1).expect("capture group not found").as_str();
    url.to_string()
}

pub fn download_atp_data(output_path: &String) {
    let url = get_file_url();
    let path = Path::new(&output_path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut file =
        File::create(path).expect(format!("not able to create the file {}", output_path).as_str());
    println!("Getting the zip file from {}", url);
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(120, 0)))
        .build()
        .unwrap();

    let req = client.get(&url).build().expect(
        format!(
            "error when building the request to get the zip file at {}",
            url
        )
        .as_str(),
    );

    let resp = client
        .execute(req)
        .expect(format!("error requesting the zip file at {}", url).as_str())
        .bytes()
        .unwrap();
    println!("Got file from {}", url);
    file.write_all(&resp).unwrap();
}
