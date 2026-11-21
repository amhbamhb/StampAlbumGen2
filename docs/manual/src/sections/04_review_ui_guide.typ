// ==============================================================================
// Section 4: Review UI Operator's Manual & Screen-by-Screen Guide
// ==============================================================================

#pagebreak()
= 4. Review UI Operator's Manual

The Review UI is a local, interactive web application accessible by default at:
#align(center)[#text(size: 13pt, weight: "bold", fill: rgb("#1B365D"))[`http://127.0.0.1:7878/`]]

It gives collectors complete visual control over catalog reconciliation, candidate linking, dimension overrides, and live PDF preview generation.

#let ui_placeholder(title, caption_text, height_val: 180pt) = {
  v(8pt)
  rect(
    width: 100%,
    height: height_val,
    stroke: (paint: rgb("#2B5B84"), dash: "dashed", thickness: 1pt),
    fill: rgb("#F0F4F8"),
    radius: 4pt,
    [
      #align(center + horizon)[
        #text(size: 14pt, fill: rgb("#2B5B84"), weight: "bold")[#title] \
        #v(4pt)
        #text(size: 9pt, fill: rgb("#667085"), style: "italic")[Reserved Image Space — Insert Screenshot Here] \
        #text(size: 8pt, fill: rgb("#98A2B3"))[Recommended Resolution: 1200 x 600 px (PNG format)]
      ]
    ]
  )
  v(2pt)
  align(center)[#text(size: 9pt, weight: "bold", fill: rgb("#344054"))[#caption_text]]
  v(10pt)
}

== 4.1 Country & Album Selection Dashboard

Upon opening the Review UI, the collector is presented with the country selection dashboard. This view lists all countries currently present in the SQLite database, indicating the total number of matched stamps, unlinked candidates, and the latest generation timestamps.

#ui_placeholder(
  "Figure 4.1: Country Selection & Status Dashboard",
  "Figure 4.1: The primary dashboard displaying collection statistics, year ranges, and country selection cards."
)

- *Country Selector:* Dropdown or search filter to immediately jump to a specific philatelic entity.
- *Batch Actions:* Buttons to initiate automated web scraping or trigger generation for all completed albums.
- *System Health Badges:* Live indicators showing database connection status and local AlbumEasy/Native renderer availability.

== 4.2 Two-Panel Stamp Reconciliation Interface

Clicking a country opens the dual-pane reconciliation workspace. The left pane represents the authoritative StampWorld database record, while the right pane displays potential Colnect candidate matches.

#ui_placeholder(
  "Figure 4.2: Dual-Pane Candidate Reconciliation Workspace",
  "Figure 4.2: Side-by-side comparison of StampWorld catalog entry (left) and potential Colnect matches (right)."
)

- *Left Panel (Album Record):* Displays current catalog number, year, issue date, denomination, color, and physical dimensions.
- *Right Panel (Candidate Pool):* Displays candidates retrieved from Colnect CSV exports or manual import tables.
- *Match Quality Score:* Automated heuristic scoring based on date, face value, and perforation matches.

== 4.3 High-Resolution Specimen Image Inspector

To prevent linking the wrong color shade, watermark, or perforation variety, the inspector allows collectors to zoom into specimen artwork side-by-side.

#ui_placeholder(
  "Figure 4.3: High-Resolution Specimen Artwork Inspector",
  "Figure 4.3: Side-by-side zoom view verifying design details, overprints, and perforation teeth."
)

- *Zoom & Pan Controls:* Examine intricate stamp engraving, secret marks, and overprints.
- *Aspect Ratio Check:* Validates that the uploaded image matches the recorded physical width-to-height ratio.

== 4.4 Manual Metadata & Dimension Override Dialog

When catalog sources disagree on stamp dimensions (e.g., measuring design size vs. outer perforated paper margins), collectors can manually override values with sub-millimetre precision.

#ui_placeholder(
  "Figure 4.4: Stamp Dimension & Metadata Override Modal",
  "Figure 4.4: Precision entry dialog for stamp width, height, mount clearances, and custom catalog annotations."
)

- *Exact Width & Height:* Millimetre inputs with up to two decimal places of precision.
- *Mount Clearance Override:* Set custom individual margin additions (standard mount clearance is +3 mm).
- *Catalog Cross-Reference:* Add or modify Scott, Michel, Stanley Gibbons, and Yvert & Tellier identifiers.

== 4.5 "Add Colnect As Stamp" Workflow

When a Colnect candidate has no corresponding record in the StampWorld scrape (such as a miniature sheet, souvenir sheet, or unlisted color variant), collectors can instantly promote the candidate into a first-class stamp record.

#ui_placeholder(
  "Figure 4.5: Add Colnect As New Stamp Workflow",
  "Figure 4.5: Promoting an unlisted Colnect candidate directly into the album database."
)

- *SG & Series Inputs:* Assign series title, catalog numbering, and chronological position.
- *Image Selection:* Choose the candidate's thumbnail, upload a custom high-resolution scan, or generate a framed mount placeholder without an image.

== 4.6 Dynamic PDF Generation Controls & Filters

The bottom control panel on the Review screen allows live rendering of album pages without returning to the CLI.

#ui_placeholder(
  "Figure 4.6: Dynamic PDF Generation Panel & Custom Filters",
  "Figure 4.6: Emission filters, format selectors, and the 'Generate PDF Now' action controls."
)

- *Emission Filter:* Filter by Commemorative, Definitive, Air Mail, Semi-Postal, or Postage Due issues.
- *Format Filter:* Filter by Single Stamp, Pair, Block, Souvenir Sheet, or Booklet Pane.
- *Generate PDF Now:* Compiles the active selection immediately and opens the resulting vector PDF in a separate browser tab.

== 4.7 Soft Candidate Deletion & Session Undo

If a candidate is irrelevant or duplicate, collectors can soft-delete it from the candidate pool:
- *Delete Colnect Candidate:* Retains the row in the database but removes it from active display.
- *Undo Delete:* Restores the most recently removed candidate during the current browser session.

== 4.8 Integrated Live Configuration Editor

The Review UI includes an in-browser configuration editor allowing collectors to modify `config.toml` parameters without touching a text editor.

#ui_placeholder(
  "Figure 4.7: In-Browser Configuration Editor",
  "Figure 4.7: Live visual configuration editor with contextual documentation for every parameter."
)
