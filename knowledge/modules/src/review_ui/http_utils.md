---
type: Rust Module
title: http_utils
resource: src/review_ui/http_utils.rs#L1-L119
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-file-utils-resolve-album-image-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [is_real_image_path](../../../functions/src/review_ui/http_utils/is_real_image_path.md)
- [local_image_file_exists](../../../functions/src/review_ui/http_utils/local_image_file_exists.md)
- [is_pdf](../../../functions/src/review_ui/http_utils/is_pdf.md)
- [open_local_path](../../../functions/src/review_ui/http_utils/open_local_path.md)
- [open_folder_in_file_manager](../../../functions/src/review_ui/http_utils/open_folder_in_file_manager.md)
- [test_is_real_image_path](../../../functions/src/review_ui/http_utils/test_is_real_image_path.md)
- [test_is_pdf](../../../functions/src/review_ui/http_utils/test_is_pdf.md)
- [test_is_pdf_extension_case_insensitive](../../../functions/src/review_ui/http_utils/test_is_pdf_extension_case_insensitive.md)
- [test_is_pdf_with_directory_path](../../../functions/src/review_ui/http_utils/test_is_pdf_with_directory_path.md)

# Imports

- `std::path::Path`
- `crate::file_utils::resolve_album_image_path`
- `super::*`
- `std::path::PathBuf`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)