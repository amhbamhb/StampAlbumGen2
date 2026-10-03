---
type: Rust Function
title: colnect_pdf_options
resource: src/album_globals/selection.rs#L76-L96
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/load_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/normalized
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/selection/dynamic_options_use_resolved_album_records_and_keep_countries_separate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/selection_allows_country_without_reference_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn colnect_pdf_options(conn: &Connection) -> rusqlite::Result<ColnectPdfOptions>`

# Calls

- [load_stamps](../../../../functions/src/album_globals/selection/load_stamps.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [normalized](../../../../functions/src/album_globals/selection/normalized.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [dynamic_options_use_resolved_album_records_and_keep_countries_separate](../../../../functions/src/album_globals/selection/dynamic_options_use_resolved_album_records_and_keep_countries_separate.md)
- [selection_allows_country_without_reference_tables](../../../../functions/src/album_globals/selection/selection_allows_country_without_reference_tables.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)