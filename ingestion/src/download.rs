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

pub fn download_atp_data(output_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = get_file_url();
    let path = Path::new(&output_path);

    debug!(
        "creating directory at {}",
        path.parent().unwrap_or_else(|| Path::new("")).display()
    );

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            error!("failed to create directory {}: {}", parent.display(), e);
            e
        })?;
    }

    debug!("creating output file at {}", output_path);
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            error!("failed to create file at {}: {}", output_path, e);
            return Err(e.into());
        }
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(120, 0)))
        .build()
        .map_err(|e| {
            error!("failed to build HTTP client: {}", e);
            e
        })?;

    debug!("preparing request to download from {}", url);
    let req = client.get(&url).build().map_err(|e| {
        error!("failed to build request for {}: {}", url, e);
        e
    })?;

    debug!("downloading zip file from {}", url);
    let resp = match client.execute(req) {
        Ok(response) => {
            if !response.status().is_success() {
                let status = response.status();
                warn!("received non-success status code {} from {}", status, url);
                return Err(format!("HTTP error: status code {}", status).into());
            }

            match response.bytes() {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("failed to read response body from {}: {}", url, e);
                    return Err(e.into());
                }
            }
        }
        Err(e) => {
            error!("request to {} failed: {}", url, e);
            return Err(e.into());
        }
    };

    info!("successfully downloaded {} bytes from {}", resp.len(), url);

    debug!("writing downloaded data to {}", output_path);
    match file.write_all(&resp) {
        Ok(_) => {
            info!("successfully wrote file to {}", output_path);
            Ok(())
        }
        Err(e) => {
            error!("failed to write data to {}: {}", output_path, e);
            Err(e.into())
        }
    }
}
