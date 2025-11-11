# Configuration Testing Matrix

## Overview

This file documents systematic testing of all configuration parameters found in config.rs. Each parameter is tested for:

1. **Valid values** - Acceptable input ranges and types
2. **Invalid values** - Input that should cause errors or warnings
3. **Default behavior** - What happens when parameter is omitted
4. **Parameter interactions** - How parameters affect each other

## Core Stamp Dimensions

### `reference_width_mm` (f32)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid range | 10.0-100.0 | Accepted | ⏳ |
| Minimum edge | 1.0 | Accepted/Warning | ⏳ |
| Zero value | 0.0 | Error/Default | ⏳ |
| Negative | -5.0 | Error/Default | ⏳ |
| String value | "abc" | Parse error | ⏳ |
| Very large | 1000.0 | Warning/Clamp | ⏳ |
| Default | omitted | 25.0 mm used | ⏳ |

### `reference_height_mm` (f32)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid range | 10.0-100.0 | Accepted | ⏳ |
| Minimum edge | 1.0 | Accepted/Warning | ⏳ |
| Zero value | 0.0 | Error/Default | ⏳ |
| Negative | -5.0 | Error/Default | ⏳ |
| String value | "abc" | Parse error | ⏳ |
| Very large | 1000.0 | Warning/Clamp | ⏳ |
| Default | omitted | 30.0 mm used | ⏳ |

### `stamp_box_height` (f32)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid range | 10.0-50.0 | Accepted | ⏳ |
| Minimum edge | 5.0 | Accepted/Warning | ⏳ |
| Zero value | 0.0 | Error/Default | ⏳ |
| Negative | -5.0 | Error/Default | ⏳ |
| String value | "abc" | Parse error | ⏳ |
| Very large | 200.0 | Warning/Clamp | ⏳ |
| Default | omitted | 30.0 mm used | ⏳ |

### `stamp_box_adjust` (f32)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid range | 0.0-10.0 | Accepted | ⏳ |
| Zero value | 0.0 | Accepted (no border) | ⏳ |
| Negative | -1.0 | Error/Default | ⏳ |
| String value | "abc" | Parse error | ⏳ |
| Very large | 50.0 | Warning/Clamp | ⏳ |
| Default | omitted | 3.0 mm used | ⏳ |

### `max_stamps_per_row` (u32)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid range | 1-10 | Accepted | ⏳ |
| Zero value | 0 | Error/Default | ⏳ |
| Negative | -1 | Parse error | ⏳ |
| String value | "abc" | Parse error | ⏳ |
| Very large | 100 | Warning/Clamp | ⏳ |
| Default | omitted | 5 used | ⏳ |

## Boolean Parameters

### `print_flag_cover` (bool)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| True | true | Flag cover enabled | ⏳ |
| False | false | Flag cover disabled | ⏳ |
| String "true" | "true" | Parse error | ⏳ |
| String "false" | "false" | Parse error | ⏳ |
| Number 1 | 1 | Parse error | ⏳ |
| Number 0 | 0 | Parse error | ⏳ |
| Default | omitted | true used | ⏳ |

### `fetch_wikipedia` (bool)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| True | true | Wikipedia fetching enabled | ⏳ |
| False | false | Wikipedia fetching disabled | ⏳ |
| String value | "yes" | Parse error | ⏳ |
| Number value | 1 | Parse error | ⏳ |
| Default | omitted | false used | ⏳ |

### `detailed_page_fit_diagnostics` (bool)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| True | true | Diagnostics enabled | ⏳ |
| False | false | Diagnostics disabled | ⏳ |
| String value | "true" | Parse error | ⏳ |
| Default | omitted | false used | ⏳ |

## String Parameters

### `series_order` (String)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid: first_date | "first_date" | Series ordered by first date | ⏳ |
| Valid: first_sg | "first_sg" | Series ordered by SG number | ⏳ |
| Valid: alpha | "alpha" | Series ordered alphabetically | ⏳ |
| Valid: first_sg_desc | "first_sg_desc" | Descending SG order | ⏳ |
| Invalid | "invalid_order" | Error/Default | ⏳ |
| Empty string | "" | Error/Default | ⏳ |
| Default | omitted | "first_date" used | ⏳ |

### `expand_grouped_ranges` (String)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid: none | "none" | No expansion | ⏳ |
| Valid: placeholders | "placeholders" | Placeholder expansion | ⏳ |
| Valid: detail_fetch | "detail_fetch" | Detail fetching | ⏳ |
| Invalid | "invalid_mode" | Error/Default | ⏳ |
| Default | omitted | "none" used | ⏳ |

### `album_pages_header_position` (String)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid: centre | "centre" | Center position | ⏳ |
| Valid: left | "left" | Left position | ⏳ |
| Valid: right | "right" | Right position | ⏳ |
| Valid: C | "C" | Center (short form) | ⏳ |
| Valid: L | "L" | Left (short form) | ⏳ |
| Valid: R | "R" | Right (short form) | ⏳ |
| Valid: LR | "LR" | Left-Right | ⏳ |
| Valid: RL | "RL" | Right-Left | ⏳ |
| Invalid | "invalid_pos" | Error/Default | ⏳ |
| Default | omitted | "centre" used | ⏳ |

## Font Parameters

### `stamp_heading_font` (String)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid AlbumEasy font | "HN" | Helvetica Normal | ⏳ |
| Valid AlbumEasy font | "TN" | Times Normal | ⏳ |
| Valid AlbumEasy font | "HB" | Helvetica Bold | ⏳ |
| System font | "Arial" | System font used | ⏳ |
| Invalid font | "NonExistentFont" | Warning/Default | ⏳ |
| Empty string | "" | Error/Default | ⏳ |
| Default | omitted | "HN" used | ⏳ |

### Font Size Parameters (`stamp_heading_size`, `page_text_size`, etc.)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid range | 6.0-72.0 | Accepted | ⏳ |
| Very small | 1.0 | Warning/Clamp | ⏳ |
| Zero | 0.0 | Error/Default | ⏳ |
| Negative | -5.0 | Error/Default | ⏳ |
| Very large | 200.0 | Warning/Clamp | ⏳ |
| String | "abc" | Parse error | ⏳ |

## Color Parameters

### Color String Parameters (all `color_*` fields)

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Named color | "red" | Color applied | ⏳ |
| Hex color | "#FF0000" | Color applied | ⏳ |
| RGB value | "rgb(255,0,0)" | Color applied | ⏳ |
| Invalid color | "notacolor" | Warning/Default | ⏳ |
| Empty string | "" | Default color | ⏳ |
| Default | omitted | Default color | ⏳ |

## Parameter Interactions

### Dimension Consistency

| Test Case | Parameters | Expected Result | Status |
|-----------|------------|----------------|---------|
| Width > Height | width=50, height=20 | Warning about aspect ratio | ⏳ |
| Very small dimensions | width=1, height=1 | Layout warnings | ⏳ |
| Box height > ref height | box=50, ref_h=20 | Scaling warnings | ⏳ |

### Font Consistency

| Test Case | Parameters | Expected Result | Status |
|-----------|------------|----------------|---------|
| Inconsistent fonts | heading="HN", text="Arial" | Mixed font warning | ⏳ |
| Size hierarchy | title=20, heading=25 | Warning about hierarchy | ⏳ |

### Layout Interactions

| Test Case | Parameters | Expected Result | Status |
|-----------|------------|----------------|---------|
| Tight margins + large stamps | small margins, large stamps | Layout warnings | ⏳ |
| High stamps per row + large stamps | max_per_row=8, large stamps | Overflow warnings | ⏳ |

## Edge Cases & Error Handling

### File Path Parameters

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Valid relative path | "templates/custom.toml" | File loaded | ⏳ |
| Valid absolute path | "/usr/share/templates.toml" | File loaded | ⏳ |
| Non-existent file | "missing.toml" | Warning/Default | ⏳ |
| Permission denied | "/root/secret.toml" | Permission error | ⏳ |
| Directory instead of file | "/tmp/" | Error/Default | ⏳ |

### Special Values

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Float infinity | inf | Parse error | ⏳ |
| Float NaN | nan | Parse error | ⏳ |
| Very long string | 1000+ chars | Truncated/Warning | ⏳ |
| Unicode characters | "Ελληνικά" | Proper handling | ⏳ |
| Special characters | "Test & Co." | Proper escaping | ⏳ |

## Configuration File Structure

### TOML Syntax Errors

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Missing quotes | key = value | Parse error | ⏳ |
| Invalid TOML | [broken | Parse error | ⏳ |
| Duplicate keys | key=1, key=2 | Error/Warning | ⏳ |
| Wrong section | [wrong_section] key=1 | Ignored/Warning | ⏳ |

### Header Formatting Section

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Promotion from [header_formatting] | include_date in header section | Promoted to top-level | ⏳ |
| Conflict between sections | top=true, header.field=false | Top-level wins | ⏳ |

## Performance & Memory Tests

### Large Configuration Files

| Test Case | Input | Expected Result | Status |
|-----------|-------|----------------|---------|
| Many parameters | 100+ config keys | Handled efficiently | ⏳ |
| Large string values | 10KB+ string values | Proper memory usage | ⏳ |
| Deep nesting | Multiple [sections] | Parsed correctly | ⏳ |

## Test Implementation Strategy

### Test Automation Phases

1. **Phase 1**: Basic type validation (f32, bool, String)
2. **Phase 2**: Range validation and boundary testing  
3. **Phase 3**: Parameter interaction testing
4. **Phase 4**: File I/O and error handling
5. **Phase 5**: Performance and edge case testing

### Test Tools Needed

- [ ] Config file generator for test cases
- [ ] Automated test runner  
- [ ] Result validator
- [ ] Performance measurement tools
- [ ] Error message validator

---
**Status**: Testing matrix defined - ready for implementation
**Next**: Implement automated config parameter tests
**Last Updated**: 2025-01-11