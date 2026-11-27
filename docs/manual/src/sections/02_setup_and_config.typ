// ==============================================================================
// Section 2: The Visual Review Workspace (Your Primary Tool)
// ==============================================================================

#pagebreak()
= 2. The Visual Review Workspace (Your Primary Tool)

The *Review Screen* is the heart of Stamp Album Generator 2. This is where you will spend 95% of your time: browsing your stamps, comparing color varieties, checking that mount dimensions give your stamps proper breathing room, and printing your album pages.

#let ui_box(title, caption_text, image_path: none, height_val: 160pt) = {
  v(8pt)
  if image_path != none {
    align(center)[
      #block(
        stroke: 0.5pt + rgb("#CBD5E1"),
        radius: 4pt,
        clip: true,
        fill: rgb("#FFFFFF"),
        inset: 2pt,
        [
          #image(image_path, width: 95%)
        ]
      )
    ]
  } else {
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
  }
  v(2pt)
  align(center)[#text(size: 9pt, weight: "bold", fill: rgb("#344054"))[#caption_text]]
  v(8pt)
}

== 2.1 The Collection Dashboard: Choosing Your Country & Control Center

When you first open `http://127.0.0.1:7878` in your web browser, you arrive at the main Collection Dashboard. This central control room gives you instant access to all your stamp albums, configuration tools, and collection management options.

#ui_box(
  "Figure 2.1: The Start Review Dashboard & Main Controls",
  "Figure 2.1: The main startup workspace showing the Country dropdown, action toolbar, update notification, and collection tools.",
  image_path: "../images/fig2_1_dashboard.png"
)

*Everything on the Main Dashboard Explained:*

- *About this app & setup hints:* A helpful collapsible banner at the top providing quick orientation notes and crediting AlbumEasy by Bundu Technology Ltd. Clicking anywhere on the banner expands or collapses it. The program automatically remembers whether you left it open or closed from one session to the next (as it does for all expandable panels across the application).
- *Country Dropdown:* Your primary collection picker. Click this menu to select whichever country or album you wish to work on (such as *Norway*). All actions and buttons on the screen immediately apply to your selected country.
- *Open Review:* Opens the interactive, side-by-side stamp matching and mount sizing workspace for the chosen country.
- *Refresh:* Re-scans your local database and album directories, instantly refreshing counts and candidate lists without needing to restart the application.
- *Site Accounts:* Quick access to online philatelic service credentials. _(Note: This legacy management button will be phased out and removed in a future software release)._
- *Edit Config:* Opens the global baseline configuration editor. Changes made here establish the default baseline settings (such as page margins, font selections, and mount clearances) used whenever a new country or album is created. Setting your personal preferences here ensures every new collection follows your standards without having to reconfigure each country individually.
- *Generate PDF Now:* Builds a complete, publication-grade vector PDF album containing all stamp items currently in that country's database with a single click.
- *Import Colnect - User Created Files:* Opens an import dialog allowing you to copy personal or custom Colnect CSV spreadsheets directly into the active country's Colnect folder (e.g. your *Norway* folder).
- *Help Manual:* Opens this comprehensive, illustrated guide directly in your computer's default PDF viewer.
- *Update Notification Badge:* StampAlbumGen2 automatically checks for new releases on startup, and once every 6 hours if kept running continuously. When a newer version is available, a friendly badge appears (e.g. *✨ Update to v1.15.11 Available*). Clicking it displays release highlights and enables a one-click automatic download, verification, and restart.
- *Logout:* Cleanly stops the local web server and safely closes the application.
- *Countries Current Count:* Displays the total number of distinct countries and thematic collections currently stored and managed in your Albums folder.
- *+ Create New Country or Theme:* Opens a guided setup dialog to start an entirely new country or specialized stamp theme, automatically creating its dedicated folder structure and SQLite database.

#ui_box(
  "Figure 2.2: Albums Root Directory & Country Status Card",
  "Figure 2.2: The Albums Root directory banner and the active country's database status card.",
  image_path: "../images/fig2_2_country_status.png"
)

*Understanding the Albums Root & Country Status:*

- *Albums Root:* Shows the master folder path on your computer where all country folders, stamp databases, scraped artwork, and generated album PDFs are saved (such as `Documents/StampAlbumGen2/Albums` on Linux/macOS or `Documents\StampAlbumGen2\Albums` on Windows). You can customize this folder if needed, though keeping all your albums inside this central directory ensures your entire collection stays neatly organized and easy to back up. Clicking the header expands or collapses the path view, and your preference is remembered.
- *Active Country Card (e.g. Norway):* Summarizes the health and readiness of your selected collection:
  - *Database File (`DB`):* Shows the path to the local SQLite database storing your stamp catalog records, mount sizes, and page layouts (e.g. `Albums/Norway/Norway.db`).
  - *Colnect Files:* Indicates how many external Colnect catalog CSV files are available for cross-referencing and automatic measurement lookups.
  - *DB Candidates:* Reports the number of candidate stamps ready for visual review and matching.

== 2.2 The Two-Panel Review Screen: Matching Stamps Easily

Once you click *Open Review* on your chosen country, you enter the convenient side-by-side workspace:

#ui_box(
  "Figure 2.3: Side-by-Side Stamp Review & Candidate Matching",
  "Figure 2.3: The left panel shows your album stamps in order; the right panel shows online candidate matches."
)

- *The Left Panel (Your Album Page):* Shows each stamp in chronological order, with its face value, color, issue date, and current mount box size (e.g. $26 times 29 "mm"$).
- *The Right Panel (Available Catalog Matches):* Shows stamps found in online catalogs that might match your stamp. If a match is found, you will see its photo, physical measurements, and catalog numbers.
- *Confirming a Match:* Simply click the match you want! The program instantly updates the stamp's picture and mount box to match.

== 2.3 Checking Details with the High-Resolution Artwork Zoom

Have two stamps that look almost identical, but one is carmine and the other is scarlet? Or an issue with an overprint or secret mark?

#ui_box(
  "Figure 2.4: Zooming In on Stamp Artwork",
  "Figure 2.4: Inspecting high-resolution artwork side-by-side to verify color shades and overprints."
)

Clicking on any stamp image opens a crystal-clear zoom inspector. You can examine fine engraving details, check perforation counts, and ensure you have linked the exact variety in your physical collection.

== 2.4 Customizing Stamp Box Sizes & Mount Clearances

Every stamp collector knows that stamps need comfortable mounts. If you use *Showgard*, *Hawid*, or *Scott* mounts, you usually want a box that is about 3 mm larger than the actual stamp paper so it slides in smoothly.

#ui_box(
  "Figure 2.5: Setting Stamp Dimensions & Mount Clearances",
  "Figure 2.5: Editing stamp width, height, and mount border clearances in millimetres."
)

- You can freely type exact measurements in millimetres (like `26.5` wide by `31.0` high).
- You can customize the caption printed underneath the stamp (such as `\"Scott #1040\\nMint Never Hinged\\nPerf 11\"`).

== 2.5 Adding a Missing Stamp ("Add Colnect As Stamp")

What if you have a souvenir sheet, miniature sheet, or local variety that wasn't included in the default country listing?

#ui_box(
  "Figure 2.6: The 'Add Colnect As Stamp' Dialog",
  "Figure 2.6: Adding a souvenir sheet or unlisted variety directly into your album with one click."
)

If you find the item in the candidate list on the right, simply click the *Add Colnect As Stamp* button! It immediately creates a new frame on your album page, complete with dimensions, series title, and photo.

== 2.6 Generating Your PDF Album Pages ("Generate PDF Now")

At the bottom of the Review screen, you have full control over generating your album:

#ui_box(
  "Figure 2.7: Generating Your Custom PDF Album Pages",
  "Figure 2.7: Filter options (Commemoratives, Definitives) and the one-click 'Generate PDF Now' button."
)

- *Filter by Issue Type:* Want to make an album page containing *only Commemoratives* or *only Air Mails*? Just pick from the *Emission* dropdown.
- *Filter by Format:* Want a separate page for *Souvenir Sheets* or *Blocks of Four*? Use the *Format* filter.
- *Click \"Generate PDF Now\":* Your vector PDF is built in seconds and opens automatically on your screen, ready to admire or print!

== 2.7 Changing Application Settings Right in the Browser

You never need to edit configuration text files by hand unless you want to. The Review UI includes a friendly settings page where you can change the album title, adjust default mount spacing, or change page colors with simple sliders and switches.

#ui_box(
  "Figure 2.8: In-Browser Settings Editor",
  "Figure 2.8: Simple, friendly settings editor with plain-English descriptions for every option."
)

== 2.8 Automatic Updates & One-Click Upgrades

Keeping StampAlbumGen2 up to date is completely effortless:

#ui_box(
  "Figure 2.9: One-Click Update Notification & Upgrade Modal",
  "Figure 2.9: When a new version is released, a friendly badge appears in the header. Click 'Update & Restart Now' to upgrade automatically."
)

- *Automatic Background Check:* When StampAlbumGen2 starts up, it quietly checks whether a newer version has been released on GitHub. It caches results for 6 hours so it never slows down your work.
- *Header Notification Badge:* If an update is available, a pulsating badge appears in the top navigation bar: *\"✨ Update Available\"*.
- *One-Click Upgrade:* Clicking the badge opens a preview showing what's new. Simply click *\"Update & Restart Now\"*! The application downloads the new version, verifies its SHA256 checksum to guarantee safety, replaces the executable, and restarts. Your browser reconnects automatically in just a few seconds.
