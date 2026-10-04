---
type: Rust Method
title: with_defaults
resource: src/config.rs#L1011-L1201
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/centering_preserves_explicit_spacing_and_is_idempotent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/full_pages_covers_and_disabled_centering_get_no_extra_space
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/globals/compute_usable_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/albumeasy_pdf_timeout_album_runtime_override_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_colnect_pdf_filters/verify_filtered_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/test_wikipedia_vspace/paragraph_vspace_uses_configured_value
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_defaults() -> Self`

# Called by

- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [centering_preserves_explicit_spacing_and_is_idempotent](../../../../functions/src/album_globals/generator_new/postprocess/centering_preserves_explicit_spacing_and_is_idempotent.md)
- [full_pages_covers_and_disabled_centering_get_no_extra_space](../../../../functions/src/album_globals/generator_new/postprocess/full_pages_covers_and_disabled_centering_get_no_extra_space.md)
- [compute_usable_height](../../../../functions/src/album_globals/globals/compute_usable_height.md)
- [load_config](../../../../functions/src/config/load_config.md)
- [albumeasy_pdf_timeout_album_runtime_override_wins](../../../../functions/src/config/albumeasy_pdf_timeout_album_runtime_override_wins.md)
- [main](../../../../functions/src/main.md)
- [verify_filtered_output](../../../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)
- [paragraph_vspace_uses_configured_value](../../../../functions/tests/test_wikipedia_vspace/paragraph_vspace_uses_configured_value.md)