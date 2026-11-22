# AlbumEasy vs Native-PDF Command Comparison & Exactness Analysis

**Prepared:** 2026-11-22  
**Target References:**
- AlbumEasy 4.2.1 Manual ([AlbumEasy_4.2.1.pdf](../../docs/AlbumEasy_4.2.1.pdf))
- AlbumEasy 4.5.0 Help Database (`C:\Program Files (x86)\AlbumEasy\AlbumEasy45.0.qch`) & Console Reference (`readme-console.txt`)
- Sample AlbumEasy Scripts ([docs/easy/](../../docs/easy/))
- Native-PDF Engine (`src/native_pdf/album.rs`, `src/native_pdf/albumeasy.rs`, `src/native_pdf/document.rs`)

---

## 1. Executive Summary & Architectural Overview

StampAlbumGen2 contains two distinct but cooperating rendering pathways:

1. **AlbumEasy Compatibility Layer (`albumeasy::render_file`)**:
   - Parses native AlbumEasy `.txt` scripts and raw-command `.stampalbum` files directly.
   - Emulates AlbumEasy 4.2.1 and 4.5.0 imperative cursor layout, interpreting commands as closely as possible to the reference AlbumEasy engine.
2. **Native Vector Album Engine (`NativeAlbum::load`)**:
   - Parses the versioned `STAMPALBUM 1` document format.
   - Implements a declarative layout model with automatic multi-row distribution, dynamic stamp caption footprints, strict minimum frame-to-text clearances (eliminating line bleeding), and high-fidelity vector rendering.

### Key Finding: Are AlbumEasy and Native-PDF Commands "Exact"?
- **Shared Commands (Exact Syntax & Semantics)**: Exactly **8 core album setup commands** share the exact same keyword and argument grammar between AlbumEasy and Native `STAMPALBUM 1` documents (`ALBUM_PAGES_SIZE`, `ALBUM_PAGES_MARGINS`, `ALBUM_PAGES_MARGINSE`, `ALBUM_PAGES_SPACING`, `ALBUM_PAGES_BORDER`, `COLOUR_ALBUM_BORDER`, `ALBUM_AUTHOR`, `ALBUM_TITLE`).
- **Mapped / Adapted Commands (Semantic Parity, Syntax Difference)**: Commands such as `ALBUM_PAGES_TITLE`, `ALBUM_PAGES_HEADER`, `ALBUM_PAGES_HEADER_NUMBER`, `ALBUM_PAGES_FOOTER_NUMBER`, and `ALBUM_STAMP_IMG_NEW_SETTING` are supported natively, but use explicit `NATIVE_*` syntax to provide deterministic control over spacing, colors, padding, and layout bounds without relying on AlbumEasy's hidden cursor heuristics.
- **Native-Only Extensions (No AlbumEasy Equivalent)**: The native engine introduces commands like `NATIVE_STAMP_TEXT_POLICY` (with `expand_footprint` and guaranteed frame clearance), `NATIVE_ROW_ALIGNMENT`, `NATIVE_ROW_BREAK`, `NATIVE_PAGE_NUMBER_SECTION`, and `NATIVE_STAMP_FOOTER_TEXT`. These commands **do not compile in AlbumEasy** because AlbumEasy's parser does not recognize them.
- **AlbumEasy Features in Backlog**: Advanced AlbumEasy features such as column mode (`PAGE_COLUMN_START/NEXT/STOP`), free-form text paragraphs (`PAGE_TEXT_PARAGRAPH_START`), horizontal rules (`PAGE_RULE_H`), geometric stamp shapes (triangles, diamonds, pentagons, hexagons), quadrille grids (`PAGE_QUADRILLE`), and preprocessor macros (`$INCLUDE`, `$DEFINE`) are currently unmapped in the native engine.

---

## 2. Command-by-Command Exactness Comparison

### A. Album Page Geometry & Setup

| AlbumEasy Command | AlbumEasy Grammar (4.2.1 / 4.5.0) | Native-PDF Grammar (`STAMPALBUM 1`) | Status | Exactness & Semantic Notes |
|---|---|---|---|---|
| `ALBUM_PAGES_SIZE` | `(width height)` in mm | `ALBUM_PAGES_SIZE (width height)` | **EXACT** | Identical name, arguments, and units. Both validate positive finite floats. |
| `ALBUM_PAGES_MARGINS` | `(left right top bottom)` in mm | `ALBUM_PAGES_MARGINS (left right top bottom)` | **EXACT** | Identical name and 4 positional arguments in mm. |
| `ALBUM_PAGES_MARGINSE` | `(left right top bottom)` in mm | `ALBUM_PAGES_MARGINSE (left right top bottom)` | **EXACT** | Identical name and arguments. Governs even-page alternating margins. |
| `ALBUM_PAGES_BORDER` | `(w1 w2 w3 gap)` or legacy `(w1 w2 gap)` in mm | `ALBUM_PAGES_BORDER (w1 w2 w3 gap)` | **EXACT** | Both engines accept 3 or 4 positive numeric widths and line gaps. |
| `COLOUR_ALBUM_BORDER` / `COLOR_` | `(colour_name)` | `COLOUR_ALBUM_BORDER (colour_name)` | **EXACT** | Both engines accept named standard colours (black, grey, navy, etc.). |
| `ALBUM_AUTHOR` | `("Author Name")` | `ALBUM_AUTHOR ("Author Name")` | **EXACT** | Identical string metadata mapped to PDF `/Author`. |
| `ALBUM_TITLE` | `("Document Title")` | `ALBUM_TITLE ("Document Title")` | **EXACT** | Identical string metadata mapped to PDF `/Title`. |
| `ALBUM_PAGES_DECORATIVE_BORDER` | `("definition_file.txt")` | Lower-level TOML/API only | **PARTIAL** | AlbumEasy loads a separate 11-command definition file. Native loader is not yet wired. |
| `ALBUM_PAGES_BORDER3` | `(w1 w2 w3 gap)` | Unsupported in Native | **OBSOLETE** | Legacy 4.2.1 alias; use `ALBUM_PAGES_BORDER`. |

---

### B. Page Spacing & Frame Clearance

| AlbumEasy Command | AlbumEasy Grammar (4.2.1 / 4.5.0) | Native-PDF Grammar (`STAMPALBUM 1`) | Status | Exactness & Semantic Notes |
|---|---|---|---|---|
| `ALBUM_PAGES_SPACING` | `(horizontal vertical)` in mm | `ALBUM_PAGES_SPACING (horizontal vertical)` | **EXACT** | Identical name and arguments. Sets global horizontal and vertical inter-element gaps. |
| *(No AlbumEasy Equivalent)* | *(Not available in AlbumEasy)* | `NATIVE_STAMP_TEXT_POLICY (mode limit_unit limit_val h_pad v_pad text_gap_mm)` | **NATIVE ONLY** | **Solves Text Bleed**: AlbumEasy has no parameter for frame-to-text gap; captions frequently touch or collide with the bottom frame line. Native `text_gap_mm` (e.g. 2.5 mm) guarantees separation. `expand_footprint` allows wide descriptions up to 140%+ without font squishing. |

---

### C. Page Titles, Headers, Footers & Numbering

| AlbumEasy Command | AlbumEasy Grammar (4.2.1 / 4.5.0) | Native-PDF Grammar (`STAMPALBUM 1`) | Status | Exactness & Semantic Notes |
|---|---|---|---|---|
| `ALBUM_PAGES_TITLE` | `(Font Size "Title")` | `NATIVE_PAGE_TITLE (Font Size "Title" following_space_mm)` | **ADAPTED** | AlbumEasy calculates following vertical spacing implicitly using global `vertical`. Native requires an explicit 4th argument `following_space_mm` to prevent header-content collision. |
| `ALBUM_PAGES_HEADER` | `(Font Size Position "Text")` | `NATIVE_PAGE_HEADER (Font Size Pos "Text" pad_mm min_height_mm)` | **ADAPTED** | Native adds explicit margin padding and minimum height. |
| `ALBUM_PAGES_HEADER_NUMBER` | `(Font Size Pos [start] ["before"] ["after"])` | `NATIVE_PAGE_HEADER_NUMBER (Font Size Pos start "before" "after" pad min_h [colour])` or `ALBUM_PAGES_HEADER_NUMBER (3-6 args)` | **COMPATIBLE** | Native `.stampalbum` accepts the 3-6 argument AlbumEasy form or the 8-9 argument extended native form. |
| `ALBUM_PAGES_FOOTER_NUMBER` | `(Font Size Pos [start] ["before"] ["after"])` | `NATIVE_PAGE_FOOTER_NUMBER (Font Size Pos start "before" "after" pad min_h colour)` | **ADAPTED** | Native requires explicit padding, minimum height, and colour. In raw `.txt` mode, `albumeasy::render_file` accepts the AlbumEasy 3-6 argument form. |
| `ALBUM_PAGES_HEADER_PAD` | `(padding_mm)` | Mapped in raw TXT & API | **EXACT** (Raw) | Signed mm padding shifting header toward/away from border. |
| `ALBUM_PAGES_FOOTER_PAD` | `(padding_mm)` | Mapped in raw TXT & API | **EXACT** (Raw) | Signed mm padding shifting footer toward/away from border. |
| `ALBUM_PAGES_MARGIN_TXT` | `(Font Size "Text" Pos Align Dir [pad] [colour])` | `NATIVE_PAGE_MARGIN_TEXT (Font Size "Text" Pos Align Dir gap_mm colour)` | **ADAPTED** | Native requires explicit border gap and colour; supports up to 2 definitions. |
| `ALBUM_PAGES_HEADER_DATE` / `FOOTER_DATE` | `(Font Size Pos Format ...)` | API / TOML only | **BACKLOG** | Not yet wired in native `.stampalbum` loader. |

---

### D. Page Flow & Structure

| AlbumEasy Command | AlbumEasy Grammar (4.2.1 / 4.5.0) | Native-PDF Grammar (`STAMPALBUM 1`) | Status | Exactness & Semantic Notes |
|---|---|---|---|---|
| `PAGE_START` | `PAGE_START` (with optional group substitutions) | `NATIVE_PAGE_BREAK ()` | **ADAPTED** | AlbumEasy uses `PAGE_START` as an imperative cursor reset. Native uses `NATIVE_PAGE_BREAK` to flush rows, start new pages, and preserve intentional blank pages. |
| `PAGE_START_VAR` | `PAGE_START_VAR (h v)` | `NATIVE_PAGE_BREAK (h v)` | **ADAPTED** | Overrides spacing for the current page. Identical semantics. |
| `PAGE_START_GROUP_BEGIN` / `END` | Group definition with `$$1`..`$$9` | Supported in raw `.txt` & `.stampalbum` | **COMPATIBLE** | Native raw interpreter expands single-pass group templates cleanly. |
| `PAGE_COLUMN_START` / `NEXT` / `STOP` | Column mode with optional gap padding | Not implemented | **BACKLOG** | Multi-column layout is currently supported only via external AlbumEasy. |
| `PAGE_TEXT`, `PAGE_TEXT_CENTRE`, `PAGE_TEXT_RIGHT` | `(Font Size "Text")` | Mapped in raw TXT & `.stampalbum` | **COMPATIBLE** | Supported in raw AlbumEasy compatibility mode; native engine handles text through titles, headings, and stamp captions. |
| `PAGE_TEXT_PARAGRAPH_START` / `END` | Block text with justification | Not implemented | **BACKLOG** | Native flow engine does not yet have a free paragraph block parser. |
| `PAGE_VSPACE` | `(vertical_mm)` | Supported in raw TXT (`albumeasy.rs`) | **COMPATIBLE** (Raw) | Signed vertical cursor displacement. |
| `PAGE_RULE_H` | `(thickness [gap] [colour])` | Not implemented | **BACKLOG** | Horizontal dividing rules. |
| *(No AlbumEasy Equivalent)* | *(Not available)* | `NATIVE_PAGE_NUMBER_SECTION ()` | **NATIVE ONLY** | Enables discrete section numbering with `$SECTION_PAGE$` and `$SECTION_PAGES$`. |

---

### E. Row Layout & Alignment

| AlbumEasy Command | AlbumEasy Grammar (4.2.1 / 4.5.0) | Native-PDF Grammar (`STAMPALBUM 1`) | Status | Exactness & Semantic Notes |
|---|---|---|---|---|
| `ROW_START_FS` | `(Font Size LineWidth Gap)` | Implicit in Native; forced by `NATIVE_ROW_BREAK` | **DIFFERENT** | AlbumEasy requires explicit row start declarations for fixed-spacing rows. Native flows stamps automatically, using `NATIVE_ROW_BREAK` when an explicit split is desired. |
| `ROW_START_ES` | `(Font Size LineWidth)` | `NATIVE_ROW_ALIGNMENT (center)` | **ADAPTED** | Evenly spaced rows in AlbumEasy divide remaining margin space. Native achieves this via row alignment policies. |
| `ROW_START_JS` | Justified row | `NATIVE_ROW_ALIGNMENT (justified)` | **ADAPTED** | Justifies items across the full row width. |
| `ROW_ALIGN_TOP` | `ROW_ALIGN_TOP` | `NATIVE_ROW_VERTICAL_ALIGNMENT (TOP)` | **COMPATIBLE** | Sets top vertical alignment for unequal stamp heights in a row. |
| `ROW_ALIGN_MIDDLE` | `ROW_ALIGN_MIDDLE` | `NATIVE_ROW_VERTICAL_ALIGNMENT (MIDDLE)` | **COMPATIBLE** | Centers stamps vertically along the row axis. |
| `ROW_ALIGN_BOTTOM` | `ROW_ALIGN_BOTTOM` | `NATIVE_ROW_VERTICAL_ALIGNMENT (BOTTOM)` | **COMPATIBLE** | Aligns stamps along the bottom baseline of the row. |

---

### F. Stamps, Frames, and Images

| AlbumEasy Command | AlbumEasy Grammar (4.2.1 / 4.5.0) | Native-PDF Grammar (`STAMPALBUM 1`) | Status | Exactness & Semantic Notes |
|---|---|---|---|---|
| `STAMP_ADD_IMG` | `(w h "path" "cap_L" "cap_C" "cap_R" ...)` | `STAMP_IMAGE ("path" w h scale "caption")` | **ADAPTED** | AlbumEasy bundles dimension, image path, and 3-part captions into one command. Native uses `STAMP_IMAGE` with chained attachments (`NATIVE_STAMP_HEADING`, `NATIVE_STAMP_FOOTER_TEXT`, etc.). |
| `STAMP_ADD` (Text Only) | `(w h "inside_text" "cap_L" "cap_C" "cap_R")` | Not yet mapped | **BACKLOG** | Native currently models image-backed stamp frames (`STAMP_IMAGE`). |
| Shapes (`DIAMOND`, `OVAL`, `TRIANGLE`, `HEXAGON`, `OCTAGON`, `PENTAGON`) | Various geometry parameters | Not implemented in Native | **BACKLOG** | Non-rectangular frame shapes are exclusive to AlbumEasy. |
| `ALBUM_STAMP_IMG_NEW_SETTING` | `(h_pad v_pad stretch)` in mm | `NATIVE_STAMP_IMAGE_POLICY (millimetres h v stretch aspect colour visible)` | **ADAPTED** | Native consolidates padding, aspect ratio, grayscale, and visibility into a unified 7-parameter policy. |
| `ALBUM_STAMP_IMG_NEW_SETTING_PCNT` | `(h_pcnt v_pcnt stretch)` | `NATIVE_STAMP_IMAGE_POLICY (percent h v stretch aspect colour visible)` | **ADAPTED** | Percentage padding variant. |
| `ALBUM_STAMP_IMG_ASPECT_RATIO` | `(preserve \| fit)` | Subsumed into `NATIVE_STAMP_IMAGE_POLICY` | **ADAPTED** | Parameter 5 in native policy. |
| `ALBUM_STAMP_IMG_GREYSCALE_ON/OFF` | Parameterless toggles | Subsumed into `NATIVE_STAMP_IMAGE_POLICY` | **ADAPTED** | Parameter 6 (`colour` vs `grayscale`). |
| `ALBUM_STAMP_IMG_HIDE/SHOW` | Parameterless toggles | Subsumed into `NATIVE_STAMP_IMAGE_POLICY` | **ADAPTED** | Parameter 7 (`show` vs `hide`). |
| `ALBUM_STAMP_SETTINGS_SAVE` / `RESTORE` | Parameterless commands | `NATIVE_STAMP_SETTINGS_SAVE` / `RESTORE` | **EXACT** | Saves and restores image options, borders, heading padding, and footer styles. |
| `STAMP_HEADING` / `_VA` | `(Font Size "Text" [dx] [dy])` | `NATIVE_STAMP_HEADING (LOCAL\|VA Font Size "Text" dx dy colour)` | **ADAPTED** | Native requires explicit mode keyword, dx, dy, and colour. |
| `STAMP_HEADING_PADDING` | `(pad_mm)` | `NATIVE_STAMP_HEADING_PADDING (pad_mm)` | **EXACT** | Signed vertical padding between stamp frame and heading. |
| `STAMP_FOOTER_TXT_ALIGN` | `(LEFT \| CENTRE \| RIGHT)` | `NATIVE_STAMP_FOOTER_TEXT_ALIGN (align)` | **EXACT** | Sets alignment for bottom caption attachments. |
| `STAMP_FOOTER_TXT_PAD` | `(pad_mm)` | `NATIVE_STAMP_FOOTER_TEXT_PAD (pad_mm)` | **EXACT** | Signed padding for below-frame text. |
| `STAMP_INNER_BORDER` | `(pad thickness style colour)` | `NATIVE_STAMP_INNER_BORDER (pad thickness style colour)` | **EXACT** | Identical 4-argument syntax for inner mounting frames. |
| `STAMP_BORDER_STYLE` / `INNER_` | `(style [selectors])` | `NATIVE_STAMP_BORDER_STYLE (style [selectors])` | **EXACT** | Supports `SOLID`, `DASHED`, `DOTTED`, `DOUBLE`, and per-side selectors (`TOP`, `BOTTOM`, `LEFT`, `RIGHT`, `SIDES`, `CORNER`). |

---

### G. Colors & Color Formats

| Feature | AlbumEasy Syntax | Native-PDF Syntax | Status | Comparison |
|---|---|---|---|---|
| Command Prefix | Both `COLOUR_*` and `COLOR_*` | Accepts both `COLOUR_*` and `COLOR_*` | **EXACT** | Native parser normalizes both spellings and mixed-case keywords. |
| Accepted Color Values | Named colors (`black`, `navy`, `darkgreen`, `grey`, `red`, etc.) | Named colors (`black`, `navy`, `darkgreen`, `grey`, `red`, etc.) | **EXACT** | Both engines resolve identical standard X11/SVG colour palettes. |
| Hexadecimal / RGB Values | **Rejected** by AlbumEasy 4.5.0 | Backlog (Issue #107) | **EXACT** | Neither engine currently accepts `#RRGGBB` or `rgb(r,g,b)` in album commands. |

---

### H. Preprocessing Directives

| Directive | AlbumEasy Behavior | Native-PDF Behavior | Status |
|---|---|---|---|
| `$INCLUDE "file.txt"` | Merges included file in place | Not implemented | **BACKLOG** |
| `$DEFINE` / `$UNDEFINE` | Defines/undefines conditional macros | Not implemented | **BACKLOG** |
| `$IFDEF` / `$ELSEIF` / `$ELSE` / `$ENDIF` | Conditional compilation | Not implemented | **BACKLOG** |

---

## 3. Practical Implications for Stamp Collectors

### Why Native `.stampalbum` Files Won't Compile in AlbumEasy
AlbumEasy's interpreter was written in C++ with a strict, closed command dictionary. When it encounters modern native commands like:
```text
NATIVE_STAMP_FRAME (0.25 black 4)
NATIVE_STAMP_TEXT_POLICY (expand_footprint percent 140 2.0 1.0 2.5)
NATIVE_PAGE_TITLE (TB 16 "Switzerland" 12)
```
AlbumEasy halts with syntax errors (`Unrecognised command`). This is intentional: `STAMPALBUM 1` documents exploit advanced typography, high-resolution vector clipping, and adaptive text wrapping that AlbumEasy's 20-year-old layout engine cannot produce.

### Benchmark & Side-by-Side Comparison Workflow
To benchmark layout fidelity:
1. **Raw Mode (`*.txt` or `*-raw.stampalbum`)**: Compiled with both AlbumEasy (`AlbumEasy -c`) and our native compatibility layer (`albumeasy::render_file`). This produces benchmark reference PDFs for exact comparison of lines, margins, and text placement.
2. **Native Mode (`*.stampalbum`)**: Rendered using `NativeAlbum::write_pdf`. This showcases high-DPI scans, zero text bleed, and dynamic multi-row flow.
