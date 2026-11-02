---
type: Rust Function
title: cli_import_targets_explicit_db_instead_of_output_only_mode
resource: tests/stampworld_pdf_cli.rs#L6-L30
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/run_stampworld_file_import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cli_import_targets_explicit_db_instead_of_output_only_mode()`

# Calls

- [run_stampworld_file_import](../../../functions/src/modes/run_stampworld_file_import.md)
- [open](../../../functions/src/external_album/open.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)