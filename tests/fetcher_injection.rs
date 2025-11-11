use reqwest::StatusCode;
use stamp_album_gen2::stampworld::fetch::Fetcher;
use stamp_album_gen2::stampworld::fetch::ReqwestFetcher;

struct FakeFetcher;

impl Fetcher for FakeFetcher {
    fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        // Return an empty HTML document for any URL
        Ok((
            StatusCode::OK,
            "<html><head></head><body></body></html>".to_string(),
        ))
    }

    fn fetch_bytes(
        &self,
        _url: &str,
    ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error> {
        // Return 404/no bytes to signal missing image
        let headers = reqwest::header::HeaderMap::new();
        Ok((StatusCode::NOT_FOUND, headers, Vec::new()))
    }
}

#[test]
fn test_collect_with_fake_fetcher_returns_empty() {
    // Prepare inputs; use a temp dir under /tmp to avoid writing into workspace
    let mut sequencer: i32 = 0;
    let separator = "@";
    let site_name = "test-site";
    let images_directory = "/tmp/stamp_album_gen2_test_images";
    let month_number = |_: &str| "0".to_string();
    let get_stampworld_image = |_url: &str, _desired: &str, _overwrite: bool| {
        stamp_album_gen2::types::ImageStatus::new("", false, 0, 0, "")
    };
    let base_url = "https://example.invalid/page=";

    // Use the fake fetcher which returns empty HTML for every request
    let fetcher = FakeFetcher;

    let result = stamp_album_gen2::stampworld::data_collector::collect_stampworld_data_with_fetcher(
        &fetcher,
        &mut sequencer,
        separator,
        site_name,
        images_directory,
        &month_number,
        &get_stampworld_image,
        base_url,
        1900,
        1901,
        false,
        None,
        1, // max_pages=1 to keep it quick
        1,
    );

    // For an empty HTML page we'd expect no stamp lines collected
    assert!(result.is_empty());
}
