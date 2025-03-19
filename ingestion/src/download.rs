use std::fs;
use std::io::Write;
use std::{fs::File, path::Path, time::Duration};

use log::{debug, error, info, warn};
use regex::Regex;

pub fn get_file_url(url: &str) -> String {
    debug!("getting the latest URL download link");
    debug!("attempting to request URL: {}", url);

    // Make the HTTP request
    let response = reqwest::blocking::get(url)
        .unwrap_or_else(|e| panic!("failed to connect to {}: {}", url, e));

    // Check status code
    if !response.status().is_success() {
        warn!(
            "request returned non-success status code: {}",
            response.status()
        );
        panic!("failed to get the latest URL download link!");
    }

    debug!("parsing response body from request");
    let body = response.text().expect("failed to parse response body");

    debug!("looking for URL pattern in response body");
    // unwrap is ok because it's a hardcoded value
    let re = Regex::new("href=[\"\'](https?://[^\"\']+?)[\"\']").unwrap();

    // Find the URL in the response body
    match re.captures(&body) {
        Some(captures) => {
            let extracted_url = captures.get(1).expect("capture group not found").as_str();

            info!("got the latest URL download link: {}", extracted_url);
            extracted_url.to_string()
        }
        None => {
            error!("no URL pattern found in response body");
            panic!("could not find the latest URL download link!");
        }
    }
}

pub fn download_atp_data(
    output_path: &str,
    file_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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

    debug!("preparing request to download from {}", file_url);
    let req = client.get(file_url).build().map_err(|e| {
        error!("failed to build request for {}: {}", file_url, e);
        e
    })?;

    debug!("downloading zip file from {}", file_url);
    let resp = match client.execute(req) {
        Ok(response) => {
            if !response.status().is_success() {
                let status = response.status();
                warn!(
                    "received non-success status code {} from {}",
                    status, file_url
                );
                return Err(format!("HTTP error: status code {}", status).into());
            }

            match response.bytes() {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("failed to read response body from {}: {}", file_url, e);
                    return Err(e.into());
                }
            }
        }
        Err(e) => {
            error!("request to {} failed: {}", file_url, e);
            return Err(e.into());
        }
    };

    info!(
        "successfully downloaded {} bytes from {}",
        resp.len(),
        file_url
    );

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
