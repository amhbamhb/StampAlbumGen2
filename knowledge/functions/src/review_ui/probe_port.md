---
type: Rust Function
title: probe_port
resource: src/review_ui.rs#L1744-L1814
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/bind_review_server
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn probe_port(addr: &str) -> PortProbeResult`

# Calls

- [parse](../../../functions/src/external_album/parser/parse.md)
- [log_startup](../../../functions/src/review_ui/log_startup.md)

# Called by

- [bind_review_server](../../../functions/src/review_ui/bind_review_server.md)