use super::{Fetcher, countries};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

type AssetResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(super) fn flag_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(path) = std::env::var_os("STAMP_FLAGS_DIR") {
        let path = PathBuf::from(path);
        if path.join("flags").is_dir() && !path.join("flag_codes.json").is_file() {
            dirs.push(path.join("flags"));
        }
        dirs.push(path.clone());
        dirs.push(path.join("flags"));
    }
    dirs.push(PathBuf::from("flags"));
    if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent() {
            dirs.push(parent.join("flags"));
        }
    if let Ok(cwd) = std::env::current_dir()
        && let Some(parent) = cwd.parent() {
            dirs.push(parent.join("flags"));
        }
    dirs.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags"));
    dirs.dedup();
    dirs
}

fn validate(bytes: &[u8], extension: &str) -> AssetResult<()> {
    match extension {
        "png" => {
            image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;
        }
        "svg" => {
            usvg::Tree::from_data(bytes, &usvg::Options::default())?;
        }
        _ => return Err(format!("Unsupported flag image type: {extension}").into()),
    }
    Ok(())
}

fn valid_file(path: &Path) -> AssetResult<()> {
    let bytes = fs::read(path)?;
    validate(
        &bytes,
        path.extension().and_then(|ext| ext.to_str()).unwrap_or(""),
    )
}

fn write_atomic(path: &Path, bytes: &[u8]) -> AssetResult<()> {
    let parent = path.parent().ok_or("Flag image path has no parent")?;
    fs::create_dir_all(parent)?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    file.persist(path)?;
    Ok(())
}

fn rasterize(bytes: &[u8], size: &str) -> AssetResult<Vec<u8>> {
    let tree = usvg::Tree::from_data(bytes, &usvg::Options::default())?;
    let pixels: u32 = size[1..].parse()?;
    let scale = if size.starts_with('w') {
        pixels as f32 / tree.size().width()
    } else {
        pixels as f32 / tree.size().height()
    };
    let width = (tree.size().width() * scale).ceil() as u32;
    let height = (tree.size().height() * scale).ceil() as u32;
    if width == 0 || height == 0 || width > 4096 || height > 4096 {
        return Err("Flag SVG dimensions exceed the supported range".into());
    }
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("Cannot allocate flag image")?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    Ok(pixmap.encode_png()?)
}

pub(super) async fn ensure_flag(
    alpha: &str,
    size: &str,
    fetcher: &dyn Fetcher,
    directories: &[PathBuf],
    overwrite: bool,
) -> AssetResult<String> {
    let alpha = alpha.trim().to_lowercase();
    if !countries::valid_code(&alpha)
        || !matches!(size.as_bytes().first(), Some(b'w' | b'h'))
        || !size.get(1..).is_some_and(|value| {
            !value.is_empty()
                && value.bytes().all(|byte| byte.is_ascii_digit())
                && value
                    .parse::<u32>()
                    .is_ok_and(|pixels| (1..=4096).contains(&pixels))
        })
    {
        return Err("Flag lookup requires an alpha-2 code and size w1..w4096 or h1..h4096".into());
    }
    if !overwrite {
        for dir in directories {
            for name in [
                format!("{alpha}-{size}.png"),
                format!("{alpha}-{size}.svg"),
                format!("{alpha}.png"),
                format!("{alpha}.svg"),
            ] {
                let path = dir.join(name);
                match valid_file(&path) {
                    Ok(()) => {
                        if path.extension().is_some_and(|ext| ext == "svg") {
                            let png = path.with_file_name(format!("{alpha}-{size}.png"));
                            let result = fs::read(&path)
                                .map_err(Into::into)
                                .and_then(|bytes| rasterize(&bytes, size))
                                .and_then(|bytes| write_atomic(&png, &bytes));
                            match result {
                                Ok(()) => return Ok(png.to_string_lossy().to_string()),
                                Err(error) => log::warn!(
                                    "Cannot rasterize cached flag {}: {error}",
                                    path.display()
                                ),
                            }
                        } else {
                            return Ok(path.to_string_lossy().to_string());
                        }
                    }
                    Err(error) => {
                        if !is_missing(error.as_ref()) {
                            log::warn!("Ignoring invalid cached flag {}: {error}", path.display());
                        }
                    }
                }
            }
        }
    }

    let mut target = None;
    for dir in directories {
        match fs::create_dir_all(dir).and_then(|_| tempfile::NamedTempFile::new_in(dir)) {
            Ok(_) => {
                target = Some(dir);
                break;
            }
            Err(error) => log::warn!("Cannot write flag cache {}: {error}", dir.display()),
        }
    }
    let target = target.ok_or("No writable flag cache directory")?;
    let path = target.join(format!("{alpha}-{size}.png"));
    let sources = [
        (
            format!("https://countries.nexperts.nl/flags/{alpha}.png"),
            "png",
        ),
        (
            format!("https://countries.nexperts.nl/flags/{alpha}.svg"),
            "svg",
        ),
        (format!("https://flagcdn.com/{size}/{alpha}.png"), "png"),
    ];
    let mut failures = Vec::new();
    for (url, extension) in sources {
        let attempt = async {
            let (status, _headers, bytes) = fetcher.fetch_bytes(&url).await?;
            if !status.is_success() {
                return Err(format!("HTTP {status}").into());
            }
            validate(&bytes, extension)?;
            let bytes = if extension == "svg" {
                rasterize(&bytes, size)?
            } else {
                bytes
            };
            validate(&bytes, "png")?;
            Ok::<_, Box<dyn std::error::Error>>(bytes)
        }
        .await;
        match attempt {
            Ok(bytes) => {
                write_atomic(&path, &bytes)?;
                return Ok(path.to_string_lossy().to_string());
            }
            Err(error) => {
                log::warn!("Flag source failed {url}: {error}");
                failures.push(format!("{url}: {error}"));
            }
        }
    }
    Err(format!("No valid flag for {alpha}: {}", failures.join("; ")).into())
}

pub(super) fn local_album_flag(dir: &Path) -> Option<PathBuf> {
    for name in ["flag_placeholder.png", "flag_placeholder.svg"] {
        let path = dir.join(name);
        match valid_file(&path) {
            Ok(()) => return Some(path),
            Err(error) => {
                if !is_missing(error.as_ref()) {
                    log::warn!("Invalid album-local flag {}: {error}", path.display());
                }
            }
        }
    }
    None
}

fn is_missing(error: &(dyn std::error::Error + 'static)) -> bool {
    error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound)
}

pub(super) fn copy_to_album(source: &Path, dir: &Path) -> AssetResult<String> {
    valid_file(source)?;
    let extension = source
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or("Flag extension missing")?;
    let path = dir.join(format!("flag_placeholder.{extension}"));
    write_atomic(&path, &fs::read(source)?)?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use reqwest::{StatusCode, header::HeaderMap};
    use std::sync::Mutex;

    struct FakeFetcher {
        replies: Vec<(StatusCode, Vec<u8>)>,
        urls: Mutex<Vec<String>>,
    }

    impl FakeFetcher {
        fn new(replies: Vec<(StatusCode, Vec<u8>)>) -> Self {
            Self {
                replies,
                urls: Mutex::new(Vec::new()),
            }
        }
        fn requests(&self) -> Vec<String> {
            self.urls.lock().expect("test lock").clone()
        }
    }

    #[async_trait]
    impl Fetcher for FakeFetcher {
        async fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
            panic!("Flag downloads must not request country HTML");
        }
        async fn fetch_bytes(
            &self,
            url: &str,
        ) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error> {
            let mut urls = self.urls.lock().expect("test lock");
            let response = self
                .replies
                .get(urls.len())
                .cloned()
                .unwrap_or((StatusCode::NOT_FOUND, Vec::new()));
            urls.push(url.into());
            Ok((response.0, HeaderMap::new(), response.1))
        }
    }

    fn png(color: u8) -> Vec<u8> {
        let image = image::RgbaImage::from_pixel(4, 2, image::Rgba([color, 0, 0, 255]));
        let mut bytes = std::io::Cursor::new(Vec::new());
        image
            .write_to(&mut bytes, image::ImageFormat::Png)
            .expect("fixture PNG");
        bytes.into_inner()
    }

    const SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect width="20" height="10" fill="red"/></svg>"#;

    #[tokio::test]
    async fn failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors() {
        for bad in [
            (StatusCode::NOT_FOUND, b"<html>Not found</html>".to_vec()),
            (StatusCode::NOT_FOUND, png(120)),
            (StatusCode::OK, b"<html>Error</html>".to_vec()),
            (StatusCode::OK, Vec::new()),
            (StatusCode::OK, SVG.to_vec()),
        ] {
            let temp = tempfile::tempdir().expect("test directory");
            let good = png(120);
            let fetcher = FakeFetcher::new(vec![
                bad,
                (StatusCode::NOT_FOUND, Vec::new()),
                (StatusCode::OK, good.clone()),
            ]);
            let result = ensure_flag("KR", "w320", &fetcher, &[temp.path().into()], false)
                .await
                .expect("CDN fallback");
            assert_eq!(fs::read(result).expect("saved flag"), good);
            assert_eq!(
                fetcher.requests(),
                [
                    "https://countries.nexperts.nl/flags/kr.png",
                    "https://countries.nexperts.nl/flags/kr.svg",
                    "https://flagcdn.com/w320/kr.png",
                ]
            );
            assert_eq!(fs::read_dir(temp.path()).expect("files").count(), 1);
        }
    }

    #[tokio::test]
    async fn valid_svg_is_rendered_to_a_real_png_at_the_requested_size() {
        let temp = tempfile::tempdir().expect("test directory");
        let fetcher = FakeFetcher::new(vec![
            (StatusCode::NOT_FOUND, Vec::new()),
            (StatusCode::OK, SVG.to_vec()),
        ]);
        let result = ensure_flag("kr", "w320", &fetcher, &[temp.path().into()], false)
            .await
            .expect("SVG fallback");
        let image = image::open(&result).expect("real PNG");
        assert_eq!(image.width(), 320);
        assert_eq!(image.height(), 160);
        assert_eq!(fetcher.requests().len(), 2);
        let album = temp.path().join("album");
        let copied = copy_to_album(Path::new(&result), &album).expect("album copy");
        assert!(copied.ends_with("flag_placeholder.png"));
        image::open(copied).expect("copied real PNG");
    }

    #[tokio::test]
    async fn valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes() {
        let temp = tempfile::tempdir().expect("test directory");
        let dirs = [temp.path().into()];
        let path = temp.path().join("kr-w320.png");
        fs::write(&path, png(10)).expect("cached flag");
        let no_network = FakeFetcher::new(Vec::new());
        ensure_flag("kr", "w320", &no_network, &dirs, false)
            .await
            .expect("cached");
        assert!(no_network.requests().is_empty());
        let refreshed = png(20);
        let overwrite = FakeFetcher::new(vec![(StatusCode::OK, refreshed.clone())]);
        ensure_flag("kr", "w320", &overwrite, &dirs, true)
            .await
            .expect("refresh");
        assert_eq!(overwrite.requests().len(), 1);
        assert_eq!(fs::read(&path).expect("updated"), refreshed);
        fs::write(&path, "<html>old cached error</html>").expect("bad cache");
        let repair = FakeFetcher::new(vec![(StatusCode::OK, png(30))]);
        ensure_flag("kr", "w320", &repair, &dirs, false)
            .await
            .expect("repair");
        valid_file(&path).expect("repaired image");
        assert_eq!(repair.requests().len(), 1);
    }

    #[tokio::test]
    async fn failed_refresh_preserves_good_cache_and_no_error_body_is_written() {
        let temp = tempfile::tempdir().expect("test directory");
        let path = temp.path().join("kr-w320.png");
        let original = png(50);
        fs::write(&path, &original).expect("cache");
        let fetcher = FakeFetcher::new(Vec::new());
        let error = ensure_flag("kr", "w320", &fetcher, &[temp.path().into()], true)
            .await
            .expect_err("sources missing");
        assert!(error.to_string().contains("HTTP 404"));
        assert_eq!(fs::read(&path).expect("original preserved"), original);
        fs::remove_file(&path).expect("remove fixture");
        assert!(
            ensure_flag("kr", "w320", &fetcher, &[temp.path().into()], false)
                .await
                .is_err()
        );
        assert_eq!(fs::read_dir(temp.path()).expect("files").count(), 0);
    }

    #[tokio::test]
    async fn cached_svg_is_supported_and_invalid_inputs_do_not_access_network() {
        let temp = tempfile::tempdir().expect("test directory");
        fs::write(temp.path().join("kr.svg"), SVG).expect("SVG cache");
        let fetcher = FakeFetcher::new(Vec::new());
        let result = ensure_flag("kr", "w80", &fetcher, &[temp.path().into()], false)
            .await
            .expect("cached SVG");
        assert_eq!(image::open(result).expect("PNG").width(), 80);
        for (code, size) in [
            ("../us", "w320"),
            ("xyz", "w320"),
            ("us", "../320"),
            ("us", "w0"),
            ("us", "w99999"),
        ] {
            assert!(
                ensure_flag(code, size, &fetcher, &[temp.path().into()], false)
                    .await
                    .is_err()
            );
        }
        assert!(fetcher.requests().is_empty());
    }

    #[test]
    fn album_local_fallback_checks_real_images_and_preserves_svg_extension() {
        let temp = tempfile::tempdir().expect("test directory");
        let album = temp.path().join("album");
        fs::create_dir(&album).expect("album");
        fs::write(album.join("flag_placeholder.png"), "<html>invalid</html>").expect("invalid");
        assert!(local_album_flag(&album).is_none());
        let source = temp.path().join("kr.svg");
        fs::write(&source, SVG).expect("SVG");
        let result = copy_to_album(&source, &album).expect("copy");
        assert!(result.ends_with("flag_placeholder.svg"));
        assert_eq!(
            local_album_flag(&album).expect("valid local SVG"),
            PathBuf::from(result)
        );
    }
}
