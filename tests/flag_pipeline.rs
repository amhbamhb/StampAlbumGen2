use async_trait::async_trait;
use reqwest::{StatusCode, header::HeaderMap};
use stamp_album_gen2::stampworld::fetch::Fetcher;
use std::{path::PathBuf, sync::Mutex};

struct RestoreEnvironment {
    cwd: PathBuf,
    flags_dir: Option<std::ffi::OsString>,
    albums_root: Option<std::ffi::OsString>,
}

impl Drop for RestoreEnvironment {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.cwd).expect("restore test cwd");
        match &self.flags_dir {
            Some(value) => std::env::set_var("STAMP_FLAGS_DIR", value),
            None => std::env::remove_var("STAMP_FLAGS_DIR"),
        }
        match &self.albums_root {
            Some(value) => std::env::set_var("STAMP_ALBUMS_ROOT", value),
            None => std::env::remove_var("STAMP_ALBUMS_ROOT"),
        }
    }
}

struct FakeFetcher {
    bytes: Vec<u8>,
    urls: Mutex<Vec<String>>,
}

#[async_trait]
impl Fetcher for FakeFetcher {
    async fn fetch_text(&self, _: &str) -> Result<(StatusCode, String), reqwest::Error> {
        panic!("No country HTML expected");
    }
    async fn fetch_bytes(
        &self,
        url: &str,
    ) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error> {
        self.urls.lock().expect("test lock").push(url.into());
        Ok((StatusCode::OK, HeaderMap::new(), self.bytes.clone()))
    }
}

#[tokio::test]
#[serial_test::serial]
async fn public_pipeline_merges_overrides_honors_refresh_and_emits_local_cover_flag() {
    let temp = tempfile::tempdir().expect("isolated runtime");
    let restore = RestoreEnvironment {
        cwd: std::env::current_dir().expect("original cwd"),
        flags_dir: std::env::var_os("STAMP_FLAGS_DIR"),
        albums_root: std::env::var_os("STAMP_ALBUMS_ROOT"),
    };
    std::env::set_current_dir(temp.path()).expect("isolated cwd");
    std::env::remove_var("STAMP_ALBUMS_ROOT");
    let root = temp.path().join("assets");
    let flags = root.join("flags");
    std::fs::create_dir_all(&flags).expect("flag directory");
    std::env::set_var("STAMP_FLAGS_DIR", &root);
    std::fs::write(
        flags.join("flag_codes.json"),
        r#"{"Test local country":"KR"}"#,
    )
    .expect("partial overrides");
    assert_eq!(
        stamp_album_gen2::flagdb::country_to_code("Test local country")
            .await
            .as_deref(),
        Some("kr")
    );
    assert_eq!(
        stamp_album_gen2::flagdb::country_to_code("South Korea")
            .await
            .as_deref(),
        Some("kr")
    );
    assert_eq!(
        stamp_album_gen2::flagdb::country_to_code("CA")
            .await
            .as_deref(),
        Some("ca")
    );

    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(4, 2, image::Rgba([255, 0, 0, 255]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .expect("fixture PNG");
    let fetcher = FakeFetcher {
        bytes: bytes.into_inner(),
        urls: Mutex::new(Vec::new()),
    };
    // ZZ is intentionally used only for the injected download API, not country resolution.
    let cached = flags.join("zz-w320.png");
    std::fs::write(&cached, &fetcher.bytes).expect("cached flag");
    std::fs::write("config.toml", "flag_cover_overwrite_flag = false\n").expect("config");
    let result = stamp_album_gen2::flagdb::ensure_flag_image_with_fetcher("zz", "w320", &fetcher)
        .await
        .expect("cache lookup");
    assert_eq!(PathBuf::from(result), cached);
    assert!(fetcher.urls.lock().expect("test lock").is_empty());
    std::fs::write("config.toml", "flag_cover_overwrite_flag = true\n").expect("refresh config");
    let result = stamp_album_gen2::flagdb::ensure_flag_image_with_fetcher("zz", "w320", &fetcher)
        .await
        .expect("refresh");
    assert_eq!(PathBuf::from(result), cached);
    assert_eq!(fetcher.urls.lock().expect("test lock").len(), 1);
    assert!(!temp.path().join("flags/zz-w320.png").exists());

    let images = temp.path().join("Custom Albums/Atlantis/images");
    std::fs::create_dir_all(&images).expect("album fixture");
    std::fs::write(images.join("flag_placeholder.png"), &fetcher.bytes).expect("local flag");
    std::fs::write(
        "config.toml",
        "bootstrap_albums_root_path = 'Custom Albums'\nprint_flag_cover = true\nflag_cover_elements = ['flag', 'name']\n",
    )
    .expect("cover config");
    let mut output = Vec::new();
    stamp_album_gen2::album_utils::globals::write_globals(
        &mut output,
        "Atlantis",
        None,
        None,
        None,
    )
    .await;
    let text = String::from_utf8(output).expect("album output");
    assert!(text.contains("# Cover flag image for album"));
    assert!(text.contains("images/flag_placeholder.png"));
    assert!(!text.contains("images/flag_placeholder.svg"));
    assert!(!temp.path().join("Albums/Atlantis").exists());

    std::fs::write(flags.join("flag_codes.json"), "invalid JSON").expect("bad override");
    assert_eq!(
        stamp_album_gen2::flagdb::country_to_code("South Korea")
            .await
            .as_deref(),
        Some("kr")
    );
    std::fs::write(
        "config.toml",
        "bootstrap_albums_root_path = 'Custom Albums'\nprint_flag_cover = true\nflag_cover_elements = ['name']\n",
    )
    .expect("name-only cover");
    let mut output = Vec::new();
    stamp_album_gen2::album_utils::globals::write_globals(
        &mut output,
        "Atlantis",
        None,
        None,
        None,
    )
    .await;
    assert!(
        !String::from_utf8(output)
            .expect("album output")
            .contains("# Cover flag image for album")
    );
    drop(restore);
}
