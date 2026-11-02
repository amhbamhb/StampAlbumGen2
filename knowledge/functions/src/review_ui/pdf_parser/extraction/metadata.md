---
type: Rust Function
title: metadata
resource: src/review_ui/pdf_parser/extraction.rs#L165-L192
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/wait_for_generated_pdf_with_timing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/wait_for_generated_pdf_rejects_missing_empty_and_stale_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/wait_for_generated_pdf_detects_delayed_new_write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/template/ensure_manual_import_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/album_creation/save_stamp_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/slug_override_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/init/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/record_import_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/tail_current_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/stampworld_db_snapshot
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn metadata(text: &str, record: &mut PdfStampRecord) -> Result<(), String>`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [wait_for_generated_pdf_with_timing](../../../../../functions/src/album_globals/wait_for_generated_pdf_with_timing.md)
- [maybe_generate_album_pdf_with_settings](../../../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)
- [generate_album_output_conn_with_pdf_override](../../../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [generate_album_output](../../../../../functions/src/album_globals/generate_album_output.md)
- [wait_for_generated_pdf_rejects_missing_empty_and_stale_files](../../../../../functions/src/album_globals/wait_for_generated_pdf_rejects_missing_empty_and_stale_files.md)
- [wait_for_generated_pdf_detects_delayed_new_write](../../../../../functions/src/album_globals/wait_for_generated_pdf_detects_delayed_new_write.md)
- [ensure_manual_import_files](../../../../../functions/src/colnect/template/ensure_manual_import_files.md)
- [save_stamp_data](../../../../../functions/src/commands_file_generator/album_creation/save_stamp_data.md)
- [slug_override_path](../../../../../functions/src/config/slug_override_path.md)
- [init_db](../../../../../functions/src/db_utils/init/init_db.md)
- [verify_source](../../../../../functions/src/external_album/verify_source.md)
- [render](../../../../../functions/src/external_album/render.md)
- [record_import_history](../../../../../functions/src/modes/record_import_history.md)
- [tail_current_log](../../../../../functions/src/review_ui/tail_current_log.md)
- [stampworld_db_snapshot](../../../../../functions/src/review_ui/stampworld_db_snapshot.md)
- [parse_stampworld_text](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text.md)
- [parse_stampworld_html](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html.md)