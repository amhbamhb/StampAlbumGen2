---
type: Rust Method
title: with_defaults
resource: src/config.rs#L827-L1010
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/compute_page_used_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/compute_page_used_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/globals/compute_usable_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/test_wikipedia_vspace/paragraph_vspace_uses_configured_value
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_defaults() -> Self`

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [compute_page_used_height](../../../../functions/src/album_globals/generator/compute_page_used_height.md)
- [compute_page_used_height](../../../../functions/src/album_globals/generator_new/postprocess/compute_page_used_height.md)
- [compute_usable_height](../../../../functions/src/album_globals/globals/compute_usable_height.md)
- [load_config](../../../../functions/src/config/load_config.md)
- [main](../../../../functions/src/main.md)
- [paragraph_vspace_uses_configured_value](../../../../functions/tests/test_wikipedia_vspace/paragraph_vspace_uses_configured_value.md)