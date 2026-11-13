---
type: Rust Function
title: test_colnect_integration
resource: tests/colnect_integration.rs#L47-L101
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
  - target: functions/src/colnect/db/get_colnect_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_colnect_integration()`

# Calls

- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [ingest_colnect_data](../../../functions/src/colnect/ingest/ingest_colnect_data.md)
- [get_colnect_info](../../../functions/src/colnect/db/get_colnect_info.md)
- [prepare](../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)