---
type: Rust Method
title: client
resource: src/stampworld/fetch.rs#L100-L102
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stampworld/fetch/build_client
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/build_image_client
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn client(&self) -> &Client`

# Called by

- [build_client](../../../../../functions/src/stampworld/fetch/build_client.md)
- [build_image_client](../../../../../functions/src/stampworld/fetch/build_image_client.md)