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
