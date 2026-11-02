---
type: Rust Module
title: pdf_extraction_test
resource: tests/pdf_extraction_test.rs#L1-L124
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-review-ui-pdf-parser-extraction-parse-stampworld-text
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [stampworld_pdf_extraction_layout](../../functions/tests/pdf_extraction_test/stampworld_pdf_extraction_layout.md)
- [canonical_html_preserves_individual_rows_and_metadata](../../functions/tests/pdf_extraction_test/canonical_html_preserves_individual_rows_and_metadata.md)
- [all_supplied_stampworld_pdfs_have_individual_rows](../../functions/tests/pdf_extraction_test/all_supplied_stampworld_pdfs_have_individual_rows.md)
- [supplied_denmark_pages_have_266_individual_stamps](../../functions/tests/pdf_extraction_test/supplied_denmark_pages_have_266_individual_stamps.md)
- [malformed_rows_and_missing_headings_are_errors_not_empty_successes](../../functions/tests/pdf_extraction_test/malformed_rows_and_missing_headings_are_errors_not_empty_successes.md)

# Imports

- `stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_text`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)