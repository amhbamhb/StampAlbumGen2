---
type: Rust Function
title: page_text_height
resource: src/album_globals/generator_new/layout.rs#L111-L135
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/ColnectPdfFilters/matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn page_text_height(line: &str, fallback: f64) -> f64`

# Calls

- [matches](../../../../../functions/src/album_globals/selection/ColnectPdfFilters/matches.md)
- [count](../../../../../functions/src/album_globals/selection/count.md)

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)