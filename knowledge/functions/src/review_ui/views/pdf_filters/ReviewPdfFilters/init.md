---
type: JavaScript Method
title: init
resource: src/review_ui/views/pdf_filters.js#L17-L34
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/bin/migrate_fix_block_metadata/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async init()`

# Calls

- [parse](../../../../../../functions/src/external_album/parser/parse.md)

# Called by

- [main](../../../../../../functions/src/bin/migrate_fix_block_metadata/main.md)