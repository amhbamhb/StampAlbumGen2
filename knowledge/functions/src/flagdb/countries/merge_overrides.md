---
type: Rust Function
title: merge_overrides
resource: src/flagdb/countries.rs#L41-L65
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/countries/valid_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/load_flag_map_from_fs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/bundled_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn merge_overrides( map: &mut HashMap<String, String>, overrides: HashMap<String, String>, ) -> Result<(), String>`

# Calls

- [valid_code](../../../../functions/src/flagdb/countries/valid_code.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [load_flag_map_from_fs](../../../../functions/src/flagdb/load_flag_map_from_fs.md)
- [bundled_map](../../../../functions/src/flagdb/countries/bundled_map.md)
- [partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply](../../../../functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply.md)