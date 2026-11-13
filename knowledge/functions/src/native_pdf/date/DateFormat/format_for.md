---
type: Rust Method
title: format_for
resource: src/native_pdf/date.rs#L29-L44
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/date/DateFormat/format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/date/DateText/text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn format_for(self, date: NaiveDate, separator: &str, prefix: &str) -> Result<String>`

# Calls

- [contains](../../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [format](../../../../../functions/src/native_pdf/date/DateFormat/format.md)
- [text](../../../../../functions/src/native_pdf/date/DateText/text.md)