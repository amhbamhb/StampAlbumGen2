---
type: JavaScript Method
title: status
resource: src/review_ui/views/pdf_filters.js#L12-L15
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/colnect/image_fetcher/ReqwestFetcher/fetcher/fetch_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/ReqwestFetcher/fetcher/fetch_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/fetch_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/fetch_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/download_to_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/get_image_blocking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/change
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/command_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/refresh_linux_desktop_caches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/mark_desktop_file_trusted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/fetch_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/fetch_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/download_stamp_images
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`status(message)`

# Called by

- [fetch_text](../../../../../../functions/src/colnect/image_fetcher/ReqwestFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../../../../../functions/src/colnect/image_fetcher/ReqwestFetcher/fetcher/fetch_bytes.md)
- [fetch_text](../../../../../../functions/src/colnect/image_fetcher/fetch_text.md)
- [fetch_bytes](../../../../../../functions/src/colnect/image_fetcher/fetch_bytes.md)
- [download_to_path](../../../../../../functions/src/commands_file_generator/image_utils/download_to_path.md)
- [get_image_blocking](../../../../../../functions/src/commands_file_generator/image_utils/get_image_blocking.md)
- [try_rasterize_svg_to_png](../../../../../../functions/src/flagdb/try_rasterize_svg_to_png.md)
- [refresh](../../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh.md)
- [change](../../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/change.md)
- [command_exists](../../../../../../functions/src/setup/command_exists.md)
- [refresh_linux_desktop_caches](../../../../../../functions/src/setup/refresh_linux_desktop_caches.md)
- [mark_desktop_file_trusted](../../../../../../functions/src/setup/mark_desktop_file_trusted.md)
- [fetch_text](../../../../../../functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../../../../../functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_bytes.md)
- [fetch_text](../../../../../../functions/src/stampworld/fetch/fetch_text.md)
- [fetch_bytes](../../../../../../functions/src/stampworld/fetch/fetch_bytes.md)
- [process_1847us_page](../../../../../../functions/src/us1847/collector/process_1847us_page.md)
- [download_stamp_images](../../../../../../functions/src/us1847/collector/download_stamp_images.md)