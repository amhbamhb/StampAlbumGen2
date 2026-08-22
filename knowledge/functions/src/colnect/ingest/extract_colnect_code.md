---
type: Rust Function
title: extract_colnect_code
resource: src/colnect/ingest.rs#L322-L330
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_colnect_code(codes: &str) -> Option<String>`

# Called by

- [ingest_colnect_data_with_mode](../../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)