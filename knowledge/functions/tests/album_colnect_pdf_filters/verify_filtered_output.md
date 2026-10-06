---
type: Rust Function
title: verify_filtered_output
resource: tests/album_colnect_pdf_filters.rs#L29-L165
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/album_colnect_pdf_filters/filtered_output_contains_only_selected_stamps_and_empty_selection_preserves_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_colnect_pdf_filters/filtered_output_generates_a_real_pdf
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn verify_filtered_output(generate_pdf: bool)`

# Calls

- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [effective_albumeasy_cli_path](../../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [with_defaults](../../../functions/src/config/Config/with_defaults.md)
- [save](../../../functions/src/review_ui/views/external_album/save.md)
- [generate_album_output_conn_with_filters](../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)

# Called by

- [filtered_output_contains_only_selected_stamps_and_empty_selection_preserves_output](../../../functions/tests/album_colnect_pdf_filters/filtered_output_contains_only_selected_stamps_and_empty_selection_preserves_output.md)
- [filtered_output_generates_a_real_pdf](../../../functions/tests/album_colnect_pdf_filters/filtered_output_generates_a_real_pdf.md)