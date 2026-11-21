# Page Size Specification (page-size-v1)

This fixture demonstrates and validates the AlbumEasy command `ALBUM_PAGES_SIZE`, configuring page media box dimensions in millimetres for standard and non-standard orientations.

## Fixture Overview

| Input | Reference PDF (AlbumEasy 4.5.0) | Pass 2 Native Album | Native Direct TXT/Raw | Description |
|---|---|---|---|---|
| [a4-portrait.txt](a4-portrait.txt) | [a4-portrait.reference.pdf](a4-portrait.reference.pdf) | [a4-portrait.stampalbum](a4-portrait.stampalbum) | [a4-portrait-raw.stampalbum](a4-portrait-raw.stampalbum) | Explicit A4 portrait (210.0 x 297.0 mm) |
| [a4-landscape.txt](a4-landscape.txt) | [a4-landscape.reference.pdf](a4-landscape.reference.pdf) | [a4-landscape.stampalbum](a4-landscape.stampalbum) | [a4-landscape-raw.stampalbum](a4-landscape-raw.stampalbum) | A4 landscape orientation via inverted bounds (297.0 x 210.0 mm) |
| [us-letter.txt](us-letter.txt) | [us-letter.reference.pdf](us-letter.reference.pdf) | [us-letter.stampalbum](us-letter.stampalbum) | [us-letter-raw.stampalbum](us-letter-raw.stampalbum) | Standard ANSI US Letter (215.9 x 279.4 mm) |
| [fractional.txt](fractional.txt) | [fractional.reference.pdf](fractional.reference.pdf) | [fractional.stampalbum](fractional.stampalbum) | [fractional-raw.stampalbum](fractional-raw.stampalbum) | Fractional millimeter page sizing (210.25 x 297.75 mm) |

### Tested Commands

- `ALBUM_PAGES_SIZE (width height)`: Sets global page dimensions in millimetres. Orientation (portrait vs landscape) is controlled directly by the width/height argument order without additional flags.

The command requires two finite positive numeric values in millimetres and must precede `PAGE_START`.

## Reference Provenance (AlbumEasy 4.5.0)

| Input File | Input SHA-256 | Reference PDF | Reference SHA-256 |
|---|---|---|---|
| `a4-portrait.txt` | `bd520a1224199f98bdc4a6df728472d8b3fffcaea379c265467f43839b7637a1` | `a4-portrait.reference.pdf` | `2e398e0b73ec28101607881677d2363f43a5ea6a6e5e51284355ba19b6b72a20` |
| `a4-landscape.txt` | `23060cf98eac9ee4650f7ba4e94b812183ba5324d8c833852984e44be6262e7d` | `a4-landscape.reference.pdf` | `d533e35794e99ca44c214fdcc33ad119a49c917867cfc3e55c5d8669920ca89d` |
| `us-letter.txt` | `20772c2f1e64072b6a38d049108009550357f8d0903c920e82d8aec0b7eb2860` | `us-letter.reference.pdf` | `46d308c593d531a8cb8c2db796e58faede01023422754d016dc7ed97e36713a6` |
| `fractional.txt` | `70e2bd0613efff939039f1a8ff7b5a10461362061a35fb9186b7a051ae24556e` | `fractional.reference.pdf` | `dab9b279ebdb8d01398a0ef0cf064d9215a75c0175e5562c3e74882c23d36d6b` |

Negative diagnostic inputs:
- `missing-size.txt`: `5e9157832de5106766fba37bb7674f1aedf33154000454ea59d79d87380b0902`
- `missing-height.txt`: `ff5947a8eb96b1b8be73ff255f07a976ff9d3a0e1e98418ff24483ae2ae4e019`

## Pass 2 Compatibility Files

1. **`*.stampalbum`**: Native `STAMPALBUM 1` documents verifying that the native vector engine parses and applies `ALBUM_PAGES_SIZE` for portrait, landscape, US Letter, and fractional dimensions.
2. **`*-raw.stampalbum`**: Raw AlbumEasy command streams testing direct native loader compatibility.

## Automated Verification

Run the dedicated page settings regression suite:

```sh
cargo test --test native_pdf_page_settings
```

This verifies:
1. Native rendering of `.txt` inputs against frozen MediaBox dimensions in `expected.json`.
2. Native loading and rendering of `.stampalbum` and `-raw.stampalbum` inputs.
3. Strict validation on argument counts, non-positive numbers, and missing dimensions.
