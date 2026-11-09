# Native stamp-footer text proof

This fixture is the explicit `STAMPALBUM 1` representation of the accepted
footer alignment/padding behavior tracked by GitHub issue
[#94](https://github.com/amhbamhb/StampAlbumGen2/issues/94). It exercises a
native stamp heading and a three-sided footer attachment on the same stamp.
The AlbumEasy compatibility inputs and their proofs remain in
`../../albumeasy/stamp-footer-text-v1/`.

The native syntax requires `NATIVE_STAMP_FOOTER_TEXT_ALIGN` and
`NATIVE_STAMP_FOOTER_TEXT_PAD` to be set explicitly before attaching
`NATIVE_STAMP_FOOTER_TEXT` to a stamp. The footer command takes a font, point
size, named colour, and quoted left, centre, and right strings. Empty strings
omit a side. The output uses a US Letter page and one embedded image.

## Reproduction and validation

Run the parser/layout regression and regenerate the checked-in proof PDF:

```sh
cargo test --test native_pdf_album explicit_native_footer_fixture_renders_with_heading
cargo test --test native_pdf_album generate_native_footer_review_pdf -- --ignored
```

The PDF was validated with `pdfinfo` and `pdfimages -list`: one 612 x 792 pt
page and one image placement. The first-page raster was inspected; heading,
left/centre/right captions, and the frame are visible without clipping.

| File | Size | SHA-256 |
|---|---:|---|
| `footer-text.stampalbum` | 815 bytes | `99b9afebaf0d9ca08d5b25876ec431dd5102ec4fc3cc01e1c6ce9d58c19870ea` |
| `footer-text-native.pdf` | 5,625 bytes | `84f991ce9c548b6b48730ff30f066509b8719e4a75acb7917035afbf49e89877` |

No database was opened or changed. The proof generation writes only the PDF
beside this input; no source image is copied or modified.
