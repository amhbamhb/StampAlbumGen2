---
type: Rust Function
title: generate_pdf_report
resource: src/review_ui/report_generation.rs#L14-L44
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_pdf_report(stubs: &[StampStub], output_path: &str) -> io::Result<()>`

# Calls

- [count](../../../../functions/src/album_globals/selection/count.md)