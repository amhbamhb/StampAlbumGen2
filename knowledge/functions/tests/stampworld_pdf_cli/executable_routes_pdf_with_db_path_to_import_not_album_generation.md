---
type: Rust Function
title: executable_routes_pdf_with_db_path_to_import_not_album_generation
resource: tests/stampworld_pdf_cli.rs#L56-L97
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn executable_routes_pdf_with_db_path_to_import_not_album_generation()`

# Calls

- [with_defaults](../../../functions/src/config/Config/with_defaults.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [open](../../../functions/src/external_album/open.md)