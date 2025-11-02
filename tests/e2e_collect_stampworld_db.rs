use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use stamp_album_gen2::stampworld::fetch::Fetcher;
use std::env;
use tempfile::tempdir;

struct FakeFetcher {
    html_fixture: String,
}

impl FakeFetcher {
    fn new(html: &str) -> Self {
        Self {
            html_fixture: html.to_string(),
        }
    }
}

impl Fetcher for FakeFetcher {
    fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        Ok((StatusCode::OK, self.html_fixture.clone()))
    }

    fn fetch_bytes(&self, _url: &str) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error> {
        Ok((StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new()))
    }
}

#[test]
fn e2e_collect_stampworld_inserts_placeholders_to_db() {
    // Prepare a temporary images directory so the collector creates a DB file we can inspect
    let td = tempdir().expect("tempdir");
    // Create a minimal config.toml in the temp dir so load_config finds expand_grouped_ranges
    let config_toml = r#"expand_grouped_ranges = "placeholders"
"#;
    let orig_cwd = env::current_dir().expect("cwd");
    std::fs::write(td.path().join("config.toml"), config_toml).expect("write config");
    // Switch to tempdir so load_config("config.toml") finds our file
    env::set_current_dir(td.path()).expect("set cwd");
    let images_dir = td.path().join("images");
    std::fs::create_dir_all(&images_dir).unwrap();
    let images_dir_str = images_dir.to_string_lossy().to_string();

    // Minimal HTML fixture with one grouped-range (1-2) which should produce 2 placeholders
    let fixture = r#"
    <div class="container-fluid content_table" id="group_box_1">
      <div class="row">
        <div class="col-12 table_header">
          <a>1970  Example Series</a>
          <p>1. January</p>
        </div>
      </div>
      <div class="row">
        <div class="col-12">
          <table>
            <tr class="stamp_tr">
              <th>1-2</th>
              <td>Type</td>
              <td>Denomination</td>
              <td>Color</td>
              <td></td>
              <td>100</td>
              <td>Description here</td>
            </tr>
          </table>
        </div>
      </div>
    </div>
    "#;

    let fetcher = FakeFetcher::new(fixture);

    // A brief sanity check (kept minimal): ensure config can be loaded from the tempdir
    let _cfg = stamp_album_gen2::config::load_config("config.toml");
    let mut sequencer: i32 = 0;
    let separator = "@";
    let site_name = "ExampleSeries"; // used as country identifier in DB
    let month_number = |_: &str| "1".to_string();
    let get_stampworld_image = |_url: &str, _overwrite: bool| {
        stamp_album_gen2::types::ImageStatus::new("", false, 0, 0, "")
    };
    let base_url = "https://example.test/page=";

    // Run collector (max_pages=1 so it only processes our single fixture)
    let results =
        stamp_album_gen2::stampworld::data_collector::collect_stampworld_data_with_fetcher(
            &fetcher,
            &mut sequencer,
            separator,
            site_name,
            &images_dir_str,
            &month_number,
            &get_stampworld_image,
            base_url,
            1970,
            1970,
            false,
            None,
            1,
            1,
        );

    // Collector returns a compact STAMP line vector; we only assert placeholders exist.

    // We expect the collector to report two placeholder stamp lines
    // (one for serial 1 and one for 2) in the returned vector
    let placeholder_lines: Vec<&String> = results
        .iter()
        .filter(|s| s.contains("PLACEHOLDER"))
        .collect();
    assert_eq!(
        placeholder_lines.len(),
        2,
        "expected 2 placeholder stamp lines, got: {:?}",
        results
    );

    // Now open the DB the collector created and assert two rows exist for the series
    // Collector builds DB path by dropping "/images" from images_dir and appending "{site_name}.db"
    let db_parent = images_dir_str.replace("/images", "");
    let db_path = format!("{}/{}.db", db_parent, site_name.replace('/', "-"));

    // We expect the collector to have created the DB file under the images parent.

    let conn = stamp_album_db::init_db(&db_path).expect("init db");
    // The collector stores the series string as extracted from the header (includes year prefix)
    let stored_series = "1970  Example Series";
    // Count rows directly to avoid mapping NULL columns for placeholder rows
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM stamps WHERE country = ?1 AND series = ?2",
            rusqlite::params![site_name, stored_series],
            |r| r.get(0),
        )
        .expect("count query");
    assert!(count >= 2, "expected at least 2 rows in DB, got {}", count);

    // Clean up (tempdir will be removed automatically)
    // Restore original cwd so later tests are not affected
    env::set_current_dir(orig_cwd).expect("restore cwd");
}
