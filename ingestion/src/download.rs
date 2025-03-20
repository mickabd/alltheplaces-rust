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
    let path = Path::new(output_path);
    if let Some(parent) = path.parent() {
        debug!("creating directory at {}", parent.display());
        fs::create_dir_all(parent).map_err(|e| {
            error!("failed to create directory {}: {}", parent.display(), e);
            e
        })?;
    }

    debug!("building HTTP client");
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(120, 0)))
        .build()
        .map_err(|e| {
            error!("failed to build HTTP client: {}", e);
            e
        })?;

    debug!("downloading zip file from {}", file_url);
    let resp = client.get(file_url).send().map_err(|e| {
        error!("request to {} failed: {}", file_url, e);
        e
    })?;

    if !resp.status().is_success() {
        let status = resp.status();
        error!(
            "received non-success status code {} from {}",
            status, file_url
        );
        return Err(format!("HTTP error: status code {}", status).into());
    }

    let bytes = resp.bytes().map_err(|e| {
        error!("failed to read response body from {}: {}", file_url, e);
        e
    })?;

    info!(
        "successfully downloaded {} bytes from {}",
        bytes.len(),
        file_url
    );

    debug!("creating and writing to output file at {}", output_path);
    let mut file = File::create(path).map_err(|e| {
        error!("failed to create file at {}: {}", output_path, e);
        e
    })?;

    file.write_all(&bytes).map_err(|e| {
        error!("failed to write data to {}: {}", output_path, e);
        e
    })?;

    info!("successfully wrote file to {}", output_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_url_success() {
        let mock_body = r#"<!DOCTYPE html><html><body><a href="https://example.com/file.zip">Download GeoJSON</a>(575 MB)<br/><small>7,878,700 rows from3,694 spiders, updated 2025-03-17T14:47:33Z</small></body></html>"#;
        let mut server = mockito::Server::new();
        let url = server.url();
        let _mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body(mock_body)
            .create();

        let result = get_file_url(&url);
        assert_eq!(result, "https://example.com/file.zip");
    }

    #[test]
    #[should_panic(expected = "failed to connect to")]
    fn test_get_file_url_connection_failure() {
        let url = "http://nonexistent.url";
        get_file_url(url);
    }

    #[test]
    #[should_panic(expected = "failed to get the latest URL download link")]
    fn test_get_file_url_non_success_status() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let _m = server.mock("GET", "/").with_status(404).create();
        get_file_url(&url);
    }

    #[test]
    #[should_panic(expected = "could not find the latest URL download link!")]
    fn test_get_file_url_no_url_in_body() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock_body = r#"<html><body>No links here</body></html>"#;
        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body(mock_body)
            .create();

        get_file_url(&url);
    }

    #[test]
    #[should_panic(expected = "could not find the latest URL download link!")]
    fn test_get_file_url_invalid_body() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("")
            .create();
        get_file_url(&url);
    }
}
