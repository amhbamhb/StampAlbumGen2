// ==============================================================================
// Section 1: Welcome & Quick Start Guide
// ==============================================================================

#pagebreak()
= 1. Welcome & Quick Start Guide

Welcome to *Stamp Album Generator 2*! Whether you are organizing a lifelong worldwide collection, building a specialized exhibit of classic issues, or putting together a neat one-country binder, this program is designed to make creating beautiful, professional stamp album pages easy and enjoyable.

#v(4pt)
#block(
  fill: rgb("#EFF8FF"),
  stroke: 0.5pt + rgb("#B2DDFF"),
  inset: 10pt,
  radius: 4pt,
  width: 100%,
  [
    #text(
      weight: "bold",
      size: 10.5pt,
      fill: rgb("#175CD3"),
    )[📘 A Living Manual — Continually Updated as Features Expand] \
    #v(3pt)
    #text(size: 9.5pt, fill: rgb("#1F2937"))[
      This help manual is actively maintained as a living guide. As our team enhances and verifies the *Native PDF* vector engine—introducing precision mount tolerances, decorative page framing, and visual collection workflows—we update this document step-by-step with real-world screenshots, collector tips, and plain-English guides. Whenever you update StampAlbumGen2, your embedded help manual stays completely up to date automatically!
    ]
  ],
)
#v(4pt)

#align(center)[
  #block(
    fill: rgb("#F0F4F8"),
    stroke: 1pt + rgb("#2B5B84"),
    inset: 12pt,
    radius: 4pt,
    width: 95%,
    [
      #text(weight: "bold", size: 12pt, fill: rgb("#1B365D"))[What Makes Stamp Album Generator Special?] \
      #v(4pt)
      Instead of spending hours manually measuring boxes or typing complicated layout codes into a word processor, Stamp Album Generator automatically looks up stamp dimensions, issue dates, catalog numbers, and full-color specimen photos. It lays them out neatly into ready-to-print vector PDF album pages tailored specifically to the stamps you own.
    ],
  )
]

== 1.1 The Quick Start: 3 Steps to Your First Album Page

You do not need to learn any programming or technical commands to get started. In fact, 95% of collectors do all their work directly inside the friendly visual *Review Screen* in their web browser!

#v(6pt)

#grid(
  columns: (30pt, 1fr),
  gutter: 10pt,
  circle(radius: 12pt, fill: rgb("#1B365D"))[#align(center + horizon)[#text(fill: white, weight: "bold")[1]]],
  [
    *Start the Program* \
    Simply double-click the `stamp_album_gen2.exe` icon (or run `./stamp_album_gen2` on Linux). The program will start up in a few seconds and automatically prepare your workspace. \
    #v(4pt)
    #block(
      fill: rgb("#F8FAFC"),
      stroke: 0.5pt + rgb("#CBD5E1"),
      inset: (x: 8pt, y: 6pt),
      radius: 4pt,
      [
        #grid(
          columns: (1fr, auto, auto),
          gutter: 10pt,
          [
            #text(weight: "bold", fill: rgb("#1B365D"))[🖥️ Desktop Icon Created Automatically:] \
            When first launched, Stamp Album Generator 2 automatically creates a convenient desktop launcher icon on your desktop (and in your Start Menu on Windows or Applications menu on Linux). You can launch your collection workspace anytime with a single click!
          ],
          [
            #align(center + horizon)[
              #block(stroke: 0.5pt + rgb("#CBD5E1"), radius: 3pt, inset: 2pt, fill: white)[
                #image("../images/fig1_desktop_icon_windows.png", height: 38pt)
              ]
              #v(1pt)
              #text(size: 6.5pt, fill: rgb("#64748B"))[Windows]
            ]
          ],
          [
            #align(center + horizon)[
              #block(stroke: 0.5pt + rgb("#CBD5E1"), radius: 3pt, inset: 2pt, fill: white)[
                #image("../images/fig1_desktop_icon_linux.png", height: 38pt)
              ]
              #v(1pt)
              #text(size: 6.5pt, fill: rgb("#64748B"))[Linux]
            ]
          ],
        )
      ],
    )
  ],

  circle(radius: 12pt, fill: rgb("#1B365D"))[#align(center + horizon)[#text(fill: white, weight: "bold")[2]]],
  [
    *Automatic Web Browser Launch* \
    When the program starts up, it automatically launches the visual Collection Dashboard in your default web browser! If your browser is already running, or if you ever need to reconnect later after starting the program, simply visit: \
    #text(size: 11pt, weight: "bold", fill: rgb("#1B365D"))[#link("http://127.0.0.1:7878")[`http://127.0.0.1:7878`]]
  ],

  circle(radius: 12pt, fill: rgb("#1B365D"))[#align(center + horizon)[#text(fill: white, weight: "bold")[3]]],
  [
    *Choose or Create a Country, Review Stamps & Generate Your PDF* \
    Pick an existing country from the dropdown, or create a brand new country or theme (using *+ Create New Country or Theme*). Click *Open Review* to match stamps, check mount clearances, and customize captions. When you are ready, click the green *Generate PDF Now* button—your custom, publication-grade album opens immediately!
  ],
)

== 1.2 Your Working Folders

When you run the program, it creates a few simple, well-organized folders next to the application:

- *`Albums/`* — This is where your finished PDF album pages and page source files are saved, organized neatly by country.
- *`UserImport/`* — A place where you can drop simple spreadsheets of custom stamps or family heirlooms not found in online catalogs.
- *`flags/`* — High-resolution national flags and historical emblems used for your album title cover pages.
- *`Logs/`* — Diagnostic records that keep track of what the application did, useful if you ever need technical help.
