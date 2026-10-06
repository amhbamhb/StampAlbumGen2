---
type: Rust Function
title: json
resource: src/external_album.rs#L75-L77
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/openConfigEditor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/saveConfigEditorChanges
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/deleteAlbumConfigOverride
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/external_album_inspect_endpoint_reports_sample_and_rejects_cross_origin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/fetchBackendHealth
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshRuntimeLog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/browseAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/clearAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pollStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/runStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/browseAlbumeasyPathFromDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/saveAlbumeasyPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pickAndCopyColnectFile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshState
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/submitCreateCountryOrTheme
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/startReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pollImportLog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/importColnectFiles
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/generateAlbumPdfNow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/fetchJsonNoStoreWithTimeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/previewLastPdfReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/fetchBackendHealthCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/refreshRuntimeLogCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/reloadLeftRowsFromServer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/refreshRightCandidatesRealtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/saveCatalogDefaultToConfig
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/ensureRightImageUrl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/openCountryFolder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/runReviewStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/applyMatches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/submitLeftEditor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/deleteCurrentLeft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/clearCurrentLeftColnectLink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/addCurrentColnectAsStamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/deleteCurrentCandidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/undoLastDelete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/closeReviewUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/returnToStartReviewPage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/generatePdfNowReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/importColnectFilesReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/pollImportLog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/pickAndCopyColnectFileReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/ensureSaved
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/api
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn json<T: Serialize>(value: &T) -> Result<String>`

# Calls

- [err](../../../functions/src/external_album/err.md)

# Called by

- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [openConfigEditor](../../../functions/src/review_ui/config_editor/openConfigEditor.md)
- [saveConfigEditorChanges](../../../functions/src/review_ui/config_editor/saveConfigEditorChanges.md)
- [deleteAlbumConfigOverride](../../../functions/src/review_ui/config_editor/deleteAlbumConfigOverride.md)
- [external_album_inspect_endpoint_reports_sample_and_rejects_cross_origin](../../../functions/src/review_ui/external_album_api/external_album_inspect_endpoint_reports_sample_and_rejects_cross_origin.md)
- [review_pdf_filters_routes_validate_requests_and_report_empty_selection](../../../functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection.md)
- [country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings](../../../functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings.md)
- [site_accounts_routes_return_json_in_bootstrap_and_review_modes](../../../functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes.md)
- [fetchBackendHealth](../../../functions/src/review_ui/views/bootstrap/fetchBackendHealth.md)
- [refreshRuntimeLog](../../../functions/src/review_ui/views/bootstrap/refreshRuntimeLog.md)
- [browseAlbumsRootPath](../../../functions/src/review_ui/views/bootstrap/browseAlbumsRootPath.md)
- [setAlbumsRootPath](../../../functions/src/review_ui/views/bootstrap/setAlbumsRootPath.md)
- [clearAlbumsRootPath](../../../functions/src/review_ui/views/bootstrap/clearAlbumsRootPath.md)
- [pollStampWorldStatus](../../../functions/src/review_ui/views/bootstrap/pollStampWorldStatus.md)
- [runStampWorldPull](../../../functions/src/review_ui/views/bootstrap/runStampWorldPull.md)
- [browseAlbumeasyPathFromDialog](../../../functions/src/review_ui/views/bootstrap/browseAlbumeasyPathFromDialog.md)
- [saveAlbumeasyPath](../../../functions/src/review_ui/views/bootstrap/saveAlbumeasyPath.md)
- [checkAlbumeasyStatusAfterPull](../../../functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull.md)
- [pickAndCopyColnectFile](../../../functions/src/review_ui/views/bootstrap/pickAndCopyColnectFile.md)
- [refreshState](../../../functions/src/review_ui/views/bootstrap/refreshState.md)
- [submitCreateCountryOrTheme](../../../functions/src/review_ui/views/bootstrap/submitCreateCountryOrTheme.md)
- [startReview](../../../functions/src/review_ui/views/bootstrap/startReview.md)
- [pollImportLog](../../../functions/src/review_ui/views/bootstrap/pollImportLog.md)
- [importColnectFiles](../../../functions/src/review_ui/views/bootstrap/importColnectFiles.md)
- [generateAlbumPdfNow](../../../functions/src/review_ui/views/bootstrap/generateAlbumPdfNow.md)
- [fetchJsonNoStoreWithTimeout](../../../functions/src/review_ui/views/compare/fetchJsonNoStoreWithTimeout.md)
- [previewLastPdfReview](../../../functions/src/review_ui/views/compare/previewLastPdfReview.md)
- [fetchBackendHealthCompare](../../../functions/src/review_ui/views/compare/fetchBackendHealthCompare.md)
- [refreshRuntimeLogCompare](../../../functions/src/review_ui/views/compare/refreshRuntimeLogCompare.md)
- [reloadLeftRowsFromServer](../../../functions/src/review_ui/views/compare/reloadLeftRowsFromServer.md)
- [refreshRightCandidatesRealtime](../../../functions/src/review_ui/views/compare/refreshRightCandidatesRealtime.md)
- [saveCatalogDefaultToConfig](../../../functions/src/review_ui/views/compare/saveCatalogDefaultToConfig.md)
- [ensureRightImageUrl](../../../functions/src/review_ui/views/compare/ensureRightImageUrl.md)
- [openCountryFolder](../../../functions/src/review_ui/views/compare/openCountryFolder.md)
- [runReviewStampWorldPull](../../../functions/src/review_ui/views/compare/runReviewStampWorldPull.md)
- [applyMatches](../../../functions/src/review_ui/views/compare/applyMatches.md)
- [submitLeftEditor](../../../functions/src/review_ui/views/compare/submitLeftEditor.md)
- [deleteCurrentLeft](../../../functions/src/review_ui/views/compare/deleteCurrentLeft.md)
- [clearCurrentLeftColnectLink](../../../functions/src/review_ui/views/compare/clearCurrentLeftColnectLink.md)
- [addCurrentColnectAsStamp](../../../functions/src/review_ui/views/compare/addCurrentColnectAsStamp.md)
- [deleteCurrentCandidate](../../../functions/src/review_ui/views/compare/deleteCurrentCandidate.md)
- [undoLastDelete](../../../functions/src/review_ui/views/compare/undoLastDelete.md)
- [closeReviewUi](../../../functions/src/review_ui/views/compare/closeReviewUi.md)
- [returnToStartReviewPage](../../../functions/src/review_ui/views/compare/returnToStartReviewPage.md)
- [generatePdfNowReview](../../../functions/src/review_ui/views/compare/generatePdfNowReview.md)
- [importColnectFilesReview](../../../functions/src/review_ui/views/compare/importColnectFilesReview.md)
- [pollImportLog](../../../functions/src/review_ui/views/compare/pollImportLog.md)
- [pickAndCopyColnectFileReview](../../../functions/src/review_ui/views/compare/pickAndCopyColnectFileReview.md)
- [load](../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/load.md)
- [ensureSaved](../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/ensureSaved.md)
- [api](../../../functions/src/review_ui/views/external_album/api.md)
- [refresh](../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh.md)