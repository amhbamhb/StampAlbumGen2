---
type: Rust Module
title: config_parameter_validation
resource: tests/config_parameter_validation.rs#L1-L508
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-album-globals-generate-album-output-conn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-config-load-config-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-db-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-file-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [write_config](../../functions/tests/config_parameter_validation/write_config.md)
- [make_minimal_test_db](../../functions/tests/config_parameter_validation/make_minimal_test_db.md)
- [load_config_from_tempdir](../../functions/tests/config_parameter_validation/load_config_from_tempdir.md)
- [test_config_f32_parameter_validation](../../functions/tests/config_parameter_validation/test_config_f32_parameter_validation.md)
- [test_config_bool_parameter_validation](../../functions/tests/config_parameter_validation/test_config_bool_parameter_validation.md)
- [test_config_string_parameter_validation](../../functions/tests/config_parameter_validation/test_config_string_parameter_validation.md)
- [test_config_u32_parameter_validation](../../functions/tests/config_parameter_validation/test_config_u32_parameter_validation.md)
- [test_config_usize_parameter_validation](../../functions/tests/config_parameter_validation/test_config_usize_parameter_validation.md)
- [test_config_color_parameter_validation](../../functions/tests/config_parameter_validation/test_config_color_parameter_validation.md)
- [test_config_parameter_interactions](../../functions/tests/config_parameter_validation/test_config_parameter_interactions.md)
- [test_config_edge_cases](../../functions/tests/config_parameter_validation/test_config_edge_cases.md)
- [test_config_header_formatting_promotion](../../functions/tests/config_parameter_validation/test_config_header_formatting_promotion.md)
- [test_config_with_actual_generation](../../functions/tests/config_parameter_validation/test_config_with_actual_generation.md)

# Imports

- `stamp_album_gen2::album_globals::generate_album_output_conn`
- `stamp_album_gen2::config::{load_config, Config}`
- `stamp_album_gen2::db_utils`
- `stamp_album_gen2::file_utils`
- `std::env`
- `std::fs`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)