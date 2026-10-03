---
type: Rust Function
title: fetch_bytes
resource: src/stampworld/fetch.rs#L160-L169
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

`pub async fn fetch_bytes( client: &Client, url: &str, ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [status](../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)