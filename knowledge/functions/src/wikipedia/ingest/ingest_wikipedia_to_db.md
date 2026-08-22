---
type: Rust Function
title: ingest_wikipedia_to_db
resource: src/wikipedia/ingest.rs#L11-L153
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/get_cached_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ingest_wikipedia_to_db( topic: &str, cache_db_path: &str, album_db_path: &str, max_entries: usize, series_prefix: &str, ) -> Result<usize, Box<dyn Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [get_cached_content](../../../../functions/src/wikipedia/database/get_cached_content.md)

# Called by

- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [wikipedia_ingest_inserts_up_to_max_entries_from_cache](../../../../functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache.md)