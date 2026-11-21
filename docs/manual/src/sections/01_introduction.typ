// ==============================================================================
// Section 1: Introduction & System Architecture
// ==============================================================================

#pagebreak()
= 1. Introduction & System Architecture

== 1.1 Project Purpose

*StampAlbumGen2* is a high-precision, automated stamp album authoring system and rendering engine designed for philatelists. Collecting stamps requires rigorous dimensional precision: stamp mounts must accommodate actual paper dimensions, perforation teeth, and clear border margins without crowding or visual distortion.

The application bridges three major data workflows:
+ *Web Scrapers & Catalog Data:* Automated retrieval of stamp metadata, dimensions, color varieties, and specimen images from StampWorld, Colnect, and US1847.
+ *Interactive Review UI:* A local browser-based reconciliation application that links online catalog entries, resolves ambiguous varieties, verifies dimensions, and manages custom user stamp imports.
+ *Dual PDF Rendering Engine:* Production of high-resolution vector stamp album pages compatible with both legacy *AlbumEasy* scripts and the next-generation *Native Vector Engine* (`.stampalbum`).

== 1.2 System Architecture Overview

The system is constructed as a modular Rust workspace consisting of two primary crates:
- `stamp_album_db`: A dedicated database crate encapsulating SQLite schema management, multi-catalog relational queries, stamp metadata normalization, and auto-migration logic.
- `stamp_album_gen2`: The core CLI binary, Review UI HTTP daemon, scrapers, AlbumEasy parser, and native vector PDF compositor.

```
+-------------------------------------------------------------------------+
|                              Data Sources                               |
|   +--------------------+  +--------------------+  +-----------------+   |
|   | StampWorld Scraper |  | Colnect CSV Ingest |  | US1847 Scraper  |   |
|   +---------+----------+  +---------+----------+  +--------+--------+   |
|             |                       |                      |            |
|             +-----------------------+----------------------+            |
|                                     |                                   |
|                                     v                                   |
|                         [ SQLite Stamp Database ]                       |
+-------------------------------------+-----------------------------------+
                                      |
                                      v
+-------------------------------------+-----------------------------------+
|                     Interactive Review Web UI (7878)                    |
|   - Visual Stamp-to-Candidate Matching                                  |
|   - Real-time Configuration Editor & Dimension Overrides                |
|   - In-Browser PDF Preview Generation                                   |
+-------------------------------------+-----------------------------------+
                                      |
                                      v
+-------------------------------------+-----------------------------------+
|                     Dual Page Generation Engine                         |
|   +---------------------------------+ +-----------------------------+   |
|   | Legacy AlbumEasy Engine (.txt)  | | Native Vector Engine        |   |
|   | - AlbumEasy 4.5.0 Compatible    | |   (.stampalbum)             |   |
|   | - Classic Script File Output    | | - Direct High-Res PDF Output|   |
|   +----------------+----------------+ +--------------+--------------+   |
|                    |                                 |                  |
|                    +----------------+----------------+                  |
|                                     |                                   |
|                                     v                                   |
|                      [ Vector PDF Stamp Album Pages ]                   |
+-------------------------------------------------------------------------+
```

== 1.3 Dual-Engine Rendering Philosophy

StampAlbumGen2 supports two distinct document representation models:
- *AlbumEasy (.txt):* Plain text script files conforming to the syntax of Bundu Technology's AlbumEasy (versions 4.2.1 through 4.5.0).
- *Native Album (.stampalbum):* A modern, extensible vector document syntax starting with `STAMPALBUM 1`. It allows sub-millimetre mount positioning, alternating double-sided margins, multi-layered decorative frames, and embedded vector graphics directly into PDF output without relying on external third-party software.
