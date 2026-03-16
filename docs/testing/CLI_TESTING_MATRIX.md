# CLI Testing Matrix

## Overview

This document outlines systematic testing of all CLI options found in main.rs. Each option is tested for:

1. **Valid values** - Acceptable input ranges and combinations
2. **Invalid values** - Input that should cause errors or helpful suggestions
3. **Required combinations** - CLI options that must be used together
4. **Conflicting combinations** - CLI options that cannot be used together
5. **Error handling** - Quality of error messages and suggestions

## Complete CLI Options from main.rs

### Core Options

| Option | Type | Default | Required | Description |
|--------|------|---------|----------|-------------|
| `--source` | String | "stampworld" | No | Source type: stampworld, touchstamps, extractfile, colnectexport |
| `--site-name` | String | None | Conditional | Site name for scraping (required for stampworld source) |
| `--album-name` | String | None | No | Album name to generate |
| `--db-path` | String | None | No | Database path (for output-only mode) |
| `--base-url` | String | None | No | Optional base URL override for StampWorld |

### Year Range Options

| Option | Type | Default | Required | Description |
|--------|------|---------|----------|-------------|
| `--start-year` | i32 | 1840 | No | Starting year for scraping |
| `--end-year` | i32 | BUILD_YEAR | No | Ending year for scraping (current year) |

### Behavior Control Options

| Option | Type | Default | Required | Description |
|--------|------|---------|----------|-------------|
| `--overwrite-album` | bool | false | No | Overwrite existing album files |
| `--overwrite-images` | bool | false | No | Overwrite existing image files |
| `--max-pages` | i32 | 0 | No | Maximum pages to fetch (0 = no limit) |
| `--stop-after-no-hits` | i32 | 3 | No | Stop after N pages with zero stamps |
| `--yes` / `-y` | bool | false | No | Accept top suggestions automatically |

## CLI Testing Matrix

### Basic Functionality Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Help display | `--help` | Shows complete help text | ⏳ |
| Version display | `--version` | Shows version information | ⏳ |
| No arguments | (empty) | Shows guidance + help | ⏳ |
| Invalid arguments | `--invalid-flag` | Error with suggestions | ⏳ |

### Source Option Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Valid: stampworld | `--source stampworld` | Source accepted | ⏳ |
| Valid: touchstamps | `--source touchstamps` | Source accepted | ⏳ |
| Valid: extractfile | `--source extractfile` | Source accepted | ⏳ |
| Valid: colnectexport | `--source colnectexport` | Source accepted | ⏳ |
| Invalid source | `--source invalid` | Error + suggestions | ⏳ |
| Typo: stamworld | `--source stamworld` | Suggest "stampworld" | ⏳ |
| Typo: stampworld-typo | `--source stampworld-typo` | Suggest "stampworld" | ⏳ |
| Empty source | `--source ""` | Error message | ⏳ |

### Database Path Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Valid DB path | `--db-path test.db` | DB loaded/created | ⏳ |
| Non-existent dir | `--db-path /invalid/path.db` | Directory creation attempt | ⏳ |
| Permission denied | `--db-path /root/test.db` | Permission error | ⏳ |
| Directory as DB | `--db-path /tmp/` | Error: not a file | ⏳ |
| Corrupt DB file | `--db-path corrupt.db` | Error + suggestions | ⏳ |
| No DB path (scraping) | `--source stampworld --site-name GB` | Use default DB path | ⏳ |

### Year Range Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Valid range | `--start-year 1900 --end-year 2000` | Range accepted | ⏳ |
| Invalid range | `--start-year 2000 --end-year 1900` | Error: start > end | ⏳ |
| Negative start | `--start-year -1` | Range validation | ⏳ |
| Future end year | `--end-year 9999` | Range validation | ⏳ |
| Start = End | `--start-year 2000 --end-year 2000` | Single year | ⏳ |
| Very large range | `--start-year 1800 --end-year 2100` | Warning about size | ⏳ |

### Boolean Flag Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Overwrite album | `--overwrite-album` | Flag enabled | ⏳ |
| Overwrite images | `--overwrite-images` | Flag enabled | ⏳ |
| Auto-accept yes | `--yes` | Flag enabled | ⏳ |
| Auto-accept -y | `-y` | Flag enabled (short form) | ⏳ |
| Combined flags | `--overwrite-album --overwrite-images` | Both flags enabled | ⏳ |

### Combination Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| DB-only mode | `--db-path X.db --album-name Y` | Output-only mode | ⏳ |
| Scrape mode | `--source stampworld --site-name GB` | Scraping mode | ⏳ |
| Complete scrape | `--source stampworld --site-name GB --album-name GB --start-year 1900 --end-year 1950` | Full scraping | ⏳ |
| Invalid combo: DB + source | `--db-path X.db --source touchstamps` | Error or override | ⏳ |
| Missing site-name | `--source stampworld` (no site-name) | Error + guidance | ⏳ |
| Album name fallback | `--source stampworld --album-name Canada` | Site name inferred | ⏳ |

### Error Handling & User Experience Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Friendly error messages | Invalid combinations | Clear, helpful errors | ⏳ |
| Suggestion quality | `--source stamworld` | High-quality suggestion | ⏳ |
| Auto-correction | `--yes --source stamworld` | Auto-accept if high confidence | ⏳ |
| Interactive prompts | Misspelled source (interactive) | Prompt for confirmation | ⏳ |
| Non-interactive | Misspelled source (script) | No auto-accept | ⏳ |
| Help quality | `--help` | Complete, accurate help | ⏳ |

### Edge Cases & Special Values

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Very long site name | `--site-name "Very-Long-Site-Name-With-Many-Hyphens"` | Handled correctly | ⏳ |
| Special characters | `--site-name "British-Columbia!Vancouver-Island"` | Proper URL encoding | ⏳ |
| Unicode characters | `--site-name "Ελλάδα"` | Unicode handling | ⏳ |
| Empty strings | `--site-name ""` | Error handling | ⏳ |
| Whitespace | `--site-name " Canada "` | Trimming/handling | ⏳ |
| Case sensitivity | `--source STAMPWORLD` | Case handling | ⏳ |

### Performance & Resource Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Max pages limit | `--max-pages 5` | Respects page limit | ⏳ |
| No hits threshold | `--stop-after-no-hits 1` | Stops after 1 empty page | ⏳ |
| Large year range | `--start-year 1800 --end-year 2100` | Performance acceptable | ⏳ |
| Network timeouts | Scraping with network issues | Graceful timeout handling | ⏳ |
| Disk space | Large album generation | Disk space error handling | ⏳ |

### Base URL Override Tests

| Test Case | Command | Expected Result | Status |
|-----------|---------|----------------|---------|
| Custom base URL | `--base-url "https://custom.stampworld.com"` | URL override works | ⏳ |
| Invalid URL | `--base-url "not-a-url"` | URL validation | ⏳ |
| URL with parameters | `--base-url "https://site.com/path?param=value"` | Complex URL handling | ⏳ |
| Local URL | `--base-url "http://localhost:8080"` | Local development support | ⏳ |

## Error Message Quality Assessment

### Required Error Message Elements

For each error condition, verify the error message includes:

1. **Clear description** of what went wrong
2. **Specific suggestions** for how to fix it
3. **Example commands** that would work
4. **Context** about why the error occurred

### Error Message Test Cases

| Error Condition | Required Message Elements | Quality Score | Status |
|----------------|---------------------------|---------------|---------|
| Unrecognized source | Description + suggestions + examples | ⏳ | ⏳ |
| Missing site-name | Description + guidance + examples | ⏳ | ⏳ |
| Invalid year range | Description + valid range info | ⏳ | ⏳ |
| DB file issues | Description + troubleshooting steps | ⏳ | ⏳ |
| Permission errors | Description + permission fix guidance | ⏳ | ⏳ |

## Implementation Strategy

### Test Automation Framework

1. **CLI Test Runner**: Automated execution of all CLI combinations
2. **Output Validator**: Verify expected outputs and error messages
3. **Resource Monitor**: Check performance and resource usage
4. **Error Quality Checker**: Validate error message completeness

### Test Phases

1. **Phase 1**: Basic option parsing and validation
2. **Phase 2**: Option combination testing
3. **Phase 3**: Error handling and user experience
4. **Phase 4**: Performance and edge cases
5. **Phase 5**: Integration with config system

### Test Tools Needed

- [ ] CLI argument generator
- [ ] Automated test runner
- [ ] Output comparison tools
- [ ] Error message validator
- [ ] Performance measurement tools

## Known Issues & Missing Features

### Potentially Missing CLI Options

Based on the config audit, these CLI options might be useful:

- [ ] `--config PATH` - Custom config file path
- [ ] `--verbose` / `--quiet` - Logging level control
- [ ] `--dry-run` - Preview without making changes
- [ ] `--output-dir PATH` - Custom output directory
- [ ] `--list-sources` - Show available source types
- [ ] `--list-sites` - Show available site names for scraping

### Error Handling Improvements

- [ ] Better suggestions for misspelled site names
- [ ] Validation of site names against known StampWorld sites
- [ ] Progress indicators for long-running operations
- [ ] Interrupt handling for graceful cancellation

### User Experience Enhancements

- [ ] Auto-completion support for shells
- [ ] Configuration file generation wizard
- [ ] Better help organization with examples
- [ ] Interactive mode for guided setup

---
**Status**: CLI testing matrix defined - ready for automated testing implementation  
**Next**: Build automated CLI test runner and validation framework
**Last Updated**: 2025-01-11