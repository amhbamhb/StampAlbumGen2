---
type: Rust Function
title: generate_album_output
resource: src/album_globals/mod.rs#L437-L469
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/imported_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn generate_album_output<F>( db_path: &str, album_name: &str, page_callback: Option<F>, start_year: Option<i32>, end_year: Option<i32>, ) -> std::path::PathBuf where F: FnMut(usize),`

# Calls

- [imported_artifacts](../../../functions/src/album_globals/imported_artifacts.md)
- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)