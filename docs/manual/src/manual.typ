// ==============================================================================
// StampAlbumGen2 — Official Comprehensive User Manual & Command Catalog
// ==============================================================================

#set document(
  title: "StampAlbumGen2 Comprehensive User Manual & Command Catalog",
  author: "Laser Spider LLC",
  date: auto
)

#set page(
  paper: "us-letter",
  margin: (x: 20mm, top: 22mm, bottom: 22mm),
  header: context {
    let page_num = counter(page).get().first()
    if page_num > 2 [
      #grid(
        columns: (1fr, 1fr),
        align(left)[#text(size: 8pt, fill: rgb("#667085"))[StampAlbumGen2 Collector's Guide & Command Catalog]],
        align(right)[#text(size: 8pt, fill: rgb("#667085"))[Laser Spider LLC]]
      )
      #v(-4pt)
      #line(length: 100%, stroke: 0.4pt + rgb("#D0D5DD"))
    ]
  },
  footer: context {
    let page_num = counter(page).get().first()
    if page_num > 1 [
      #line(length: 100%, stroke: 0.4pt + rgb("#D0D5DD"))
      #v(2pt)
      #grid(
        columns: (1fr, 1fr),
        align(left)[#text(size: 8pt, fill: rgb("#667085"))[Official Website: stampalbumgenerator2.com]],
        align(right)[#text(size: 8pt, fill: rgb("#667085"))[Page #counter(page).display("1")]]
      )
    ]
  }
)

#set text(
  font: ("Segoe UI", "Arial", "Liberation Sans"),
  size: 10.5pt,
  fill: rgb("#1F2A37"),
  lang: "en"
)

#set par(
  justify: true,
  leading: 0.65em
)

// --- Cover Page ---
#align(center + horizon)[
  #block(
    stroke: 2pt + rgb("#1B365D"),
    inset: 24pt,
    radius: 6pt,
    fill: rgb("#F8FAFC"),
    width: 95%,
    [
      #v(10pt)
      #text(size: 26pt, weight: "bold", fill: rgb("#1B365D"))[StampAlbumGen2] \
      #v(6pt)
      #text(size: 14pt, weight: "medium", fill: rgb("#2B5B84"))[
        Comprehensive Collector's Manual, Review UI Guide & Illustrated Command Catalog
      ]
      #v(12pt)
      #line(length: 80%, stroke: 1pt + rgb("#CBD5E1"))
      #v(12pt)
      #text(size: 10pt, fill: rgb("#475467"))[
        High-Resolution Vector PDF Album Generation for World Stamp Collections \
        Covering Both Legacy AlbumEasy (4.2.1 / 4.5.0) and Native Vector (`.stampalbum`) Standards
      ]
      #v(20pt)
      #text(size: 9pt, fill: rgb("#64748B"))[
        Software Version 1.14+ | Document Edition: November 2026 \
        Laser Spider LLC | https://stampalbumgenerator2.com
      ]
      #v(10pt)
    ]
  )
]

#pagebreak()

// --- Table of Contents ---
#text(size: 18pt, weight: "bold", fill: rgb("#1B365D"))[Table of Contents]
#v(10pt)
#outline(
  title: none,
  indent: 1.5em,
  depth: 2
)

// --- Modular Section Imports ---
#include "sections/01_introduction.typ"
#include "sections/02_setup_and_config.typ"
#include "sections/03_scrapers_and_sources.typ"
#include "sections/04_review_ui_guide.typ"
#include "sections/05_command_reference.typ"
#include "sections/06_command_catalog.typ"
#include "sections/07_troubleshooting_and_appendix.typ"
