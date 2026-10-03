---
type: Rust Function
title: collect_stampworld_data_async
resource: src/stampworld/data_collector.rs#L55-L118
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn collect_stampworld_data_async( fetcher: &dyn Fetcher, params: &CollectStampworldParams<'_>, ) -> Vec<String>`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)