// ==============================================================================
// Section 5: Technical Engine & Command Architecture
// ==============================================================================

#pagebreak()
= 5. Technical Engine & Command Architecture

StampAlbumGen2 is engineered from the ground up in high-performance Rust, combining a lightning-fast vector layout planner with an uncompromising, publication-grade native PDF renderer.

Whether you generate albums through the point-and-click Review Screen or write AlbumEasy `.stampalbum` script files directly, this section details the precision mechanics, coordinate models, and command vocabulary powering the application.

== 5.1 Absolute Precision Geometry & Internal Coordinate Model

Philatelic mounts require exact, millimeter-accurate positioning. Even a fraction of a millimeter error can cause mounts to buckle or crowd neighboring rows.

To deliver uncompromising dimensional precision across platforms and print drivers:
- *Internal Representation:* All lengths, coordinates, offsets, and clearances are represented internally as 64-bit floating-point millimetres (`f64`).
- *PDF Coordinate Translation:* Dimensions are converted to Adobe PostScript / PDF points at output time using the exact standard philatelic conversion ratio:
  $ 1 "pt" = 25.4 / 72 "mm" approx 0.3527777777777778 "mm" $
  $ 1 "mm" = 72 / 25.4 "pt" approx 2.834645669291339 "pt" $
- *Rounding Defense:* All line endings, corner radii, and perimeter offsets are snapped to $0.001 "mm"$ ($1 mu"m"$), eliminating cumulative rounding drift across long multi-column stamp arrays.

== 5.2 Origin Coordinates & Box Placement Paradigms

StampAlbumGen2 supports both the legacy AlbumEasy coordinate paradigm and the modern, auto-centering native layout engine.

#table(
  columns: (1fr, 1.2fr, 2fr),
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#1B365D")) } else { 0.5pt + rgb("#E4E7EC") },
  fill: (x, y) => if y == 0 { rgb("#F0F4F8") } else if calc.even(y) { rgb("#FAFAFA") } else { none },
  inset: 7pt,
  [*Coordinate Paradigm*], [*Origin Reference*], [*Typical Philatelic Application*],
  [AlbumEasy Row Grid], [Bottom-left or top-left of row container], [Automated flowing catalogs where stamps are added sequentially with `STAMP_ADD`.],
  [Explicit Cartesian (`STAMP_AT`)], [Top-left margin intersection], [Precision custom exhibits, souvenir sheets, and asymmetric cover displays.],
  [Centered Mount Grid], [Horizontal center of usable printable area], [Balanced presentation pages with uniform left and right visual margins.],
)

== 5.3 Stamp Frame Hierarchy & Mount Construction

A single stamp placement in StampAlbumGen2 consists of up to four concentric, mathematically aligned bounding frames:

#enum(
  [
    *Outer Cut Perimeter:* The boundary where you cut the mount film. Controlled by mount clearance settings (default: stamp width + 3 mm, stamp height + 3 mm).
  ],
  [
    *Frame Border Line:* The solid, dashed, or colored vector stroke printed onto the album page. Can be customized with `NATIVE_STAMP_FRAME` (line width, color, corner radius).
  ],
  [
    *Inner Accent Frame:* An optional secondary recessed or decorative inner border line, configured via `NATIVE_STAMP_INNER_FRAME`.
  ],
  [
    *Stamp Paper Area:* The exact physical paper boundary of the original postage stamp.
  ],
)

- *Centering:* Stamp specimen images or text are positioned inside the frame according to the active `NATIVE_STAMP_IMAGE_POLICY`.
- *Mount Padding:* Clearances are distributed symmetrically around the center axis, ensuring the printed frame always frames your stamp perfectly on all four sides.

== 5.4 Page Border Mechanics & Multi-Line Framing Rules

Album page borders provide visual structure and elegance. StampAlbumGen2 offers comprehensive support for single, double, and triple vector framing rules via `ALBUM_PAGES_BORDER (w_outer w_inner1 w_inner2 gap)`.

#table(
  columns: (1.5fr, 2fr, 2.5fr),
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#1B365D")) } else { 0.5pt + rgb("#E4E7EC") },
  fill: (x, y) => if y == 0 { rgb("#F0F4F8") } else if calc.even(y) { rgb("#FAFAFA") } else { none },
  inset: 7pt,
  [*Border Style*], [*Command Syntax*], [*Visual Character*],
  [Classic Single Line], [`ALBUM_PAGES_BORDER (0.3 0.0 0.0 0.0)`], [Clean, understated 0.3 mm perimeter line. Standard for modern definitives.],
  [Double-Line Framing], [`ALBUM_PAGES_BORDER (0.5 0.2 0.0 1.0)`], [Heavy 0.5 mm outer frame with a delicate 0.2 mm inner rule separated by 1 mm.],
  [Deluxe Triple-Line], [`ALBUM_PAGES_BORDER (0.1 0.5 0.2 1.25)`], [Ornate presentation framing with a prominent center accent for souvenir sheets and title pages.],
  [Recessed Inner Line], [`ALBUM_PAGES_BORDER (0.0 0.3 0.0 1.25)`], [Floating inset rule offset from the page margin bounds by the gap distance.],
  [Borderless / Omitted], [`ALBUM_PAGES_BORDER (0.0 0.0 0.0 0.0)`], [Disables border strokes entirely for modern full-bleed or minimalist page designs.],
)

=== Automatic Content Inset Calculation
The vector engine dynamically computes the total border occupancy inset ($"occupancy"_"mm"$) reserving spacing only for active border rules:
$ "occupancy" = w_"outer" + sum_(i in {"inner"_1, "inner"_2}, w_i > 0) (w_i + "gap") $
This mathematical guarantee ensures that title blocks, header captions, and stamp mounts never encroach upon or collide with page border strokes.

== 5.5 Decorative Page Borders & Vintage Page Ornaments (`decorative-border-v1`)

For commemorative presentation exhibits, historical classics, and luxury album layouts, StampAlbumGen2 provides full native support for ornate decorative page borders via `ALBUM_PAGES_DECORATIVE_BORDER`.

A decorative border is composed of high-resolution transparent vector or raster page ornaments positioned at page corners and along margin centers, connected by precision vector framing rules:

- *Corner Ornaments (`IMAGE_CORNER`):* Placed at the top-left and automatically rotated / mirrored for top-right, bottom-right, and bottom-left.
- *Edge Ornaments (`IMAGE_EDGE_TOP`, `IMAGE_EDGE_BOTTOM`, `IMAGE_EDGE_LEFT`, `IMAGE_EDGE_RIGHT`):* Centered along margins to create classic framed exhibits.
- *Color Tinting (`COLOUR_ALBUM_DECORATIVE_BORDER`):* Re-tints black monochrome ornaments into any philatelic ink shade (such as dark navy, sepia, forest green, or burgundy) at render time.

== 5.6 Command Vocabulary Quick Reference

The following table summarizes the most commonly used commands across the layout engine:

#table(
  columns: (1.5fr, 1.2fr, 2.5fr),
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#1B365D")) } else { 0.5pt + rgb("#E4E7EC") },
  fill: (x, y) => if y == 0 { rgb("#F0F4F8") } else if calc.even(y) { rgb("#FAFAFA") } else { none },
  inset: 6pt,
  [*Command*], [*Arguments*], [*Purpose & Philatelic Function*],
  [`PAGE_START`], [—], [Begins a new album page and resets row cursors.],
  [`PAGE_START_VAR` / `NATIVE_PAGE_START_VAR`], [`(horizontal vertical)`], [Applies custom horizontal and vertical spacing strictly to the current page.],
  [`ALBUM_PAGES_SIZE`], [`(width height)`], [Sets album page dimensions in mm (e.g. `(215.9 279.4)` for US Letter).],
  [`ALBUM_PAGES_MARGINS`], [`(left right top bottom)`], [Sets printable area margins in millimetres.],
  [`ALBUM_PAGES_BORDER`], [`(outer in1 in2 gap)`], [Configures single, double, or triple vector border rules.],
  [`PAGE_TITLE`], [`(font size "text" gap)`], [Prints a centered page title block with spacing below.],
  [`PAGE_HEADER`], [`(font size align "text" dx dy)`], [Places running header text at the top of the page.],
  [`PAGE_FOOTER`], [`(font size align "text" dx dy)`], [Places running footer text at the bottom of the page.],
  [`STAMP_ADD`], [`(width height "caption")`], [Appends a new stamp frame to the current active row.],
  [`ROW_START`], [—], [Advances to the next vertical row on the page.],
  [`ROW_ALIGN_BOTTOM`], [—], [Aligns stamps along their bottom baseline instead of top edge.],
  [`NATIVE_STAMP_FRAME`], [`(width colour radius)`], [Customizes stamp border stroke width, color, and rounded corners.],
  [`NATIVE_STAMP_INNER_FRAME`], [`(width colour gap)`], [Adds a recessed secondary inner accent line inside the stamp box.],
  [`NATIVE_STAMP_IMAGE_POLICY`], [`(unit pad_x pad_y fit aspect col show)`], [Controls specimen stamp image scaling, padding, and grayscale conversion.],
  [`IMAGE_SCALE`], [`(factor)`], [Scales ornament graphics (e.g. `0.12` or `0.24` for 300/600 DPI scans at 72 DPI layout).],
  [`IMAGE_CORNER`], [`("filename.png")`], [Corner ornament PNG placed at the top-left; automatically rotated and mirrored for all 4 corners.],
  [`IMAGE_CORNER_PAD`], [`(dx dy)`], [Horizontal and vertical offsets in mm from corner margin intersections.],
  [`IMAGE_EDGE_TOP`], [`("filename.png")`], [Top margin ornament centered horizontally. Flipped automatically when used for bottom.],
  [`IMAGE_EDGE_BOTTOM`], [`("filename.png")`], [Distinct bottom margin ornament centered horizontally.],
  [`IMAGE_EDGE_LEFT`], [`("filename.png")`], [Left margin ornament centered vertically. Flipped automatically when used for right.],
  [`IMAGE_EDGE_RIGHT`], [`("filename.png")`], [Distinct right margin ornament centered vertically.],
  [`IMAGE_EDGE_PAD`], [`(dx dy)`], [Shifts edge ornaments outward or inward relative to margin perimeters.],
  [`IMAGE_BORDER_PAD`], [`(pad)`], [Expands or contracts the enclosing frame line to fit decorative ornaments.],
  [`COLOUR_ALBUM_DECORATIVE_BORDER`], [`(colour)`], [Applies a uniform philatelic ink tint to all page ornaments.],
)
