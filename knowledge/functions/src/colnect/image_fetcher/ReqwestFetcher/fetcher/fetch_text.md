---
type: Rust Method
title: fetch_text
resource: src/colnect/image_fetcher.rs#L115-L120
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn fetch_text(&self, url: &str) -> Result<(StatusCode, String), reqwest::Error>`

# Calls

- [get](../../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [status](../../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)