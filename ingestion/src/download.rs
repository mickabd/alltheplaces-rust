use std::fs;
use std::io::Write;
use std::{fs::File, path::Path, time::Duration};

use log::{debug, error, info, warn};
use regex::Regex;

fn get_file_url() -> String {
    debug!("getting the latest URL download link");
    let atp_base_url = String::from("https://data.alltheplaces.xyz/runs/latest/info_embed.html");

    debug!("attempting to request URL: {}", atp_base_url);
    let request = reqwest::blocking::get(&atp_base_url)
        .expect(format!("failed to connect to {}", atp_base_url).as_str());

    if !request.status().is_success() {
        warn!(
            "request returned non-success status code: {}",
            request.status()
        );
    }

    debug!("parsing response body from request");
    let body = request.text().expect("failed to parse response body");

    debug!("looking for URL pattern in response body");
    // unwrap is ok because it's a hardcoded value
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']").unwrap();

    let captures = match re.captures(&body) {
        Some(c) => c,
        None => {
            error!("no URL pattern found in response body");
            panic!("could not find the latest URL download link!");
        }
    };

    // The first capture is always the full string containing the match
    let url = captures.get(1).expect("capture group not found").as_str();
    info!("got the latest URL download link: {}", url);

    if !url.starts_with("https://") && !url.starts_with("http://") {
        warn!("URL doesn't start with http(s)://: {}", url);
    }

    url.to_string()
}

pub fn download_atp_data(output_path: &String) {
    let url = get_file_url();
    let path = Path::new(&output_path);

    debug!(
        "Creating the directory {}",
        path.parent().unwrap().display()
    );
    fs::create_dir_all(path.parent().unwrap()).unwrap();

    debug!("Creating an empty file at {}", output_path);
    let mut file =
        File::create(path).expect(format!("not able to create the file {}", output_path).as_str());

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

    debug!("Getting the zip file from {}", url);
    let resp = client
        .execute(req)
        .expect(format!("error requesting the zip file at {}", url).as_str())
        .bytes()
        .unwrap();
    info!("Got file from {}", url);
    debug!("Writing the file to {}", output_path);
    file.write_all(&resp).unwrap();
    info!("File written to {}", output_path);
}
