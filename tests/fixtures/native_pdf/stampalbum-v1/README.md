# Native `.stampalbum` version 1 vertical slice

`basic.stampalbum` demonstrates the program-owned, human-editable command
format. It has a version header, `#` comments, readable command names, explicit
page/layout/frame/text-policy settings, and a relative image asset path.

All commands are required once, except `STAMP_IMAGE`, which may repeat. Page
dimensions, margins, spacing, border widths, frame width, image dimensions,
scale, and text-policy distances are millimetres. `STAMP_IMAGE` scale is
millimetres per source pixel. The text-policy command arguments are wrapping
mode, expanded-width unit/value, horizontal padding, vertical padding, and gap.
For example, `ALBUM_PAGES_SIZE (215.9 279.4)` selects US Letter, while
`STAMP_IMAGE ("assets/stamp.png" 42 34 0.5)` places a 42 x 34 mm image frame
and scales each source pixel to 0.5 mm before fitting.

The current first slice accepts page size, margins, spacing, page border and
colour, row alignment, stamp frame style, text-footprint policy, and
`STAMP_IMAGE` placements. `STAMP_IMAGE` currently renders the image and frame;
per-stamp heading/caption fields, other stamp shapes, command-state semantics,
native-only extensions, and AlbumEasy TXT import/export are not implemented
by this slice. Unknown commands and missing required settings are errors.
The text-footprint policy is recorded now for the upcoming native text-item
work; this image-only slice has no headings or captions to which it applies.
Header commands are also out of scope for this first slice; consult the
existing commented header-command examples in the application when that
command family is mapped.

The image is the synthetic asymmetric PNG from the AlbumEasy reference fixture.
No application generation route is connected yet; the example renderer takes a
native file and output PDF directly.

Run the API/render test with:

```sh
cargo test --test native_pdf_album
```

Render the checked-in sample directly:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/native_pdf/stampalbum-v1/basic.stampalbum \
  /tmp/stampalbum-native.pdf
```
