// ==============================================================================
// Section 5: Vector Page Geometry, Coordinate Systems & Mount Standards
// ==============================================================================

#pagebreak()
= 5. Vector Page Geometry, Coordinate Systems & Mount Standards

== 5.1 Coordinate Spaces & Dimensional Units

Philatelic mounting demands absolute dimensional consistency. StampAlbumGen2 operates in exact millimetres ($"mm"$) internally and maps directly to the standard PostScript/PDF coordinate system (where 1 point = $1/72$ inch):

$ 1 "mm" = 72 / 25.4 "points" approx 2.83464567 "points" $

#table(
  columns: (2fr, 2fr, 2fr),
  fill: (x, y) => if y == 0 { rgb("#1B365D").lighten(85%) } else if calc.even(y) { rgb("#F8F9FA") } else { none },
  stroke: 0.5pt + rgb("#D0D5DD"),
  table.header([*Paper Format*], [*Dimensions (mm)*], [*PDF MediaBox (points)*]),
  [US Letter (Portrait)], [215.9 × 279.4 mm], [612.0 × 792.0 pt],
  [US Letter (Landscape)], [279.4 × 215.9 mm], [792.0 × 612.0 pt],
  [A4 (Portrait)], [210.0 × 297.0 mm], [595.28 × 841.89 pt],
  [A4 (Landscape)], [297.0 × 210.0 mm], [841.89 × 595.28 pt],
)

== 5.2 Alternating Margins for Book-Binding (`ALBUM_PAGES_MARGINSE`)

When printed pages are bound in classic philatelic ring binders (such as 3-ring, 4-ring, or springback albums), the binding edge requires an expanded margin clearance to prevent holes or binding posts from encroaching on stamp frames:

#grid(
  columns: (1fr, 1fr),
  gutter: 15pt,
  [
    #block(fill: rgb("#F8F9FA"), inset: 10pt, radius: 4pt, stroke: 0.5pt + rgb("#D0D5DD"))[
      *Odd Pages (Right-Hand Page)* \
      - Left Margin: *25.0 mm* (Binding Edge) \
      - Right Margin: *12.0 mm* (Outer Edge) \
      - Top Margin: *18.0 mm* \
      - Bottom Margin: *21.0 mm*
    ]
  ],
  [
    #block(fill: rgb("#F8F9FA"), inset: 10pt, radius: 4pt, stroke: 0.5pt + rgb("#D0D5DD"))[
      *Even Pages (Left-Hand Page)* \
      - Left Margin: *12.0 mm* (Outer Edge) \
      - Right Margin: *25.0 mm* (Binding Edge) \
      - Top Margin: *18.0 mm* \
      - Bottom Margin: *21.0 mm*
    ]
  ]
)

In native syntax, this alternating parity is declared in the document header:
```
ALBUM_PAGES_MARGINS (25.0 12.0 18.0 21.0)
ALBUM_PAGES_MARGINSE (12.0 25.0 18.0 21.0)
```

== 5.3 Stamp Physical Dimensions vs. Mount Clearances

A common error in stamp album design is drawing frames exactly the size of the stamp paper. Stamp mounts (such as Showgard or Hawid strips) require extra physical clearance around the stamp to ensure easy insertion and prevent damage to delicate perforations:

- *Stamp Physical Dimensions ($W times H$):* The true physical measurement of the stamp paper (e.g., $40.0 times 25.0 "mm"$).
- *Standard Mount Clearance:* Typically $+1.5 "mm"$ on all four sides ($+3.0 "mm"$ total width and height).
- *Drawn Frame:* $43.0 times 28.0 "mm"$.
- *Centering:* Stamp specimen images or text are positioned inside the frame according to the active `NATIVE_STAMP_IMAGE_POLICY`.

== 5.4 Concentric Page Border Framing (`ALBUM_PAGES_BORDER`)

Page borders define the outer visual perimeter of an album exhibit, creating a distinguished framing boundary between the album margins and stamp content.

The `ALBUM_PAGES_BORDER` command configures single, double, or triple concentric border frames:

#table(
  columns: (2fr, 2.5fr, 3.5fr),
  fill: (x, y) => if y == 0 { rgb("#1B365D").lighten(85%) } else if calc.even(y) { rgb("#F8F9FA") } else { none },
  stroke: 0.5pt + rgb("#D0D5DD"),
  table.header([*Framing Style*], [*Command Syntax*], [*Collector Application*]),
  [Classic Double-Line], [`ALBUM_PAGES_BORDER (0.5 0.1 0.0 1.25)`], [Traditional philatelic standard: bold 0.5 mm outer border paired with a crisp 0.1 mm inner hairline.],
  [Crisp Single-Line], [`ALBUM_PAGES_BORDER (0.25 0.0 0.0 1.25)`], [Clean, modern aesthetic ideal for colorful topical and modern commemorative issues.],
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

```
ALBUM_PAGES_DECORATIVE_BORDER ("borders/ImperialExhibition.txt")
COLOUR_ALBUM_DECORATIVE_BORDER (navy)
```

=== Decorative Border Definition Grammar
Each border design is defined in a standalone `.txt` specification file containing these commands:

#table(
  columns: (2.5fr, 2fr, 3.5fr),
  fill: (x, y) => if y == 0 { rgb("#1B365D").lighten(85%) } else if calc.even(y) { rgb("#F8F9FA") } else { none },
  stroke: 0.5pt + rgb("#D0D5DD"),
  table.header([*Command*], [*Arguments*], [*Purpose & Geometry*]),
  [`IMAGE_SCALE`], [`(factor)`], [Scales ornament graphics (e.g. `0.12` or `0.24` for 300/600 DPI scans at 72 DPI layout).],
  [`IMAGE_CORNER`], [`("filename.png")`], [Corner ornament PNG placed at the top-left; automatically rotated and mirrored for all 4 corners.],
  [`IMAGE_CORNER_PAD`], [`(dx dy)`], [Horizontal and vertical offsets in mm from corner margin intersections.],
  [`IMAGE_EDGE_TOP`], [`("filename.png")`], [Top margin ornament centered horizontally. Flipped automatically when used for bottom.],
  [`IMAGE_EDGE_BOTTOM`], [`("filename.png")`], [Distinct bottom margin ornament centered horizontally.],
  [`IMAGE_EDGE_LEFT`], [`("filename.png")`], [Left margin ornament centered vertically. Flipped automatically when used for right.],
  [`IMAGE_EDGE_RIGHT`], [`("filename.png")`], [Distinct right margin ornament centered vertically.],
  [`IMAGE_EDGE_PAD`], [`(dx dy)`], [Shifts edge ornaments outward or inward relative to margin perimeters.],
  [`BORDER_LINES`], [`(outer inner1 inner2 spacing)`], [Draws up to 3 vector framing rules connecting ornament positions.],
  [`BORDER_PAD`], [`(dx dy)`], [Perpendicular offsets shifting border lines outward (+) or inward (-).],
  [`BORDER_END_OFFSETS`], [`(dx dy)`], [Insets border line endpoints from corners to prevent overlapping corner ornaments.],
)

=== Built-in Border Styles & Search Path Resolution
When locating border definition files, the generator first searches the root `borders/` application library, followed by the folder containing the active album source document:

- *Classic Victorian (`Classic.txt`):* Stately corner acanthus scroll vignettes framed by balanced double rules.
- *Art Deco (`ArtDeco.txt`, `ArtDecoSmall.txt`):* Geometric stepped corner flourishes evoking 1920s–1930s design.
- *Baroque Corner Vignettes (`CornerStyleA.txt`, `CornerStyleB.txt`):* Ornate scrolled flourishes for classical issues.
- *Architectural Pillars (`CornerAndVerticalStyleC.txt`):* Stately fluted classical column rules along vertical margins.
- *Imperial Exhibition (`ImperialExhibition.txt`):* Deluxe custom showcase border uniting corner palmettes, classical pillar side rules, and refined double hairline framing.
