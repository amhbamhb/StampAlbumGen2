---
type: Rust Module
title: album_colnect_pdf_filters
resource: tests/album_colnect_pdf_filters.rs#L1-L165
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-album-globals-generate-album-output-conn-with-filters-generate-album-output-conn-with-pdf-override-selection-colnectpdffilters
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-config-config-db-utils-init-db
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [RestoreWorkingDirectory](../../classes/tests/album_colnect_pdf_filters/RestoreWorkingDirectory.md)
- [drop](../../functions/tests/album_colnect_pdf_filters/RestoreWorkingDirectory/drop/drop.md)
- [filtered_output_contains_only_selected_stamps_and_empty_selection_preserves_output](../../functions/tests/album_colnect_pdf_filters/filtered_output_contains_only_selected_stamps_and_empty_selection_preserves_output.md)
- [filtered_output_generates_a_real_pdf](../../functions/tests/album_colnect_pdf_filters/filtered_output_generates_a_real_pdf.md)
- [verify_filtered_output](../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)

# Imports

- `stamp_album_gen2::album_globals::{
    generate_album_output_conn_with_filters, generate_album_output_conn_with_pdf_override,
    selection::ColnectPdfFilters,
}`
- `stamp_album_gen2::{config::Config, db_utils::init_db}`
- `std::{fs, path::PathBuf}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)