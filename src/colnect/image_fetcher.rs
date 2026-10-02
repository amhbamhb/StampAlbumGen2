use reqwest::{Client, StatusCode};
use std::time::Duration;

/// A lightweight trait that allows injecting a test double for network
/// operations. Production code should use `ReqwestFetcher` while tests can
/// implement a fake `Fetcher`.
pub trait Fetcher {
    /// Fetch text body for a URL, returning status and the textual body.
    fn fetch_text(
        &self,
        url: &str,
    ) -> impl std::future::Future<Output = Result<(StatusCode, String), reqwest::Error>> + Send;

    /// Fetch raw bytes for a URL, returning (status, headers, bytes).
    fn fetch_bytes(
        &self,
        url: &str,
    ) -> impl std::future::Future<
        Output = Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error>,
    > + Send;
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
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"),
        );
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
        );

        let client = Client::builder()
            .default_headers(default_headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to build reqwest client");

        Self { client }
    }

    /// Build a client tuned for image downloads (browser-like headers,
    /// limited redirects, shorter timeout).
    pub fn new_for_images() -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"),
        );
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("image/webp,image/apng,image/*,*/*;q=0.8"),
        );

        let client = Client::builder()
            .default_headers(default_headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(Duration::from_secs(30)) // Shorter timeout for image-focused requests
            .build()
            .expect("Failed to build image reqwest client");

        Self { client }
    }

    /// Build a client tuned for API calls (includes User-Agent header).
    pub fn new_for_api() -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("StampAlbumGen2/1.0 (https://github.com/your-repo/stamp-album-gen2)"),
        );
        // Prefer JSON responses from APIs
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(default_headers)
            .timeout(Duration::from_secs(60))
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
    async fn fetch_text(&self, url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        let body = resp.text().await?;
        Ok((status, body))
    }

    async fn fetch_bytes(&self, url: &str) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error> {
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        let headers = resp.headers().clone();
        let bytes = resp.bytes().await?;
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
pub async fn fetch_text(client: &Client, url: &str) -> Result<(StatusCode, String), reqwest::Error> {
    let resp = client.get(url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;
    Ok((status, body))
}
pub async fn fetch_bytes(
    client: &Client,
    url: &str,
) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error> {
    let resp = client.get(url).send().await?;
    let status = resp.status();
    let headers = resp.headers().clone();
    let bytes = resp.bytes().await?;
    Ok((status, headers, bytes.to_vec()))
}

/// Resolves the Colnect image URL from a StampWorld page URL.
pub fn resolve_colnect_image_url(page_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Placeholder implementation for now. Actual logic requires knowledge of the StampWorld structure or other endpoints.
    Err(format!("resolve_colnect_image_url not yet implemented in this context for URL: {}", page_url).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_text_invalid_url_err() {
        let f = ReqwestFetcher::new();
        let res = tokio::runtime::Runtime::new().unwrap().block_on(fetch_text(f.client(),   "http:::/::invalid-url"));
        assert!(res.is_err());
    }

    #[test]
    fn test_fetch_bytes_invalid_url_err() {
        let f = ReqwestFetcher::new();
        let res = tokio::runtime::Runtime::new().unwrap().block_on(fetch_bytes(f.client(), "http:::/::invalid-url"));
        assert!(res.is_err());
    }
}
