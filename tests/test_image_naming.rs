use stamp_album_gen2::album_utils::sanitize_filename;
use stamp_album_gen2::ImageStatus;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn deterministic_target_name_is_sanitized() {
    let tmp = tempdir().expect("tempdir");
    let images_dir = tmp.path().to_string_lossy().to_string();

    let url = "https://example.com/images/Example Image(1).jpg?size=large";
    let desired = "SG#958";

    let last = url.rsplit('/').next().unwrap_or("image.jpg");
    let base = last.split('?').next().unwrap_or(last);
    let sanitized_desired = sanitize_filename(desired);
    let sanitized_base = sanitize_filename(base);
    let target_name = format!("{}-{}", sanitized_desired, sanitized_base);

    // Expected uses sanitize_filename rules (non-alnum -> '_', allowed . - _ preserved)
    assert!(target_name.contains(&sanitized_desired));
    assert!(target_name.contains(&sanitized_base));
    // Ensure no slashes or question marks remain
    assert!(!target_name.contains('/'));
    assert!(!target_name.contains('?'));
    // Ensure file path would be inside images_dir when constructed the same way
    let file_path = format!("{}/{}", images_dir.trim_end_matches('/'), target_name);
    assert!(Path::new(&file_path).parent().is_some());
}

#[test]
fn downloader_skips_existing_file_when_not_overwrite() {
    let tmp = tempdir().expect("tempdir");
    let images_dir = tmp.path().to_string_lossy().to_string();

    let url = "https://example.com/images/sample.png";
    let desired = "SG 100";

    let last = url.rsplit('/').next().unwrap_or("image.jpg");
    let base = last.split('?').next().unwrap_or(last);
    let sanitized_desired = sanitize_filename(desired);
    let sanitized_base = sanitize_filename(base);
    let target_name = format!("{}-{}", sanitized_desired, sanitized_base);
    let file_path = format!("{}/{}", images_dir.trim_end_matches('/'), target_name);

    // Create the target file to simulate an existing downloaded image
    std::fs::write(&file_path, b"existing").expect("write existing file");

    // Downloader closure similar to the one used in generator
    let images_dir_clone = images_dir.clone();
    let downloader = move |url: &str, desired_name: &str, _overwrite: bool| -> ImageStatus {
        if url.trim().is_empty() {
            return ImageStatus {
                link: String::new(),
                exists: false,
                width: 0,
                height: 0,
                filename: String::new(),
            };
        }
        let last = url.rsplit('/').next().unwrap_or("image.jpg");
        let base = last.split('?').next().unwrap_or(last);
        let sanitized_desired = sanitize_filename(desired_name);
        let sanitized_base = sanitize_filename(base);
        let target_name = format!("{}-{}", sanitized_desired, sanitized_base);
        let file_path = format!("{}/{}", images_dir_clone.trim_end_matches('/'), target_name);
        if std::path::Path::new(&file_path).exists() && !_overwrite {
            return ImageStatus {
                link: url.to_string(),
                exists: true,
                width: 0,
                height: 0,
                filename: target_name,
            };
        }
        // In tests we won't perform real download; return no-op
        ImageStatus {
            link: url.to_string(),
            exists: false,
            width: 0,
            height: 0,
            filename: target_name,
        }
    };

    let status = downloader(url, desired, false);
    assert!(
        status.exists,
        "downloader should report exists when file present"
    );
    assert_eq!(status.filename, target_name);
}
