---
type: Rust Module
title: db_query
resource: src/review_ui/db_query.rs#L1-L90
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-db-utils-insert-stamp-from-struct-stampinsert
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-http-utils-is-real-image-path-local-image-file-exists
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-pdf-parser-types-importsummary-pdfstamprecord
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [best_effort_prepare_images_for_pdf](../../../functions/src/review_ui/db_query/best_effort_prepare_images_for_pdf.md)
- [is_duplicate](../../../functions/src/review_ui/db_query/is_duplicate.md)
- [process_pdf_records](../../../functions/src/review_ui/db_query/process_pdf_records.md)

# Imports

- `crate::db_utils::{insert_stamp_from_struct, StampInsert}`
- `crate::review_ui::http_utils::{is_real_image_path, local_image_file_exists}`
- `crate::review_ui::pdf_parser::types::{ImportSummary, PdfStampRecord}`
- `rusqlite::{params, Connection}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)