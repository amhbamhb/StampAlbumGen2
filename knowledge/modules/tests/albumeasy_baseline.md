---
type: Rust Module
title: albumeasy_baseline
resource: tests/albumeasy_baseline.rs#L1-L380
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/oxidize-pdf-parser-pdfdocument
    resolved_by: tree-sitter
    confidence: exact
  - target: external/roxmltree-document-node
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-ffi-osstr
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Expectations](../../classes/tests/albumeasy_baseline/Expectations.md)
- [fixture](../../functions/tests/albumeasy_baseline/fixture.md)
- [expectations](../../functions/tests/albumeasy_baseline/expectations.md)
- [check_hash](../../functions/tests/albumeasy_baseline/check_hash.md)
- [near](../../functions/tests/albumeasy_baseline/near.md)
- [numbers](../../functions/tests/albumeasy_baseline/numbers.md)
- [coordinate](../../functions/tests/albumeasy_baseline/coordinate.md)
- [check_trace](../../functions/tests/albumeasy_baseline/check_trace.md)
- [check_text](../../functions/tests/albumeasy_baseline/check_text.md)
- [tool_output](../../functions/tests/albumeasy_baseline/tool_output.md)
- [inspect_text](../../functions/tests/albumeasy_baseline/inspect_text.md)
- [inspect](../../functions/tests/albumeasy_baseline/inspect.md)
- [page_traces](../../functions/tests/albumeasy_baseline/page_traces.md)
- [check_strict_trace](../../functions/tests/albumeasy_baseline/check_strict_trace.md)
- [check_frozen](../../functions/tests/albumeasy_baseline/check_frozen.md)
- [frozen_identities_and_hash_drift](../../functions/tests/albumeasy_baseline/frozen_identities_and_hash_drift.md)
- [oxidize_reference_text_and_negative_controls](../../functions/tests/albumeasy_baseline/oxidize_reference_text_and_negative_controls.md)
- [tolerance_and_missing_text_are_rejected](../../functions/tests/albumeasy_baseline/tolerance_and_missing_text_are_rejected.md)
- [strict_trace_detects_styles_but_ignores_filename](../../functions/tests/albumeasy_baseline/strict_trace_detects_styles_but_ignores_filename.md)
- [reference_geometry_text_and_negative_controls](../../functions/tests/albumeasy_baseline/reference_geometry_text_and_negative_controls.md)
- [candidate_matches_reference_strictly](../../functions/tests/albumeasy_baseline/candidate_matches_reference_strictly.md)

# Imports

- `oxidize_pdf::parser::PdfDocument`
- `roxmltree::{Document, Node}`
- `serde::Deserialize`
- `sha2::{Digest, Sha256}`
- `std::error::Error`
- `std::ffi::OsStr`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `std::process::Command`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)