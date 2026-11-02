---
type: Rust Module
title: report_generation
resource: src/review_ui/report_generation.rs#L1-L74
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-db-utils-stampstub
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [StampGroup](../../../classes/src/review_ui/report_generation/StampGroup.md)
- [generate_pdf_report](../../../functions/src/review_ui/report_generation/generate_pdf_report.md)
- [generate_album_review_pdf](../../../functions/src/review_ui/report_generation/generate_album_review_pdf.md)

# Imports

- `crate::db_utils::StampStub`
- `std::fs::File`
- `std::io::{self, Write}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)