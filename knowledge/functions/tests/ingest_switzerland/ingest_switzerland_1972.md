---
type: Rust Function
title: ingest_switzerland_1972
resource: tests/ingest_switzerland.rs#L7-L27
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/ingest_colnect_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ingest_switzerland_1972()`

# Calls

- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [ingest_colnect_data](../../../functions/src/colnect/ingest/ingest_colnect_data.md)