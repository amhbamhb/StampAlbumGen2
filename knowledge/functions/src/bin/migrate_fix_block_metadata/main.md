---
type: Rust Function
title: main
resource: src/bin/migrate_fix_block_metadata.rs#L4-L39
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn main()`

# Calls

- [init](../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/init.md)
- [args](../../../../functions/tests/native_pdf_page_settings/args.md)
- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)
- [open](../../../../functions/src/external_album/open.md)