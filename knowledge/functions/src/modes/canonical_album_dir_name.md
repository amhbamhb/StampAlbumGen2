---
type: Rust Function
title: canonical_album_dir_name
resource: src/modes.rs#L39-L41
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/country_slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/move_to_country_inbox
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_stampworld_file_import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn canonical_album_dir_name(name: &str) -> String`

# Called by

- [country_slug](../../../functions/src/modes/country_slug.md)
- [move_to_country_inbox](../../../functions/src/modes/move_to_country_inbox.md)
- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../functions/src/modes/run_database_mode.md)
- [run_stampworld_file_import](../../../functions/src/modes/run_stampworld_file_import.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)