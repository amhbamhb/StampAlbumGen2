---
type: Rust Module
title: manual_import_template
resource: tests/manual_import_template.rs#L1-L174
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-colnect-ingest-ingest-colnect-data-with-mode-parser-parse-csv-template-ensure-manual-import-files-template-csv-template-guide
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [creates_missing_files_and_preserves_existing_copies](../../functions/tests/manual_import_template/creates_missing_files_and_preserves_existing_copies.md)
- [reports_conflicting_template_path_without_overwriting_it](../../functions/tests/manual_import_template/reports_conflicting_template_path_without_overwriting_it.md)
- [manual_rows_parse_and_replace_only_the_same_staged_identity](../../functions/tests/manual_import_template/manual_rows_parse_and_replace_only_the_same_staged_identity.md)
- [documented_example_is_accepted_by_the_existing_parser](../../functions/tests/manual_import_template/documented_example_is_accepted_by_the_existing_parser.md)

# Imports

- `stamp_album_gen2::colnect::{
    ingest::ingest_colnect_data_with_mode,
    parser::parse_csv,
    template::{ensure_manual_import_files, TEMPLATE_CSV, TEMPLATE_GUIDE},
}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)