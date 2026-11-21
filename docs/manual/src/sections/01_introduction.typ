// ==============================================================================
// Section 1: Welcome & Quick Start Guide
// ==============================================================================

#pagebreak()
= 1. Welcome & Quick Start Guide

Welcome to *Stamp Album Generator 2*! Whether you are organizing a lifelong worldwide collection, building a specialized exhibit of classic issues, or putting together a neat one-country binder, this program is designed to make creating beautiful, professional stamp album pages easy and enjoyable.

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
    ]
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
    Simply double-click the `stamp_album_gen2.exe` icon (or run `./stamp_album_gen2` on Linux). The program will start up in a few seconds and automatically prepare your workspace.
  ],
  circle(radius: 12pt, fill: rgb("#1B365D"))[#align(center + horizon)[#text(fill: white, weight: "bold")[2]]],
  [
    *Open Your Web Browser* \
    Open your favorite web browser (such as Chrome, Edge, or Firefox) and visit: \
    #text(size: 11pt, weight: "bold", fill: rgb("#1B365D"))[#link("http://127.0.0.1:7878")[`http://127.0.0.1:7878`]] \
    You will see the visual collection dashboard ready to go.
  ],
  circle(radius: 12pt, fill: rgb("#1B365D"))[#align(center + horizon)[#text(fill: white, weight: "bold")[3]]],
  [
    *Review Your Stamps and Click "Generate PDF Now"* \
    Select your country, check that the stamp mount boxes and pictures look great, and click the green *Generate PDF Now* button. Your custom album opens immediately, ready to view or print!
  ]
)

== 1.2 Your Working Folders

When you run the program, it creates a few simple, well-organized folders next to the application:

- *`Albums/`* — This is where your finished PDF album pages and page source files are saved, organized neatly by country.
- *`UserImport/`* — A place where you can drop simple spreadsheets of custom stamps or family heirlooms not found in online catalogs.
- *`flags/`* — High-resolution national flags and historical emblems used for your album title cover pages.
- *`Logs/`* — Diagnostic records that keep track of what the application did, useful if you ever need technical help.
