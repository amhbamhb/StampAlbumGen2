/// Path validation, image checking, and PDF detection utilities for the Review UI.
use std::path::Path;

/// Validates that an image path is a real image path, not a placeholder/no_image
pub fn is_real_image_path(image_path: &str) -> bool {
    let p = image_path.trim();
    let lower = p.to_ascii_lowercase();
    !p.is_empty() && !lower.contains("no_image")
}

/// Checks if an image file exists for the given album and image path
pub fn local_image_file_exists(album_name: &str, image_path: &str) -> bool {
    use crate::file_utils::resolve_album_image_path;
    resolve_album_image_path(album_name, image_path).exists()
}

#[allow(dead_code)]
/// Validates that a file is a PDF by checking extension (case-insensitive)
pub fn is_pdf<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

/// Opens a local file with the default viewer/browser (platform-specific)
#[allow(dead_code)]
pub fn open_local_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    // Platform-specific implementation for opening files
    if path.is_file() {
        #[cfg(target_family = "unix")]
        {
            return std::process::Command::new("xdg-open")
                .arg(path)
                .spawn()
                .is_ok();
        }
        #[cfg(target_family = "windows")]
        {
            return std::process::Command::new("start")
                .arg(path)
                .spawn()
                .is_ok();
        }
    }
    false
}

/// Opens a folder in the file manager (platform-specific)
pub fn open_folder_in_file_manager<P: AsRef<Path>>(folder_path: P) -> bool {
    #[allow(dead_code)]
    let path = folder_path.as_ref();
    if path.is_dir() {
        #[cfg(target_family = "unix")]
        {
            return std::process::Command::new("xdg-open")
                .arg(path)
                .spawn()
                .is_ok();
        }
        #[cfg(target_family = "windows")]
        {
            return std::process::Command::new("start")
                .arg(path)
                .spawn()
                .is_ok();
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_real_image_path() {
        assert!(is_real_image_path("images/stamp.png"));
        assert!(is_real_image_path("./images/another-stamp.jpg"));
        assert!(!is_real_image_path("images/no_image.png"));
        assert!(!is_real_image_path("images/NO_IMAGE.PNG"));
        assert!(!is_real_image_path("no_image.png"));
        assert!(!is_real_image_path(""));
        assert!(!is_real_image_path("   "));
    }

    #[test]
    fn test_is_pdf() {
        let pdf = Path::new("document.pdf");
        assert!(is_pdf(pdf));

        let txt = Path::new("document.txt");
        assert!(!is_pdf(txt));

        let no_ext = Path::new("document");
        assert!(!is_pdf(no_ext));
    }

    #[test]
    fn test_is_pdf_extension_case_insensitive() {
        let pdf = Path::new("document.PDF");
        assert!(is_pdf(pdf));

        let pdf = Path::new("document.Pdf");
        assert!(is_pdf(pdf));

        let pdf = Path::new("Document.pdf");
        assert!(is_pdf(pdf));
    }

    #[test]
    fn test_is_pdf_with_directory_path() {
        let pdf = PathBuf::from("/tmp/test/document.pdf");
        assert!(is_pdf(pdf));
    }
}
