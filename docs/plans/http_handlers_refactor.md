# Plan: Break http_handlers.rs into Separate Modules (Phase 1)

## Overview

The `src/review_ui/http_handlers.rs` file is 2915 lines and contains mixed responsibilities. This plan breaks it down into separate modules following the Single Responsibility Principle:

| Module | Purpose | Lines | Status |
| -------- | --------- | ------- | -------- |
| `http_utils.rs` | Path validation, file operations, sanitization | ~300 | Planned |
| `db_query.rs` | Database queries and stamp stub generation | ~450 | Planned |
| `pdf_generation.rs` | PDF output generation logic | ~400 | Planned |
| `ui_state.rs` | UI state management (reload, shutdown) | ~250 | Planned |
| `config_api.rs` | Config manipulation endpoints | ~350 | Planned |

## Phase 1: Create http_utils.rs with Path Validation & File Utilities

### Plan for Module: http_utils.rs

**Objective:** Extract all path-related utilities and file validation logic from http_handlers.rs into a new module.

#### Step 1a: Identify functions to extract

- `resolve_album_image_path` - Already in file_utils, but verify usage pattern
- `is_real_image_path` - Image path validation
- `local_image_file_exists` - File existence checks
- Any path sanitization logic
- File extension validation (PDF check)

#### Step 1b: Create http_utils.rs with extracted functions

```rust
// src/review_ui/http_utils.rs
/// Validates that an image path is a real image path, not a placeholder/no_image
pub fn is_real_image_path(image_path: &str) -> bool {
    let p = image_path.trim();
    !p.is_empty() && !p.to_ascii_lowercase().starts_with("images/no_image")
}

/// Checks if an image file exists for the given album and image path
pub fn local_image_file_exists(album_name: &str, image_path: &str) -> bool {
    use crate::file_utils::resolve_album_image_path;
    resolve_album_image_path(album_name, image_path).exists()
}

/// Validates that a file is a PDF by checking extension
pub fn is_pdf(path: std::path::Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

/// Opens a local file with the default viewer/browser
pub fn open_local_path(path: std::path::Path) -> bool {
    // Implementation details...
    true  // Placeholder - implement platform-specific file opening
}

/// Opens a folder in the file manager
pub fn open_folder_in_file_manager(folder_str: &str) -> bool {
    // Platform-specific implementation
    true  // Placeholder
}
```

#### Step 1c: Create unit tests for http_utils.rs

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real_image_path() {
        assert!(is_real_image_path("images/stamp.png"));
        assert!(!is_real_image_path("images/no_image.png"));
        assert!(!is_real_image_path("images/NO_IMAGE.PNG"));
        assert!(!is_real_image_path("no_image.png"));
        assert!(!is_real_image_path(""));
    }

    #[test]
    fn test_is_pdf() {
        use std::path::Path;
        let pdf = Path::new("document.pdf");
        assert!(is_pdf(pdf));
        
        let txt = Path::new("document.txt");
        assert!(!is_pdf(txt));
    }

    #[test]
    fn test_is_pdf_extension_case_insensitive() {
        use std::path::Path;
        let pdf = Path::new("document.PDF");
        assert!(is_pdf(pdf));
        
        let pdf = Path::new("document.Pdf");
        assert!(is_pdf(pdf));
    }
}
```

#### Step 1d: Update http_handlers.rs to import from http_utils.rs

- Replace all inline path validation functions with imports from `http_utils`
- Remove duplicate code
