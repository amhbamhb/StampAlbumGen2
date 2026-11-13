---
type: Rust Module
title: document
resource: src/native_pdf/document.rs#L1-L473
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-commands-colour-colour-album-border-albumbordercolour-commands-page-album-pages-border-pageborder-album-pages-crop-marks-albumpagescropmarks-cropmarkslayout-album-pages-decorative-border-albumpagesdecorativeborder-decorativeborderlayout-album-pages-footer-albumpagesfooter-footerlayout-album-pages-footer-date-albumpagesfooterdate-album-pages-footer-number-albumpagesfooternumber-album-pages-header-albumpagesheader-headerlayout-album-pages-header-date-albumpagesheaderdate-album-pages-header-number-albumpagesheadernumber-album-pages-margins-margins-album-pages-marginse-evenmargins-album-pages-size-pagesize-album-pages-spacing-spacing-album-pages-title-albumpagestitle-titlelayout-date-datesource-text-text-lines-overlap-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-commands-page-album-pages-border-borderline
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Bounds](../../../classes/src/native_pdf/document/Bounds.md)
- [inset](../../../functions/src/native_pdf/document/Bounds/inset.md)
- [validate](../../../functions/src/native_pdf/document/Bounds/validate.md)
- [PageLayout](../../../classes/src/native_pdf/document/PageLayout.md)
- [PageSettings](../../../classes/src/native_pdf/document/PageSettings.md)
- [new](../../../functions/src/native_pdf/document/PageSettings/new.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)
- [set_crop_marks](../../../functions/src/native_pdf/document/PageSettings/set_crop_marks.md)
- [set_decorative_border](../../../functions/src/native_pdf/document/PageSettings/set_decorative_border.md)
- [set_title](../../../functions/src/native_pdf/document/PageSettings/set_title.md)
- [set_header](../../../functions/src/native_pdf/document/PageSettings/set_header.md)
- [set_header_number](../../../functions/src/native_pdf/document/PageSettings/set_header_number.md)
- [set_header_date](../../../functions/src/native_pdf/document/PageSettings/set_header_date.md)
- [capture_generation_date](../../../functions/src/native_pdf/document/PageSettings/capture_generation_date.md)
- [set_footer](../../../functions/src/native_pdf/document/PageSettings/set_footer.md)
- [set_footer_number](../../../functions/src/native_pdf/document/PageSettings/set_footer_number.md)
- [set_footer_date](../../../functions/src/native_pdf/document/PageSettings/set_footer_date.md)
- [spacing](../../../functions/src/native_pdf/document/PageSettings/spacing.md)
- [set_border](../../../functions/src/native_pdf/document/PageSettings/set_border.md)
- [border](../../../functions/src/native_pdf/document/PageSettings/border.md)
- [set_border_colour](../../../functions/src/native_pdf/document/PageSettings/set_border_colour.md)
- [border_colour](../../../functions/src/native_pdf/document/PageSettings/border_colour.md)
- [set_border_line_colours](../../../functions/src/native_pdf/document/PageSettings/set_border_line_colours.md)
- [border_line_colour](../../../functions/src/native_pdf/document/PageSettings/border_line_colour.md)
- [resolve](../../../functions/src/native_pdf/document/PageSettings/resolve.md)
- [resolve_with_page_count](../../../functions/src/native_pdf/document/PageSettings/resolve_with_page_count.md)
- [resolve_with_context](../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)
- [resolve_page](../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)

# Imports

- `super::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder,
        album_pages_crop_marks::{AlbumPagesCropMarks, CropMarksLayout},
        album_pages_decorative_border::{AlbumPagesDecorativeBorder, DecorativeBorderLayout},
        album_pages_footer::{AlbumPagesFooter, FooterLayout},
        album_pages_footer_date::AlbumPagesFooterDate,
        album_pages_footer_number::AlbumPagesFooterNumber,
        album_pages_header::{AlbumPagesHeader, HeaderLayout},
        album_pages_header_date::AlbumPagesHeaderDate,
        album_pages_header_number::AlbumPagesHeaderNumber,
        album_pages_margins::Margins,
        album_pages_marginse::EvenMargins,
        album_pages_size::PageSize,
        album_pages_spacing::Spacing,
        album_pages_title::{AlbumPagesTitle, TitleLayout},
    },
    date::DateSource,
    text::text_lines_overlap,
    Result,
}`
- `super::commands::page::album_pages_border::BorderLine`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)