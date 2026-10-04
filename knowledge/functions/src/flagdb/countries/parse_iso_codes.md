---
type: Rust Function
title: parse_iso_codes
resource: src/flagdb/countries.rs#L18-L39
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/valid_code
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/countries/bundled_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/supplied_iso_file_has_246_unique_codes_and_names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn parse_iso_codes(input: &str) -> Result<HashMap<String, String>, String>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [valid_code](../../../../functions/src/flagdb/countries/valid_code.md)

# Called by

- [bundled_map](../../../../functions/src/flagdb/countries/bundled_map.md)
- [supplied_iso_file_has_246_unique_codes_and_names](../../../../functions/src/flagdb/countries/supplied_iso_file_has_246_unique_codes_and_names.md)