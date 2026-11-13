# Real-stamp AlbumEasy and native PDF example

This fixture pairs human-editable input with output from both renderers:

| Input | PDF output |
|---|---|
| [AlbumEasy 4.5.0 `album.txt`](album.txt) | [AlbumEasy `album.pdf`](album.pdf) |
| [Native `album.stampalbum`](album.stampalbum) | [Native `album-native.pdf`](album-native.pdf) |

Both inputs use the same four real US stamp images and their recorded Colnect
physical dimensions. The native document is program-owned text, not TOML or
AlbumEasy TXT. The two PDFs demonstrate their respective renderers; this is not
a claim of exact visual or behavioral parity.

## Stamp data and image provenance

The images are reused from the real-stamp pagination fixture; no Album database
files or images were modified. The source database was
`/home/ray/AlbumGen2/Albums/United-States/United-States.db`, opened read-only.
The selected rows' current Colnect dimensions were verified directly against
the database. Per-image source paths, SHA-256 values, and pixel dimensions are
recorded in [the source manifest](../stamp-pagination-v1/source-records.json).

| DB ID | Year | SG number | Colnect ID | Physical size |
|---:|---:|---|---:|---:|
| 150 | 1959 | 1125a | 700029 | 52 x 29 mm |
| 151 | 1959 | 1137a | 700273 | 26 x 29 mm |
| 152 | 1959 | 1137b | 700274 | 26 x 29 mm |
| 156 | 1960 | 1148a | 700272 | 52 x 29 mm |

The native image scale values are calculated from each image's source pixel
width and the recorded Colnect width. The native renderer fits the image
proportionally inside the physical box; it does not enlarge the source image.

## Manual review

Print either PDF on US Letter paper at **Actual size / 100%**, with Fit/Shrink
disabled. Compare the stamps' printed sizes and appearance, then record whether
the native output is visually acceptable before treating it as an approved
example. PDF creation and automated checks do not replace this human review.
Physical printing and maintainer visual acceptance are still pending.

## Reproduce

Run the AlbumEasy 4.5.0 reference renderer from this directory (Linux example):

```sh
xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb \
  /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
```

Render the native document from the repository root:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/native-real-stamps-v1/album.stampalbum \
  tests/fixtures/albumeasy/native-real-stamps-v1/album-native.pdf
```

Validate the native fixture:

```sh
cargo test --test native_pdf_album real_stamp_album_file_renders_letter_pdf
```
