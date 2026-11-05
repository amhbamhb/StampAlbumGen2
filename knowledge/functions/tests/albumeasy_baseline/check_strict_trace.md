---
type: Rust Function
title: check_strict_trace
resource: tests/albumeasy_baseline.rs#L261-L268
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/page_traces
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/albumeasy_baseline/strict_trace_detects_styles_but_ignores_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/candidate_matches_reference_strictly
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_strict_trace(reference: &str, actual: &str) -> Result<()>`

# Calls

- [page_traces](../../../functions/tests/albumeasy_baseline/page_traces.md)

# Called by

- [strict_trace_detects_styles_but_ignores_filename](../../../functions/tests/albumeasy_baseline/strict_trace_detects_styles_but_ignores_filename.md)
- [candidate_matches_reference_strictly](../../../functions/tests/albumeasy_baseline/candidate_matches_reference_strictly.md)