# Page Margins and Even-Page Margin Override (margins-v1)

This fixture demonstrates and validates the AlbumEasy commands `ALBUM_PAGES_MARGINS` and `ALBUM_PAGES_MARGINSE`, defining page margins and independent margin overrides for even-numbered pages (facing-page / book-binding layouts).

## Fixture Overview

| Input | Reference PDF (AlbumEasy 4.5.0) | Pass 2 Native Album | Native Direct TXT/Raw | Description |
|---|---|---|---|---|
| [symmetric.txt](symmetric.txt) | [symmetric.reference.pdf](symmetric.reference.pdf) | [symmetric.stampalbum](symmetric.stampalbum) | [symmetric-raw.stampalbum](symmetric-raw.stampalbum) | Symmetric 15mm margins on all edges |
| [asymmetric.txt](asymmetric.txt) | [asymmetric.reference.pdf](asymmetric.reference.pdf) | [asymmetric.stampalbum](asymmetric.stampalbum) | [asymmetric-raw.stampalbum](asymmetric-raw.stampalbum) | Asymmetric margins (25mm left binding clearance) |
| [fractional.txt](fractional.txt) | [fractional.reference.pdf](fractional.reference.pdf) | [fractional.stampalbum](fractional.stampalbum) | [fractional-raw.stampalbum](fractional-raw.stampalbum) | Fractional millimeter margins (12.25 17.75 20.25 23.75) |
| [even-override.txt](even-override.txt) | [even-override.reference.pdf](even-override.reference.pdf) | [even-override.stampalbum](even-override.stampalbum) | [even-override-raw.stampalbum](even-override-raw.stampalbum) | 3-page document with alternating odd/even binding margins |

### Tested Commands

- `ALBUM_PAGES_MARGINS (left right top bottom)`: Sets default margins for all pages (or odd pages when even override is configured).
- `ALBUM_PAGES_MARGINSE (left right top bottom)`: Overrides margins on even-numbered physical pages (2, 4, 6...).

Both commands require four positive numeric arguments (in millimeters) and must precede `PAGE_START`. Command keywords are case-insensitive.

## Reference Provenance (AlbumEasy 4.5.0)

| Input File | Input SHA-256 | Reference PDF | Reference SHA-256 |
|---|---|---|---|
| `symmetric.txt` | `eeb1aeabed1b81e1d72d1dcb50a31156fcf68beedb7217d62cda838f11fdb5dd` | `symmetric.reference.pdf` | `783fe6d20f61571e82b8204b829d3cb1658c4ca5d4de594c99c8900d1f8a640c` |
| `asymmetric.txt` | `e18251b812d03e7fab60c2fa5e1c0c6a5d86075393d26fffcf306a9aca7dc6d2` | `asymmetric.reference.pdf` | `5eeba9a90f39242116b6f20dde69ef3c2be886ecb6f41f8f5fced76ecc1f8f49` |
| `fractional.txt` | `f10fc639a5c713338f1e55d1fb6afd216950d981293f72f32e52cd0b1451ef75` | `fractional.reference.pdf` | `6f467111bf4faeea602ca3f91111c20bb7bd3f0821e5ce2e25998bd25232932c` |
| `even-override.txt` | `24186d5df249e3a7de42f3c4fefdf276458bbbc174277c07c534542d0c061314` | `even-override.reference.pdf` | `fd3e97d5614452bdc58f4e6a42cfaf5564c8ab0960e92860f33b7cc540d93af6` |

Negative diagnostic inputs:
- `missing-margins.txt`: `8c2f53bf3b210a25cea7d1421663742ed8affa9a270a22bdb6aca85efe008b49`
- `missing-bottom.txt`: `70c804120fce71fdd0fc6bd58a113fb32bf683f9824a7ab2b991cb49241a96bc`

## Pass 2 Compatibility Files

1. **`*.stampalbum`**: Native `STAMPALBUM 1` documents verifying that the native vector engine parses and applies symmetric, asymmetric, fractional, and alternating even margins (`ALBUM_PAGES_MARGINSE`).
2. **`*-raw.stampalbum`**: Raw AlbumEasy commands files testing direct native loader compatibility across all margin variants.

## Automated Verification

Run the dedicated margin and even-margin regression suites:

```sh
cargo test --test native_pdf_marginse
cargo test --test native_pdf_page_settings
```

This verifies:
1. `even-override.txt` renders 3 pages natively with alternating odd/even content and margin bounds.
2. `even-override-raw.stampalbum` produces identical page streams to the TXT render.
3. Native `STAMPALBUM 1` documents parse `ALBUM_PAGES_MARGINSE` and apply alternating layouts across page breaks.
4. Case-insensitivity (`ALBUM_PAGES_MARGINSE`, `Album_Pages_Marginse`, `album_pages_marginse`).
5. Strict validation on argument counts, non-numeric values, negative values, post-`PAGE_START` placement, and native duplicates.
