---
type: Rust Module
title: external_album_api
resource: src/review_ui/external_album_api.rs#L1-L560
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-external-album
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-atomic-atomicu64
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ImportJob](../../../classes/src/review_ui/external_album_api/ImportJob.md)
- [SourceRequest](../../../classes/src/review_ui/external_album_api/SourceRequest.md)
- [ImportRequest](../../../classes/src/review_ui/external_album_api/ImportRequest.md)
- [AlbumRequest](../../../classes/src/review_ui/external_album_api/AlbumRequest.md)
- [EditRequest](../../../classes/src/review_ui/external_album_api/EditRequest.md)
- [ImageRequest](../../../classes/src/review_ui/external_album_api/ImageRequest.md)
- [body](../../../functions/src/review_ui/external_album_api/body.md)
- [root](../../../functions/src/review_ui/external_album_api/root.md)
- [renderer](../../../functions/src/review_ui/external_album_api/renderer.md)
- [start_job](../../../functions/src/review_ui/external_album_api/start_job.md)
- [parameter](../../../functions/src/review_ui/external_album_api/parameter.md)
- [same_origin](../../../functions/src/review_ui/external_album_api/same_origin.md)
- [route](../../../functions/src/review_ui/external_album_api/route.md)
- [store_image](../../../functions/src/review_ui/external_album_api/store_image.md)
- [pdf_path](../../../functions/src/review_ui/external_album_api/pdf_path.md)
- [handle](../../../functions/src/review_ui/external_album_api/handle.md)
- [respond](../../../functions/src/review_ui/external_album_api/respond.md)
- [external_album_query_decodes_names_without_interpreting_paths](../../../functions/src/review_ui/external_album_api/external_album_query_decodes_names_without_interpreting_paths.md)
- [external_album_image_upload_preserves_original_and_rejects_invalid_data](../../../functions/src/review_ui/external_album_api/external_album_image_upload_preserves_original_and_rejects_invalid_data.md)
- [external_album_image_upload_rejects_case_collision](../../../functions/src/review_ui/external_album_api/external_album_image_upload_rejects_case_collision.md)
- [external_album_inspect_endpoint_reports_sample_and_rejects_cross_origin](../../../functions/src/review_ui/external_album_api/external_album_inspect_endpoint_reports_sample_and_rejects_cross_origin.md)

# Imports

- `super::*`
- `crate::external_album`
- `serde_json::{json, Value}`
- `std::io::Read`
- `std::sync::atomic::AtomicU64`
- `std::io::Write`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)