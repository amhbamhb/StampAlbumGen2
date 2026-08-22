---
type: Rust Module
title: modes
resource: src/modes.rs#L1-L1892
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-progress-progressindicator
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-error-info-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-commands-file-generator-commandsfilegenerator
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [parse_colnect_csv_filename](../../functions/src/modes/parse_colnect_csv_filename.md)
- [canonical_album_dir_name](../../functions/src/modes/canonical_album_dir_name.md)
- [resolve_runtime_albums_root](../../functions/src/modes/resolve_runtime_albums_root.md)
- [infer_and_fix_catalog_codes_for_colnect_rows](../../functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows.md)
- [has_downloaded_images](../../functions/src/modes/has_downloaded_images.md)
- [is_colnect_export_filename](../../functions/src/modes/is_colnect_export_filename.md)
- [collect_colnect_csv_files_rec](../../functions/src/modes/collect_colnect_csv_files_rec.md)
- [country_slug](../../functions/src/modes/country_slug.md)
- [file_sha256](../../functions/src/modes/file_sha256.md)
- [ensure_colnect_import_history_table](../../functions/src/modes/ensure_colnect_import_history_table.md)
- [has_import_checksum](../../functions/src/modes/has_import_checksum.md)
- [record_import_history](../../functions/src/modes/record_import_history.md)
- [infer_country_from_stamps](../../functions/src/modes/infer_country_from_stamps.md)
- [move_to_country_inbox](../../functions/src/modes/move_to_country_inbox.md)
- [run_colnect_inbox_mode](../../functions/src/modes/run_colnect_inbox_mode.md)
- [sync_missing_stamps_from_colnect](../../functions/src/modes/sync_missing_stamps_from_colnect.md)
- [extract_year](../../functions/src/modes/extract_year.md)
- [numeric_tokens](../../functions/src/modes/numeric_tokens.md)
- [score_candidate](../../functions/src/modes/score_candidate.md)
- [remap_colnect_sg_to_existing_stamps](../../functions/src/modes/remap_colnect_sg_to_existing_stamps.md)
- [backfill_colnect_links_on_existing_stamps](../../functions/src/modes/backfill_colnect_links_on_existing_stamps.md)
- [run_database_mode](../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../functions/src/modes/run_scraping_mode.md)

# Imports

- `crate::progress::ProgressIndicator`
- `sha2::{Digest, Sha256}`
- `std::io::Read`
- `log::{error, info, warn}`
- `log::info`
- `log::{debug, info}`
- `crate::commands_file_generator::CommandsFileGenerator`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)