// ==============================================================================
// Section 2: The Visual Review Workspace (Your Primary Tool)
// ==============================================================================

#pagebreak()
= 2. The Visual Review Workspace (Your Primary Tool)

The *Review Screen* is the heart of Stamp Album Generator 2. This is where you will spend 95% of your time: browsing your stamps, comparing color varieties, checking that mount dimensions give your stamps proper breathing room, and printing your album pages.

#let ui_box(title, caption_text, height_val: 160pt) = {
  v(8pt)
  rect(
    width: 100%,
    height: height_val,
    stroke: (paint: rgb("#2B5B84"), dash: "dashed", thickness: 1pt),
    fill: rgb("#F0F4F8"),
    radius: 4pt,
    [
      #align(center + horizon)[
        #text(size: 13pt, fill: rgb("#2B5B84"), weight: "bold")[#title] \
        #v(4pt)
        #text(size: 9pt, fill: rgb("#667085"), style: "italic")[Screenshot Placeholder — Insert User Interface Image Here] \
        #text(size: 8pt, fill: rgb("#98A2B3"))[Recommended: Capture from your browser at 1200 x 600 px]
      ]
    ]
  )
  v(2pt)
  align(center)[#text(size: 9pt, weight: "bold", fill: rgb("#344054"))[#caption_text]]
  v(8pt)
}

== 2.1 The Collection Dashboard: Choosing Your Country

When you first open `http://127.0.0.1:7878`, you arrive at the main Collection Dashboard.

#ui_box(
  "Figure 2.1: The Country Selection & Collection Dashboard",
  "Figure 2.1: The main screen listing all your stamp countries, total counts, and quick generation buttons."
)

*What you see here:*
- *Country Cards:* Each country in your collection is displayed with a summary of how many stamps are cataloged, how many are matched with images, and the year range covered (for example: *Switzerland 1970–1975*).
- *Quick Search:* A search box at the top lets you instantly find any country by name.
- *Open Collection:* Clicking on any country takes you straight into that country's visual matching room.

== 2.2 The Two-Panel Review Screen: Matching Stamps Easily

Once you click a country, you will see a convenient side-by-side workspace:

#ui_box(
  "Figure 2.2: Side-by-Side Stamp Review & Candidate Matching",
  "Figure 2.2: The left panel shows your album stamps in order; the right panel shows online candidate matches."
)

- *The Left Panel (Your Album Page):* Shows each stamp in chronological order, with its face value, color, issue date, and current mount box size (e.g. $26 times 29 "mm"$).
- *The Right Panel (Available Catalog Matches):* Shows stamps found in online catalogs that might match your stamp. If a match is found, you will see its photo, physical measurements, and catalog numbers.
- *Confirming a Match:* Simply click the match you want! The program instantly updates the stamp's picture and mount box to match.

== 2.3 Checking Details with the High-Resolution Artwork Zoom

Have two stamps that look almost identical, but one is carmine and the other is scarlet? Or an issue with an overprint or secret mark?

#ui_box(
  "Figure 2.3: Zooming In on Stamp Artwork",
  "Figure 2.3: Inspecting high-resolution artwork side-by-side to verify color shades and overprints."
)

Clicking on any stamp image opens a crystal-clear zoom inspector. You can examine fine engraving details, check perforation counts, and ensure you have linked the exact variety in your physical collection.

== 2.4 Customizing Stamp Box Sizes & Mount Clearances

Every stamp collector knows that stamps need comfortable mounts. If you use *Showgard*, *Hawid*, or *Scott* mounts, you usually want a box that is about 3 mm larger than the actual stamp paper so it slides in smoothly.

#ui_box(
  "Figure 2.4: Setting Stamp Dimensions & Mount Clearances",
  "Figure 2.4: Editing stamp width, height, and mount border clearances in millimetres."
)

- You can freely type exact measurements in millimetres (like `26.5` wide by `31.0` high).
- You can customize the caption printed underneath the stamp (such as `"Scott #1040\nMint Never Hinged\nPerf 11"`).

== 2.5 Adding a Missing Stamp ("Add Colnect As Stamp")

What if you have a souvenir sheet, miniature sheet, or local variety that wasn't included in the default country listing?

#ui_box(
  "Figure 2.5: The 'Add Colnect As Stamp' Dialog",
  "Figure 2.5: Adding a souvenir sheet or unlisted variety directly into your album with one click."
)

If you find the item in the candidate list on the right, simply click the *Add Colnect As Stamp* button! It immediately creates a new frame on your album page, complete with dimensions, series title, and photo.

== 2.6 Generating Your PDF Album Pages ("Generate PDF Now")

At the bottom of the Review screen, you have full control over generating your album:

#ui_box(
  "Figure 2.6: Generating Your Custom PDF Album Pages",
  "Figure 2.6: Filter options (Commemoratives, Definitives) and the one-click 'Generate PDF Now' button."
)

- *Filter by Issue Type:* Want to make an album page containing *only Commemoratives* or *only Air Mails*? Just pick from the *Emission* dropdown.
- *Filter by Format:* Want a separate page for *Souvenir Sheets* or *Blocks of Four*? Use the *Format* filter.
- *Click "Generate PDF Now":* Your vector PDF is built in seconds and opens automatically on your screen, ready to admire or print!

== 2.7 Changing Application Settings Right in the Browser

You never need to edit configuration text files by hand unless you want to. The Review UI includes a friendly settings page where you can change the album title, adjust default mount spacing, or change page colors with simple sliders and switches.

#ui_box(
  "Figure 2.7: In-Browser Settings Editor",
  "Figure 2.7: Simple, friendly settings editor with plain-English descriptions for every option."
)
