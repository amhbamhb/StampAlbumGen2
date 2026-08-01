# Unified Configuration Format for Commemorative & Definitive Issues

## Overview

This format extends the existing album configuration to support both:

- **COMMEMORATIVE ISSUES**: Single-year, event-focused stamps (e.g., 1955 year issues)
- **DEFINITIVE/ORDINARY ISSUES**: Multi-year series with printing variations (e.g., 1954 Liberty Series)

---

## Configuration Header Extensions

### Issue Type Declaration

```
ISSUE_TYPE (commemorative)          # or "definitive"
ISSUE_NAME ("US Stamps - 1955")
ISSUE_SERIES ("Presidential Year")  # Optional for commemorative
SERIES_START_YEAR (1954)             # For definitive issues spanning years
SERIES_END_YEAR (1968)               # Leave blank for ongoing series
```

### Metadata Tags (Issue-Specific)

```
# For COMMEMORATIVE:
ISSUE_CLASSIFICATION (annual)       # annual, theme, event
THEME_CATEGORY (government)         # government, transportation, culture, etc.
GOVERNMENT_INFO (Eisenhower, Summerfield)

# For DEFINITIVE:
ISSUE_CLASSIFICATION (ordinary)     # ordinary, definitive, permanent
BASE_DESIGN ("Liberty Series")
PORTRAIT_SUBJECT ("George Washington")
```

---

## Content Sections by Type

### COMMEMORATIVE ISSUE Structure

```
################################################################################
# COMMEMORATIVE SECTION - 1955
################################################################################

ISSUE_TYPE (commemorative)
ISSUE_NAME ("United States - 1955")
ISSUE_CLASSIFICATION (annual)
THEME_CATEGORY (government)
GOVERNMENT_INFO_SECTION_BEGIN
    PRESIDENT ("Dwight David Eisenhower")
    POSTMASTER_GENERAL ("Arthur E. Summerfield")
    DOMESTIC_RATE ("3¢ per oz.")
    POSTCARD_RATE ("2¢")
    AIRMAIL_RATE ("2¢")
GOVERNMENT_INFO_SECTION_END

DESIGN_CREDITS_SECTION_BEGIN
    ENGRAVER ("C. A. Brooks")
    DESIGNER ("McCloskey, Jr")
    TAGGING_INFO ("None on 1955 stamps")
DESIGN_CREDITS_SECTION_END

COMMEMORATIVE_STAMP_BEGIN (Scott 1064)
    TITLE ("Pennsylvania Academy of the Arts Sesquicentennial")
    DATE_ISSUED (Jan. 15, 1955)
    PLACE_ISSUED ("Philadelphia, PA")
    DENOMINATION ("3¢")
    PERFORATION ("10½ x 11")
    PRINTING_METHOD (Rotary Press)
    PRINTING_DETAILS ("200 Subject Electric Eye Plates")
    COLOR ("Carmine")
    QUANTITY_ISSUED (116139800)
    DESCRIPTION ("Pennsylvania Academy of the Arts Sesquicentennial Stamp")
COMMEMORATIVE_STAMP_END

COMMEMORATIVE_STAMP_BEGIN (Scott 1065)
    TITLE ("Land Grant Colleges Centennial")
    DATE_ISSUED (Feb. 12, 1955)
    PLACE_ISSUED ("East Lansing, MI")
    DENOMINATION ("3¢")
    PERFORATION ("10½ x 11")
    PRINTING_METHOD (Rotary Press)
    COLOR ("Deep Blue")
    QUANTITY_ISSUED (120484800)
COMMEMORATIVE_STAMP_END
# ... additional commemorative stamps
```

### DEFINITIVE/ORDINARY ISSUE Structure

```
################################################################################
# DEFINITIVE SECTION - 1954 Liberty Series
################################################################################

ISSUE_TYPE (definitive)
ISSUE_NAME ("Designs of the 1954 Series - The Liberty Issue")
ISSUE_CLASSIFICATION (ordinary)
BASE_DESIGN ("Liberty Series")
PORTRAIT_SUBJECT ("George Washington")
SERIES_START_YEAR (1954)
SERIES_END_YEAR (1968)

DEFINITIVE_DESIGN_BEGIN (Washington Portrait - Scott 1041)
    DESIGN_SIZE ("22.7mm x 22.8mm")
    DESIGN_ASPECT (portrait)
    COLOR_PRIMARY ("Carmine")
    FIRST_ISSUE_DATE (April 9, 1954)
DEFINITIVE_DESIGN_END

DEFINITIVE_VARIANT_BEGIN (Scott 1041 - Flat Plate)
    VARIANT_LABEL ("Flat Plate Dry Printing")
    FDC_DATE (April 9, 1954)
    PRINTING_METHOD (flat plate dry printing)
    PRINTING_TYPE (sheet stamp)
    PERFORATION ("11 x 11")
    TAGGING (none)
    SIZE (22.7mm high)
    COLLECTIBLE_VARIETIES (plate block 4, 2 plate numbers)
    NOTES ("Size ~22.7mm high")
DEFINITIVE_VARIANT_END

DEFINITIVE_VARIANT_BEGIN (Scott 1041B - Rotary Press)
    VARIANT_LABEL ("Rotary Press Dry Printing")
    FDC_DATE (April 9, 1954)
    PRINTING_METHOD (rotary press dry printing)
    PRINTING_TYPE (sheet stamp)
    PERFORATION ("11 x 11")
    TAGGING (none)
    SIZE (22.9mm high)
    COLLECTIBLE_VARIETIES (plate block 4, 2 plate numbers - one red, one blue)
    NOTES ("Size slightly taller ~22.9mm")
DEFINITIVE_VARIANT_END

DEFINITIVE_VARIANT_BEGIN (Scott 1042 - Giori Press)
    VARIANT_LABEL ("Giori Press")
    FDC_DATE (March 22, 1958)
    PRINTING_METHOD (Giori press)
    PRINTING_TYPE (sheet stamp)
    PERFORATION ("11 x 11")
    TAGGING (none)
    SIZE (22.8mm high)
    COLLECTIBLE_VARIETIES (plate number block 4)
    NOTES ("Only one plate number - both colors printed in one pass")
DEFINITIVE_VARIANT_END

# Additional variants follow...

DEFINITIVE_REFERENCE_CATALOG_BEGIN
    TITLE ("1954 Liberty Series Catalog")
    TABLE_COLUMNS (Cat #, FDC, Printing Method, Perforation, Tagging, Varieties, Usage)
    # Data reference structure for variant table output
DEFINITIVE_REFERENCE_CATALOG_END
```

---

## Page Layout Directives

### For COMMEMORATIVE Pages

```
COMMEMORATIVE_PAGE_LAYOUT_BEGIN
    PAGE_SECTION_HEADER (government_info)      # Show president, postmaster, rates
    PAGE_SECTION_DETAIL (historical_context)   # Designer, engraver, tagging info
    PAGE_SECTION_STAMPS (arranged_chronological) # By issue date
    PAGE_SECTION_FOOTER (quantities_issued)     # Total quantities per stamp
COMMEMORATIVE_PAGE_LAYOUT_END
```

### For DEFINITIVE Pages

```
DEFINITIVE_PAGE_LAYOUT_BEGIN
    PAGE_SECTION_HEADER (series_overview)       # Series name, years
    PAGE_SECTION_VARIANTS (table_format)        # Variant comparison table
    PAGE_SECTION_STAMPS (arranged_by_variant)   # Group by printing method
    PAGE_SECTION_FOOTER (philatelic_notes)      # Varieties, collecting tips
DEFINITIVE_PAGE_LAYOUT_END
```

---

## Rendering Configuration

```
# Auto-detect layout based on issue type
RENDER_MODE (adaptive)              # or "manual"

# Conditional styling
STYLE_COMMEMORATIVE_HEADER (darkblue, larger, centered)
STYLE_COMMEMORATIVE_DETAIL (darkblue, italics)

STYLE_DEFINITIVE_HEADER (navy, technical_font)
STYLE_DEFINITIVE_TABLE (standard_table, borders_on)

# Content emphasis
EMPHASIS_COMMEMORATIVE (historical, cultural, quantities)
EMPHASIS_DEFINITIVE (technical, variants, collecting)
```

---

## Color Coding by Issue Type

```
# Visual distinction in output
COLOR_COMMEMORATIVE_BACKGROUND (light_cream)
COLOR_DEFINITIVE_BACKGROUND (light_gray)

COLOR_COMMEMORATIVE_ACCENT (darkblue)
COLOR_DEFINITIVE_ACCENT (navy)

BADGE_COMMEMORATIVE_TYPE ("COMMEMORATIVE")
BADGE_DEFINITIVE_TYPE ("DEFINITIVE SERIES")
```

---

## Data Extraction Markers

### For Web Scraping/Parsing

```
# Commemorative Issue Markers
<!--BEGIN_COMMEMORATIVE_SECTION-->
<!--ISSUE_YEAR: 1955-->
<!--STAMP_COUNT: 9-->
<!--TOTAL_ISSUED: 120,000,000+-->
<!--END_COMMEMORATIVE_SECTION-->

# Definitive Issue Markers
<!--BEGIN_DEFINITIVE_SERIES-->
<!--SERIES_NAME: Liberty Issue-->
<!--YEARS: 1954-1968-->
<!--VARIANT_COUNT: 45+-->
<!--END_DEFINITIVE_SERIES-->
```

---

## Example Usage Mapping

| Field | Commemorative Example | Definitive Example |
|-------|---|---|
| ISSUE_TYPE | commemorative | definitive |
| CLASSIFICATION | annual | ordinary |
| Primary Grouping | By date issued | By design variant |
| Key Metadata | Place, date, quantities | Printing method, perf, tagging |
| Table Type | Chronological list | Technical variants table |
| Layout Focus | Historical narrative | Philatelic comparison |
| Collector Interest | Complete year sets | Printing varieties |
| Image Emphasis | Single image per stamp | Multiple variants per design |
