---
type: Rust Module
title: pdf_import_tool
resource: tests/pdf_import_tool.rs#L1-L217
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-review-ui-db-query-process-pdf-records
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-review-ui-pdf-parser-import-stampworld-file-types-pdfstamprecord
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [fixture](../../functions/tests/pdf_import_tool/fixture.md)
- [sample_record](../../functions/tests/pdf_import_tool/sample_record.md)
- [test_manual_import_single_record](../../functions/tests/pdf_import_tool/test_manual_import_single_record.md)
- [imports_real_pdfs_into_disk_database_and_skips_reimports](../../functions/tests/pdf_import_tool/imports_real_pdfs_into_disk_database_and_skips_reimports.md)
- [imports_denmark_pages_and_skips_all_266_on_second_pass](../../functions/tests/pdf_import_tool/imports_denmark_pages_and_skips_all_266_on_second_pass.md)
- [duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata](../../functions/tests/pdf_import_tool/duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata.md)
- [insert_errors_roll_back_the_entire_file_and_are_not_counted_as_duplicates](../../functions/tests/pdf_import_tool/insert_errors_roll_back_the_entire_file_and_are_not_counted_as_duplicates.md)
- [filters_years_imports_html_and_reports_invalid_inputs](../../functions/tests/pdf_import_tool/filters_years_imports_html_and_reports_invalid_inputs.md)

# Imports

- `stamp_album_gen2::review_ui::db_query::process_pdf_records`
- `stamp_album_gen2::review_ui::pdf_parser::{import_stampworld_file, types::PdfStampRecord}`
- `std::path::{Path, PathBuf}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)