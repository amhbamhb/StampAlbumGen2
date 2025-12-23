use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

/// A lightweight trait that allows injecting a test double for network
/// operations. Production code should use `ReqwestFetcher` while tests can
/// implement a fake `Fetcher`.
pub trait Fetcher {
    /// Fetch text body for a URL, returning status and the textual body.
    fn fetch_text(&self, url: &str) -> Result<(StatusCode, String), reqwest::Error>;

    /// Fetch raw bytes for a URL, returning (status, headers, bytes).
    fn fetch_bytes(&self, url: &str) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error>;
}

/// Default Reqwest-based fetcher implementation.
pub struct ReqwestFetcher {
    client: Client,
}

impl Default for ReqwestFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ReqwestFetcher {
    /// Build a standard client used by the site collector (60s timeout).
    pub fn new() -> Self {
        // Use a browser-like User-Agent and reasonable redirect policy so
        // StampWorld pages return the same content a browser / curl would.
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        default_headers.insert(
            reqwest::header::ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
                .parse()
                .unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(default_headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("Failed to build reqwest client");
        Self { client }
    }

    /// Build a client tuned for image downloads (browser-like headers,
    /// limited redirects, shorter timeout).
    pub fn new_for_images() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        default_headers.insert(
            reqwest::header::ACCEPT,
            "image/webp,image/apng,image/*,*/*;q=0.8"
                .parse()
                .unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(default_headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build image reqwest client");

        Self { client }
    }

    /// Build a client tuned for API calls (includes User-Agent header).
    pub fn new_for_api() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT,
            "StampAlbumGen2/1.0 (https://github.com/your-repo/stamp-album-gen2)"
                .parse()
                .unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(default_headers)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("Failed to build API reqwest client");

        Self { client }
    }

    /// Convenience: return an owned Client if callers need it directly.
    pub fn client(&self) -> &Client {
        &self.client
    }
}

impl Fetcher for ReqwestFetcher {
    fn fetch_text(&self, url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        let resp = self.client.get(url).send()?;
        let status = resp.status();
        let body = resp.text()?;
        Ok((status, body))
    }

    fn fetch_bytes(&self, url: &str) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error> {
        let resp = self.client.get(url).send()?;
        let status = resp.status();
        let headers = resp.headers().clone();
        let bytes = resp.bytes()?;
        Ok((status, headers, bytes.to_vec()))
    }
}

/// Backwards-compatible free functions used by a few callers in the repo.
pub fn build_client() -> Client {
    ReqwestFetcher::new().client().clone()
}

pub fn build_image_client() -> Client {
    ReqwestFetcher::new_for_images().client().clone()
}

pub fn fetch_text(client: &Client, url: &str) -> Result<(StatusCode, String), reqwest::Error> {
    let resp = client.get(url).send()?;
    let status = resp.status();
    let body = resp.text()?;
    Ok((status, body))
}

pub fn fetch_bytes(
    client: &Client,
    url: &str,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error> {
    let resp = client.get(url).send()?;
    let status = resp.status();
    let headers = resp.headers().clone();
    let bytes = resp.bytes()?;
    Ok((status, headers, bytes.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_text_invalid_url_err() {
        let f = ReqwestFetcher::new();
        let res = f.fetch_text("http:::/::invalid-url");
        assert!(res.is_err());
    }

    #[test]
    fn test_fetch_bytes_invalid_url_err() {
        let f = ReqwestFetcher::new();
        let res = f.fetch_bytes("http:::/::invalid-url");
        assert!(res.is_err());
    }
}
