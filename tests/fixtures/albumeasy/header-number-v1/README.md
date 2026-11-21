# Header Page Numbering Example (header-number-v1)

This fixture validates AlbumEasy commands for top page headers:
- `ALBUM_PAGES_HEADER_NUMBER`: Font, point size, alignment, start page number, and prefix/suffix labels.
- `ALBUM_PAGES_HEADER_PAD`: Signed vertical padding in millimetres shifting the header away from (or towards) the page border.
- `PAGE_TEXT`: Left-aligned free page text anchor.

## Fixture Overview

| Input | Reference PDF (AlbumEasy 4.5.0) | Pass 2 Native Album | Native Direct TXT/Raw |
|---|---|---|---|
| [header-number-basic.txt](header-number-basic.txt) | [header-number-basic.pdf](header-number-basic.pdf) | [header-number-basic.stampalbum](header-number-basic.stampalbum) | [header-number-basic-raw.stampalbum](header-number-basic-raw.stampalbum) |
| [header-number-centre.txt](header-number-centre.txt) | [header-number-centre.pdf](header-number-centre.pdf) | [header-number-centre.stampalbum](header-number-centre.stampalbum) | [header-number-centre-raw.stampalbum](header-number-centre-raw.stampalbum) |
| [header-number-left.txt](header-number-left.txt) | [header-number-left.pdf](header-number-left.pdf) | — | — |
| [header-number-lr.txt](header-number-lr.txt) | [header-number-lr.pdf](header-number-lr.pdf) | [header-number-lr.stampalbum](header-number-lr.stampalbum) | [header-number-lr-raw.stampalbum](header-number-lr-raw.stampalbum) |
| [header-number-omitted.txt](header-number-omitted.txt) | [header-number-omitted.pdf](header-number-omitted.pdf) | — | — |
| [header-number-rl.txt](header-number-rl.txt) | [header-number-rl.pdf](header-number-rl.pdf) | — | — |
| [header-number-coexist.txt](header-number-coexist.txt) | [header-number-coexist.pdf](header-number-coexist.pdf) | — | — |

## Tested Commands

- `ALBUM_PAGES_HEADER_NUMBER (font size position [start] [before] [after])`:
  - `font`: Unquoted built-in font identifier (e.g. `HN`, `TB`).
  - `size`: Font size in points (e.g. `8`, `16`).
  - `position`: Alignment position: `L` (Left), `C` (Centre), `R` (Right), `LR` (alternating Left on odd, Right on even), or `RL` (Right on odd, Left on even).
  - `start` (optional, default 1): Starting page number integer.
  - `before` (optional, default `""`): Prefix text before page number (can contain `$PAGES$` macro and explicit newlines).
  - `after` (optional, default `""`): Suffix text after page number (can contain `$PAGES$` macro).
- `ALBUM_PAGES_HEADER_PAD (padding_mm)`:
  - Finite signed millimetres.
- `PAGE_TEXT (font size text [gap])`:
  - Left-aligned content text.
