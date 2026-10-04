use async_trait::async_trait;
use reqwest::StatusCode;
use stamp_album_gen2::stampworld::fetch::Fetcher;

struct FakeFetcher;

#[async_trait]
impl Fetcher for FakeFetcher {
    async fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        // Return an empty HTML document for any URL
        Ok((
            StatusCode::OK,
            "<html><head></head><body></body></html>".to_string(),
        ))
    }

    async fn fetch_bytes(
        &self,
        _url: &str,
    ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error> {
        // Return 404/no bytes to signal missing image
        let headers = reqwest::header::HeaderMap::new();
        Ok((StatusCode::NOT_FOUND, headers, Vec::new()))
    }
}

#[tokio::test]
async fn test_collect_with_fake_fetcher_returns_empty() {
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

    let result =
        stamp_album_gen2::stampworld::data_collector::collect_stampworld_data_with_fetcher(
            &fetcher,
            stamp_album_gen2::stampworld::data_collector::CollectStampworldParams {
                sequencer: &mut sequencer,
                separator,
                site_name,
                images_directory,
                month_number_fn: &month_number,
                downloader: &get_stampworld_image,
                base_url,
                start_year: 1900,
                end_year: 1901,
                overwrite_images: false,
                page_callback: None,
                max_pages: 1, // max_pages=1 to keep it quick
                stop_after_no_hits: 1,
            },
        )
        .await;

    // For an empty HTML page we'd expect no stamp lines collected
    assert!(result.is_empty());
}
