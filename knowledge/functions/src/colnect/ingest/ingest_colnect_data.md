---
type: Rust Function
title: ingest_colnect_data
resource: src/colnect/ingest.rs#L6-L8
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/colnect_integration/test_colnect_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ingest_switzerland/ingest_switzerland_1972
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ingest_colnect_data(conn: &Connection, stamps: &[ColnectStamp]) -> Result<()>`

# Calls

- [ingest_colnect_data_with_default_catalog](../../../../functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog.md)

# Called by

- [test_colnect_integration](../../../../functions/tests/colnect_integration/test_colnect_integration.md)
- [ingest_switzerland_1972](../../../../functions/tests/ingest_switzerland/ingest_switzerland_1972.md)