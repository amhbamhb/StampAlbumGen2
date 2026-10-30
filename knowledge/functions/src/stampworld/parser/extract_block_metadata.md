---
type: Rust Function
title: extract_block_metadata
resource: src/stampworld/parser.rs#L47-L127
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
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/data_collector/test_extract_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/data_collector/test_extract_block_metadata_multiline_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_block_metadata(details: &[String]) -> (String, String, String, String)`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [test_extract_block_metadata](../../../../functions/tests/data_collector/test_extract_block_metadata.md)
- [test_extract_block_metadata_multiline_block](../../../../functions/tests/data_collector/test_extract_block_metadata_multiline_block.md)