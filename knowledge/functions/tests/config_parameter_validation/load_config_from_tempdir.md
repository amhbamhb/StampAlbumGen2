---
type: Rust Function
title: load_config_from_tempdir
resource: tests/config_parameter_validation.rs#L50-L56
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/config_parameter_validation/test_config_f32_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_bool_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_string_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_u32_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_usize_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_color_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_parameter_interactions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_edge_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_header_formatting_promotion
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_config_from_tempdir(dir: &std::path::Path) -> Config`

# Calls

- [load_config](../../../functions/src/config/load_config.md)

# Called by

- [test_config_f32_parameter_validation](../../../functions/tests/config_parameter_validation/test_config_f32_parameter_validation.md)
- [test_config_bool_parameter_validation](../../../functions/tests/config_parameter_validation/test_config_bool_parameter_validation.md)
- [test_config_string_parameter_validation](../../../functions/tests/config_parameter_validation/test_config_string_parameter_validation.md)
- [test_config_u32_parameter_validation](../../../functions/tests/config_parameter_validation/test_config_u32_parameter_validation.md)
- [test_config_usize_parameter_validation](../../../functions/tests/config_parameter_validation/test_config_usize_parameter_validation.md)
- [test_config_color_parameter_validation](../../../functions/tests/config_parameter_validation/test_config_color_parameter_validation.md)
- [test_config_parameter_interactions](../../../functions/tests/config_parameter_validation/test_config_parameter_interactions.md)
- [test_config_edge_cases](../../../functions/tests/config_parameter_validation/test_config_edge_cases.md)
- [test_config_header_formatting_promotion](../../../functions/tests/config_parameter_validation/test_config_header_formatting_promotion.md)