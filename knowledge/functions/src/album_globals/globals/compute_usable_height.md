---
type: Rust Function
title: compute_usable_height
resource: src/album_globals/globals.rs#L19-L44
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_globals/compute_usable_height_basic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/usable_height/default_usable_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/usable_height/custom_padding_and_margins
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_usable_height(cfg: &Config) -> f64`

# Calls

- [with_defaults](../../../../functions/src/config/Config/with_defaults.md)

# Called by

- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [new_row](../../../../functions/src/album_globals/page_functions/new_row.md)
- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)
- [compute_usable_height_basic](../../../../functions/tests/album_globals/compute_usable_height_basic.md)
- [default_usable_height](../../../../functions/tests/usable_height/default_usable_height.md)
- [custom_padding_and_margins](../../../../functions/tests/usable_height/custom_padding_and_margins.md)