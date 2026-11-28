// ==============================================================================
// Section 6: Comprehensive Illustrated Command Catalog (All 43 Families)
// ==============================================================================

#pagebreak()
= 6. Comprehensive Illustrated Command Catalog

This chapter catalogs all 43 command families featured on the official website gallery (#link("https://stampalbumgenerator2.com/pages/examples.html")[`stampalbumgenerator2.com`]). Each entry provides exact syntax, philatelic intent, AlbumEasy vs Native code examples, and visual layout specifications.

#let command_entry(name, category, syntax_str, desc, albumeasy_code, native_code, thumb_name: "") = {
  v(12pt)
  block(
    fill: rgb("#FFFFFF"),
    stroke: 0.5pt + rgb("#D0D5DD"),
    inset: 12pt,
    radius: 4pt,
    width: 100%,
    [
      #grid(
        columns: (3fr, 1.5fr),
        gutter: 10pt,
        [
          #text(size: 13pt, weight: "bold", fill: rgb("#1B365D"))[#name] \
          #text(size: 8.5pt, fill: rgb("#2B5B84"), weight: "bold")[Category: #category]
          #v(4pt)
          #text(size: 9.5pt)[#desc]
          #v(6pt)
          #text(size: 8.5pt, weight: "bold", fill: rgb("#344054"))[Syntax Specification:] \
          #text(size: 8.5pt, font: "Consolas", fill: rgb("#027A48"))[`#syntax_str`]
        ],
        [
          #align(center + horizon)[
            #rect(
              width: 100%,
              height: 75pt,
              stroke: 0.5pt + rgb("#CBD5E1"),
              fill: rgb("#F8FAFC"),
              radius: 3pt,
              [
                #align(center + horizon)[
                  #text(size: 7.5pt, fill: rgb("#64748B"), weight: "bold")[Visual Proof] \
                  #text(size: 6.5pt, fill: rgb("#94A3B8"))[#thumb_name]
                ]
              ]
            )
          ]
        ]
      )

      #v(6pt)
      #grid(
        columns: (1fr, 1fr),
        gutter: 8pt,
        [
          #text(size: 8pt, weight: "bold", fill: rgb("#475467"))[AlbumEasy Format (.txt)] \
          #block(fill: rgb("#F8F9FA"), inset: 6pt, radius: 3pt, width: 100%)[
            #text(size: 7.5pt, font: "Consolas")[#albumeasy_code]
          ]
        ],
        [
          #text(size: 8pt, weight: "bold", fill: rgb("#475467"))[Native Vector Format (.stampalbum)] \
          #block(fill: rgb("#F8F9FA"), inset: 6pt, radius: 3pt, width: 100%)[
            #text(size: 7.5pt, font: "Consolas")[#native_code]
          ]
        ]
      )
    ]
  )
}

== 6.1 Category 1: Page Setup & Layout

#command_entry(
  "Page Dimensions (page-size-v1)",
  "Page Setup & Layout",
  "ALBUM_PAGES_SIZE / NATIVE_PAGES_SIZE (width_mm height_mm)",
  "Defines the global MediaBox dimensions. Orientation is determined directly by width vs height values.",
  "ALBUM_PAGES_SIZE (210.0 297.0)\nPAGE_START",
  "STAMPALBUM 1\nNATIVE_PAGES_SIZE (210.0 297.0)",
  thumb_name: "page-size-v1--a4-portrait.png"
)

#command_entry(
  "Page Margins & Clearances (margins-v1)",
  "Page Setup & Layout",
  "ALBUM_PAGES_MARGINS / NATIVE_PAGES_MARGINS (left right top bottom) / ALBUM_PAGES_MARGINSE / NATIVE_PAGES_MARGINSE (left right top bottom)",
  "Defines printable boundaries and independent alternating margins for odd and even pages in book-bound albums.",
  "ALBUM_PAGES_MARGINS (25.0 12.0 18.0 21.0)\nALBUM_PAGES_MARGINSE (12.0 25.0 18.0 21.0)",
  "ALBUM_PAGES_MARGINS (25.0 12.0 18.0 21.0)\nALBUM_PAGES_MARGINSE (12.0 25.0 18.0 21.0)",
  thumb_name: "margins-v1--even-override.png"
)

#command_entry(
  "Row & Stamp Spacing (spacing-v1)",
  "Page Setup & Layout",
  "ALBUM_PAGES_SPACING (horizontal_mm vertical_mm)",
  "Sets standard clearance between adjacent stamps in a row and spacing between successive rows.",
  "ALBUM_PAGES_SPACING (4.0 8.0)",
  "ALBUM_PAGES_SPACING (4.0 8.0)",
  thumb_name: "spacing-v1--combined.png"
)

#command_entry(
  "Page Breaks & Group Templates (page-start-v1)",
  "Page Setup & Layout",
  "PAGE_START / NATIVE_PAGE_START / NATIVE_PAGE_BREAK ()",
  "Triggers explicit physical page boundaries, group template parameter expansions, and deliberate blank pages.",
  "PAGE_START
PAGE_START (\"United States\" 1959 \"Liberty Issue\")",
  "NATIVE_PAGE_START ()
NATIVE_PAGE_BREAK (4.0 14.0)",
  thumb_name: "page-start-v1--two-pages.png"
)

#command_entry(
  "Variable Page Spacing (page-start-var-v1)",
  "Page Setup & Layout",
  "PAGE_START_VAR / NATIVE_PAGE_START_VAR (horizontal_mm vertical_mm)",
  "Overrides horizontal stamp and vertical row spacing on a single target page without modifying global album defaults.",
  "PAGE_START_VAR (12.0 6.0)\nPAGE_START_VAR (0.0 0.0)",
  "NATIVE_PAGE_START_VAR (12.0 6.0)\nNATIVE_PAGE_BREAK (0.0 0.0)",
  thumb_name: "page-start-var-v1--album.png"
)

#command_entry(
  "Printer Crop Marks (crop-marks-v1)",
  "Page Setup & Layout",
  "ALBUM_PAGES_CROP_MARKS (arms_mm pad_x pad_y line_w colour)",
  "Renders professional corner crop marks aiding collectors in trimming custom sheets to binder specifications.",
  "ALBUM_PAGES_CROP_MARKS (5.0 5.0 5.0 0.2 black)",
  "ALBUM_PAGES_CROP_MARKS (5.0 5.0 5.0 0.2 black)",
  thumb_name: "crop-marks-v1--crop-default.png"
)

== 6.2 Category 2: Page Borders & Decorative Framing

#command_entry(
  "Single, Double & Triple Borders (border-v1)",
  "Page Borders & Decorative Framing",
  "ALBUM_PAGES_BORDER (outer_w middle_w inner_w gap_mm) [or 3-arg: outer inner gap]",
  "Renders elegant single, double, or triple framing rules around page margins with exact line weights and separation gaps. Ideal for classic double-frame archival pages or deluxe triple-line commemorative presentation pages. When omitted, no border is printed.",
  "ALBUM_PAGES_BORDER (0.5 0.1 0.0 1.25)",
  "ALBUM_PAGES_BORDER (0.5 0.1 0.0 1.25)\nCOLOUR_ALBUM_BORDER (black)",
  thumb_name: "border-v1--double.png"
)

#command_entry(
  "Border Colors & Tints (border-colour-v1)",
  "Page Borders & Decorative Framing",
  "COLOUR_ALBUM_BORDER (colour_name)",
  "Applies named RGB colors or grayscale tints to page perimeter borders.",
  "COLOUR_ALBUM_BORDER (darkblue)",
  "COLOUR_ALBUM_BORDER (darkblue)",
  thumb_name: "border-colour-v1--named.png"
)

#command_entry(
  "Decorative Ornate Borders (decorative-border-v1)",
  "Page Borders & Decorative Framing",
  "ALBUM_PAGES_DECORATIVE_BORDER (\"border_definition_file\")",
  "Embeds intricate vintage corner flourishes, margin edge ornaments, and vector framing rules around the album perimeter defined in an external border specification.",
  "ALBUM_PAGES_DECORATIVE_BORDER (\"Classic.txt\")",
  "ALBUM_PAGES_DECORATIVE_BORDER (\"borders/ImperialExhibition.txt\")\nCOLOUR_ALBUM_DECORATIVE_BORDER (navy)",
  thumb_name: "decorative-border-v1--classic-showcase.png"
)

#command_entry(
  "Decorative Border Color (decorative-border-colour-v1)",
  "Page Borders & Decorative Framing",
  "COLOUR_ALBUM_DECORATIVE_BORDER (colour_name)",
  "Specifies the stroke color for vector framing lines drawn within decorative borders (defaults to black). Supports all standard named colors.",
  "COLOUR_ALBUM_DECORATIVE_BORDER (darkblue)",
  "COLOUR_ALBUM_DECORATIVE_BORDER (darkblue)",
  thumb_name: "decorative-border-v1--edge-mixed.png"
)

#command_entry(
  "Color Formats & Hex/RGB Support (color-formats-v1)",
  "Page Borders & Decorative Framing",
  "COLOUR_ALBUM_BORDER (color) / COLOR_ALBUM_BORDER (color)",
  "Universal color parser supporting 6-digit hex (#RRGGBB), 3-digit shorthand hex (#RGB), bare hex (RRGGBB), integer RGB rgb(r,g,b), percentage RGB rgb(r%,g%,b%), and SVG/CSS named colors across all border, text, and stamp commands.",
  "COLOUR_ALBUM_BORDER (\"#1E4A7A\")",
  "COLOUR_ALBUM_BORDER (#1E4A7A)\nNATIVE_STAMP_FRAME (0.5 \"rgb(30, 74, 122)\" 1.5)",
  thumb_name: "color-formats-v1--hex-6digit.png"
)

== 6.3 Category 3: Headers, Footers & Titles

#command_entry(
  "Album & Page Titles (title-v1)",
  "Headers, Footers & Titles",
  "PAGE_TITLE / NATIVE_PAGE_TITLE (font size \"text\" gap_mm)",
  "Generates bold commemorative series titles positioned above stamp content.",
  "PAGE_TITLE (TB 16 \"UNITED STATES - LIBERTY ISSUE\")",
  "NATIVE_PAGE_TITLE (TB 16 \"UNITED STATES - LIBERTY ISSUE\" 5)",
  thumb_name: "title-v1--album.png"
)

#command_entry(
  "Running Page Headers (header-v1)",
  "Headers, Footers & Titles",
  "NATIVE_PAGE_HEADER (font size align \"text\" offset_x offset_y)",
  "Repeating top running header text supporting Left, Center, Right, or Alternating parity.",
  "ALBUM_PAGES_HEADER (HN 9 C \"1959 COMMEMORATIVES\")",
  "NATIVE_PAGE_HEADER (HN 9 C \"1959 COMMEMORATIVES\" 0 0)",
  thumb_name: "header-v1--centre.png"
)

#command_entry(
  "Header Page Numbering (header-number-v1)",
  "Headers, Footers & Titles",
  "NATIVE_PAGE_HEADER_NUMBER (font size align start \"prefix\" \"suffix\" dx dy colour)",
  "Top-margin automated folio numbering with total page count substitution (`$PAGES$`).",
  "ALBUM_PAGES_HEADER_NUMBER (HN 8 R 1 \"Page \" \" of $PAGES$\")",
  "NATIVE_PAGE_HEADER_NUMBER (HN 8 R 1 \"Page \" \" of $PAGES$\" 0 0 black)",
  thumb_name: "header-number-v1--header-number-basic.png"
)

#command_entry(
  "Running Page Footers (footer-v1)",
  "Headers, Footers & Titles",
  "NATIVE_PAGE_FOOTER (font size align \"text\" offset_x offset_y)",
  "Bottom margin annotations and publisher notices.",
  "ALBUM_PAGES_FOOTER (HN 8 C \"Laser Spider LLC / Collector Edition\")",
  "NATIVE_PAGE_FOOTER (HN 8 C \"Laser Spider LLC / Collector Edition\" 0 0)",
  thumb_name: "footer-v1--centre.png"
)

#command_entry(
  "Footer Page Numbering (footer-number-v1)",
  "Headers, Footers & Titles",
  "NATIVE_PAGE_FOOTER_NUMBER (font size align start \"prefix\" \"suffix\" dx dy colour)",
  "Bottom-of-page automated folio numbering with custom prefixes.",
  "ALBUM_PAGES_FOOTER_NUMBER (HN 8 C 1 \"Sheet \" \"\")",
  "NATIVE_PAGE_FOOTER_NUMBER (HN 8 C 1 \"Sheet \" \"\" 0 0 navy)",
  thumb_name: "footer-number-v1--normal.png"
)

#command_entry(
  "Vertical Margin Text (margin-text-v1)",
  "Headers, Footers & Titles",
  "NATIVE_PAGE_MARGIN_TEXT (font size \"text\" position align dir gap colour)",
  "Rotated vertical marginalia (90° and 270°) along left or right page edges for country branding.",
  "ALBUM_PAGES_MARGIN_TXT (HN 8 \"UNITED STATES\" L CENTRE UP 3.0)",
  "NATIVE_PAGE_MARGIN_TEXT (HN 8 \"UNITED STATES\" L CENTRE UP 3.0 darkblue)",
  thumb_name: "margin-text-v1--top-down.png"
)

#command_entry(
  "Album Author & Metadata (album-author-v1)",
  "Headers, Footers & Titles",
  "ALBUM_AUTHOR (\"name\") / ALBUM_TITLE (\"title\")",
  "Embeds formal collector copyright and title metadata directly into the PDF document dictionary properties.",
  "ALBUM_AUTHOR (\"John Doe\")\nALBUM_TITLE (\"Liberty Issue\")",
  "ALBUM_AUTHOR (\"John Doe\")\nALBUM_TITLE (\"Liberty Issue\")",
  thumb_name: "album-author-v1--basic.png"
)

#command_entry(
  "Section Page Numbering (native-section-numbering-v1)",
  "Headers, Footers & Titles",
  "NATIVE_PAGE_NUMBER_SECTION ()",
  "Resets page numbering counters for independent sections (e.g. Definitives 1-4, Airmails 1-2).",
  "# Native feature",
  "NATIVE_PAGE_NUMBER_SECTION ()",
  thumb_name: "native-section-numbering-v1--section-demo.png"
)

== 6.4 Category 4: Stamp Frames, Shapes & Rows

#command_entry(
  "Stamp Mount Clearances (stamp-frame-v1)",
  "Stamp Frames, Shapes & Rows",
  "NATIVE_STAMP_FRAME (line_w colour addition_mm)",
  "Draws precision outer mount borders, adding uniform millimeter clearances for mount strips.",
  "STAMP_BOXES_SIZE_ADJUST (3.0 3.0)",
  "NATIVE_STAMP_FRAME (0.25 black 3.0)",
  thumb_name: "stamp-frame-v1--portrait.png"
)

#command_entry(
  "Non-Rectangular Stamp Shapes (stamp-shapes-v1)",
  "Stamp Frames, Shapes & Rows",
  "STAMP_TRIANGLE / STAMP_DIAMOND / STAMP_HEXAGON",
  "Renders geometric mount outlines for triangular, diamond, round, and octagonal world stamp issues.",
  "STAMP_TRIANGLE_POINT_UP (40 35)",
  "STAMP_TRIANGLE_POINT_UP (40 35)",
  thumb_name: "stamp-shapes-v1--triangle-up.png"
)

#command_entry(
  "Horizontal Row Alignment (row-alignment-v1)",
  "Stamp Frames, Shapes & Rows",
  "NATIVE_ROW_ALIGNMENT (left | center | right | justified)",
  "Controls how stamp items are distributed across the horizontal row width.",
  "ROW_ALIGN_CENTRE",
  "NATIVE_ROW_ALIGNMENT (center)",
  thumb_name: "row-alignment-v1--center.png"
)

#command_entry(
  "Vertical Row Alignment (row-vertical-alignment-v1)",
  "Stamp Frames, Shapes & Rows",
  "NATIVE_ROW_VERTICAL_ALIGNMENT (TOP | MIDDLE | BOTTOM)",
  "Aligns stamps of varying physical heights along a common top, middle, or bottom baseline.",
  "ROW_ALIGN_BOTTOM",
  "NATIVE_ROW_VERTICAL_ALIGNMENT (BOTTOM)",
  thumb_name: "row-vertical-alignment-v1--bottom.png"
)

== 6.5 Category 5: Stamp Images & Placement

#command_entry(
  "Image Scaling & Placement (stamp-image-controls-v1)",
  "Stamp Images & Placement",
  "NATIVE_STAMP_IMAGE_POLICY (unit pad_x pad_y fit_flag aspect colour show)",
  "Fine-grained control over stamp image scaling, padding, grayscale conversion, and frame visibility.",
  "# Global config settings",
  "NATIVE_STAMP_IMAGE_POLICY (millimetres 0 0 false preserve colour show)",
  thumb_name: "stamp-image-controls-v1--album.png"
)

#command_entry(
  "Aspect Ratio Preservation (stamp-aspect-ratio-v1)",
  "Stamp Images & Placement",
  "STAMP_IMAGE (path width height scale)",
  "Guarantees stamp specimen artwork is rendered without non-uniform stretching or aspect distortion.",
  "STAMP_IMAGE (\"us-150.png\" 52 29 0.05)",
  "STAMP_IMAGE (\"us-150.png\" 52 29 0.05)",
  thumb_name: "stamp-aspect-ratio-v1--preserve.png"
)

== 6.6 Category 6: Text, Headings & Captions

#command_entry(
  "Stamp Captions & Sizing (stamp-text-v1)",
  "Text, Headings & Captions",
  "STAMP_ADD_POST / NATIVE_STAMP_TEXT_POLICY",
  "Formats descriptive text lines beneath stamps, handling multi-line wrapping and font point size scaling.",
  "STAMP_ADD_POST (HN 8 \"1c Washington\\nGreen\")",
  "NATIVE_STAMP_TEXT_POLICY (wrap_to_frame percent 100 0 0 1.2)",
  thumb_name: "stamp-text-v1--multi-line.png"
)

#command_entry(
  "Section & Set Headings (stamp-headings-v1)",
  "Text, Headings & Captions",
  "NATIVE_STAMP_HEADING (LOCAL|VA font size \"text\" offset baseline_gap colour)",
  "Places prominent section headings directly above individual stamp rows with optional vertical alignment.",
  "STAMP_HEADING (\"TB\" 10 \"1959 ISSUES\")",
  "NATIVE_STAMP_HEADING_PADDING (0)\nNATIVE_STAMP_HEADING (LOCAL TB 10 \"1959 ISSUES\" 0 3.175 darkred)",
  thumb_name: "stamp-headings-v1--alignment.png"
)

#command_entry(
  "Sub-Mount Annotation Text (stamp-footer-text-v1)",
  "Text, Headings & Captions",
  "NATIVE_STAMP_FOOTER_TEXT (font size colour \"left\" \"centre\" \"right\")",
  "Draws structured three-column annotation text directly beneath stamp mounts for catalog, mint state, and perforation data.",
  "# Native feature",
  "NATIVE_STAMP_FOOTER_TEXT (HN 8 darkblue \"Scott #1040\" \"Mint NH\" \"Perf 11\")",
  thumb_name: "stamp-footer-text-v1--footer-text.png"
)

== 6.7 Category 7: Settings & Pagination

#command_entry(
  "Push & Pop Settings (stamp-settings-save-restore-v1)",
  "Settings & Pagination",
  "NATIVE_STAMP_SETTINGS_SAVE () / NATIVE_STAMP_SETTINGS_RESTORE ()",
  "Saves and restores framing, font, and padding states to isolate custom stamp layouts without global side-effects.",
  "# Native stack feature",
  "NATIVE_STAMP_SETTINGS_SAVE ()\n# ... custom style ...\nNATIVE_STAMP_SETTINGS_RESTORE ()",
  thumb_name: "stamp-settings-save-restore-v1--album.png"
)

#command_entry(
  "Stamp Frame Shading & Fills (stamp-background-colours-v1)",
  "Settings & Pagination",
  "NATIVE_STAMP_BACKGROUND (colour | none)",
  "Applies subtle shaded background fills inside stamp boxes to distinguish missing stamps or special varieties.",
  "STAMP_BOX_BACKGROUND (lightgrey)",
  "NATIVE_STAMP_BACKGROUND (lightgrey)",
  thumb_name: "stamp-background-colours-v1--album.png"
)

#command_entry(
  "Multi-Layer Mount Borders (stamp-inner-borders-v1)",
  "Settings & Pagination",
  "NATIVE_STAMP_INNER_BORDER (pad_mm thickness_mm style colour)",
  "Renders dashed or dotted inner guide lines inside mount frames simulating collector mount borders.",
  "# Native vector feature",
  "NATIVE_STAMP_INNER_BORDER (2.0 0.3 DASHED navy)",
  thumb_name: "stamp-inner-borders-v1--album.png"
)

== 6.8 Category 8: Real Stamp Collections

#command_entry(
  "Denmark Commemoratives (denmark-txt-native-v1)",
  "Real Stamp Collections",
  "Full Commemorative Collection",
  "16-page complete commemorative issue demonstration verifying 41 real image placements and multi-stamp set layouts.",
  "AlbumEasy Denmark 1989 Script",
  "STAMPALBUM 1 Denmark 1989 Vector Layout",
  thumb_name: "denmark-txt-native-v1--Denmark-1989-1989.png"
)
