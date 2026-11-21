// ==============================================================================
// Section 7: Advanced Options, Command Line & Troubleshooting
// ==============================================================================

#pagebreak()
= 7. Advanced Options, Command Line & Troubleshooting

While 95% of collectors will do everything conveniently inside the visual Review Screen in their web browser, StampAlbumGen2 also includes powerful command-line tools and fine-grained configuration files for power users, automated overnight batch jobs, and scripting.

== 7.1 Command Line Usage (Power Users)

You can run `stamp_album_gen2` from a terminal (PowerShell, Command Prompt, or Linux bash) with optional flags to generate whole albums in headless batch mode without opening the browser.

#table(
  columns: (2fr, 3fr),
  fill: (x, y) => if y == 0 { rgb("#1B365D").lighten(85%) } else if calc.even(y) { rgb("#F8F9FA") } else { none },
  stroke: 0.5pt + rgb("#D0D5DD"),
  table.header([*Command Line Example*], [*What It Does*]),
  [
    `stamp_album_gen2`
  ], [
    Standard mode: Starts the local server and launches the interactive visual Review UI.
  ],
  [
    `stamp_album_gen2 --site-name Switzerland --start-year 1970 --end-year 1975`
  ], [
    Batch generation: Automatically compiles Switzerland issues from 1970 to 1975 straight to PDF in `Albums/`.
  ],
  [
    `stamp_album_gen2 --source colnect --site-name "United States"`
  ], [
    Scrape and ingest data using Colnect as the primary catalog source.
  ],
  [
    `stamp_album_gen2 --pdf-dx-logging`
  ], [
    Enables deep diagnostic logging for layout bounds, margins, and font measurements.
  ],
  [
    `stamp_album_gen2 --review-ui-bind 0.0.0.0:8080`
  ], [
    Custom network port: Useful if port 7878 is used by another application.
  ],
)

== 7.2 Customizing `config.toml` Directly

If you prefer editing text configuration files over using the in-browser settings screen, you can open `config.toml` in Notepad or your favorite text editor:

- `albums_root = "Albums"` — Change the output destination folder where your finished albums are saved.
- `stampworld_fetch_delay_ms = 500` — Delay between web scraper requests (default is 500 milliseconds).
- `albumeasy_pdf_timeout_secs = 300` — Maximum time to wait when generating very large multi-hundred page albums.
- `print_flag_cover = true` — Turn on or off the national flag title cover page.

== 7.3 Troubleshooting & Getting Help

If an album page does not look as expected, here are quick solutions to common questions:

#block(
  fill: rgb("#F8F9FA"),
  inset: 12pt,
  radius: 4pt,
  stroke: 0.5pt + rgb("#D0D5DD"),
  [
    *Q: The stamp mount box is slightly smaller than my physical mount.* \
    *A:* In your PDF viewer's Print dialog, make sure page scaling is set to *100% / Actual Size*. Never select *\"Fit to Page\"*.

    #v(4pt)
    *Q: A long stamp heading is overlapping or spilling past the page border.* \
    *A:* In the Review UI, open the stamp's edit dialog and shorten the heading, reduce the font size by 1 or 2 points, or attach the heading to a wider stamp in the row.

    #v(4pt)
    *Q: Where are diagnostic logs saved?* \
    *A:* Look inside the `Logs/` directory. Each run creates a clear, timestamped log file (`Logs/stamp_album_gen2-<timestamp>.log`) recording every operation and error message.
  ]
)

== 7.4 License & Acknowledgments

*Laser Spider Stamp Collector License (LSSCL) v1.0* \
Individual stamp collectors may use StampAlbumGen2 for personal, non-commercial album generation completely free of charge. Non-collector, commercial, institutional, or business use requires a separate written agreement with Laser Spider LLC.

*Acknowledgments:* \
StampAlbumGen2 honors and builds upon the pioneering work of *AlbumEasy* by Bundu Technology Ltd. (#link("https://www.thestampweb.com/albumeasy")[thestampweb.com/albumeasy]).

Official project website and online command gallery: #link("https://stampalbumgenerator2.com")[stampalbumgenerator2.com].
