use async_trait::async_trait;
use reqwest::StatusCode;
use stamp_album_gen2::stampworld::fetch::Fetcher;
use tempfile::tempdir;

struct FakeFetcherEmpty;

#[async_trait]
impl Fetcher for FakeFetcherEmpty {
    async fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        Ok((StatusCode::OK, "<html><body></body></html>".to_string()))
    }

    async fn fetch_bytes(
        &self,
        _url: &str,
    ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error> {
        Ok((
            StatusCode::NOT_FOUND,
            reqwest::header::HeaderMap::new(),
            Vec::new(),
        ))
    }
}

#[tokio::test]
async fn collector_creates_db_at_expected_path() {
    // Create a temp dir and an images/ child to simulate album layout
    let td = tempdir().expect("tempdir");
    let images_dir = td.path().join("images");
    std::fs::create_dir_all(&images_dir).expect("create images dir");
    let images_dir_str = images_dir.to_string_lossy().to_string();

    // Use the collector with a tiny fake fetcher. The collector computes
    // the DB path from images_directory by removing "/images" and using the
    // basename of the parent folder as the DB filename.
    let fetcher = FakeFetcherEmpty;
    let mut sequencer: i32 = 0;
    let separator = "@";
    let site_name = "SomeSite";
    let month_number = |_: &str| "1".to_string();
    let get_stampworld_image = |_u: &str, _d: &str, _o: bool| {
        stamp_album_gen2::types::ImageStatus::new("", false, 0, 0, "")
    };

    let base_url = "https://example.test/page=";

    // Run collector; it will initialize the DB at the expected path
    let _ = stamp_album_gen2::stampworld::data_collector::collect_stampworld_data_with_fetcher(
        &fetcher,
        stamp_album_gen2::stampworld::data_collector::CollectStampworldParams {
            sequencer: &mut sequencer,
            separator,
            site_name,
            images_directory: &images_dir_str,
            month_number_fn: &month_number,
            downloader: &get_stampworld_image,
            base_url,
            start_year: 2000,
            end_year: 2001,
            overwrite_images: false,
            page_callback: None,
            max_pages: 1,
            stop_after_no_hits: 1,
        },
    )
    .await;

    // Compute expected DB path the same way the collector does
    let db_parent = images_dir_str.replace("/images", "");
    let album_basename = std::path::Path::new(&db_parent)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| site_name.replace('/', "-"));
    let db_path = format!("{}/{}.db", db_parent, album_basename);

    // Assert the DB file exists and is a regular file
    let p = std::path::Path::new(&db_path);
    assert!(
        p.exists(),
        "Expected DB file at {}, but it does not exist",
        db_path
    );
    assert!(p.is_file(), "Expected DB path to be a file: {}", db_path);
}
