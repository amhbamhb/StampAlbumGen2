---
type: Rust Function
title: fetch_and_cache_content
resource: src/wikipedia/database.rs#L6-L129
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/fetchers/fetch_grokipedia_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/fetchers/fetch_wikipedia_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/legacy/fetch_and_cache_wikipedia_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fetch_and_cache_content( topic: &str, cache_db_path: &str, fetcher: &dyn crate::stampworld::fetch::Fetcher, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [fetch_grokipedia_content](../../../../functions/src/wikipedia/fetchers/fetch_grokipedia_content.md)
- [fetch_wikipedia_content](../../../../functions/src/wikipedia/fetchers/fetch_wikipedia_content.md)

# Called by

- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [fetch_and_cache_wikipedia_content](../../../../functions/src/wikipedia/legacy/fetch_and_cache_wikipedia_content.md)