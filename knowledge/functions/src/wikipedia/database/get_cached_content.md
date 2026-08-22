---
type: Rust Function
title: get_cached_content
resource: src/wikipedia/database.rs#L132-L159
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/ingest/ingest_wikipedia_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/legacy/get_cached_wikipedia_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_cached_content( topic: &str, cache_db_path: &str, ) -> Result<Option<(String, ContentSource)>, Box<dyn std::error::Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)
- [ingest_wikipedia_to_db](../../../../functions/src/wikipedia/ingest/ingest_wikipedia_to_db.md)
- [get_cached_wikipedia_content](../../../../functions/src/wikipedia/legacy/get_cached_wikipedia_content.md)