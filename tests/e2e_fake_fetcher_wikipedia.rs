use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use stamp_album_gen2::stampworld::fetch::Fetcher;
use stamp_album_gen2::wikipedia;
use tempfile::tempdir;

struct FakeFetcher {
    response_body: String,
}

impl FakeFetcher {
    fn new(body: &str) -> Self {
        Self {
            response_body: body.to_string(),
        }
    }
}

impl Fetcher for FakeFetcher {
    fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        Ok((StatusCode::OK, self.response_body.clone()))
    }

    fn fetch_bytes(&self, _url: &str) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error> {
        Ok((StatusCode::OK, HeaderMap::new(), Vec::new()))
    }
}

#[test]
fn e2e_wikipedia_fetcher_fake() {
    let td = tempdir().unwrap();
    let album_dir = td.path().to_str().unwrap();
    let fixture = r#"{"extract":"This is a fake extract for testing."}"#;
    let fetcher = FakeFetcher::new(fixture);
    let res =
        wikipedia::fetch_and_save_summary_with_fetcher("Test Topic", album_dir, &fetcher).unwrap();
    let content = std::fs::read_to_string(res).unwrap();
    assert_eq!(content, "This is a fake extract for testing.");
}
