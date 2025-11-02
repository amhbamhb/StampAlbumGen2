# CLI Options Inventory

## Complete CLI Options from main.rs

### Core Options
| Option | Type | Default | Required | Description |
|--------|------|---------|----------|-------------|
| `--source` | String | "stampworld" | No | Source type: stampworld, touchstamps, extractfile, colnectexport |
| `--site-name` | String | None | No | Site name for scraping (e.g. 'Great-Britain', 'Canada') |
| `--album-name` | String | None | No | Album name to generate |
| `--db-path` | String | None | No | Database path (for output-only mode) |
| `--base-url` | String | None | No | Optional base URL to use when scraping StampWorld |

### Year Range Options
| Option | Type | Default | Required | Description |
|--------|------|---------|----------|-------------|
| `--start-year` | i32 | 1840 | No | Starting year for scraping |
| `--end-year` | i32 | BUILD_YEAR | No | Ending year for scraping (set at compile time) |

### Behavior Control Options
| Option | Type | Default | Required | Description |
|--------|------|---------|----------|-------------|
| `--overwrite-album` | bool | false | No | Overwrite existing album files |
| `--overwrite-images` | bool | false | No | Overwrite existing image files |
| `--max-pages` | i32 | 0 | No | Maximum pages to fetch when scraping (0 = no limit) |
| `--stop-after-no-hits` | i32 | 3 | No | Stop after N consecutive pages with zero stamps |
| `--yes` / `-y` | bool | false | No | Accept top suggestions automatically (for scripts) |

## CLI Option Testing Matrix

### Basic Functionality Tests
- [ ] `--help` - Shows complete help text
- [ ] `--version` - Shows version information  
- [ ] No arguments - Shows guidance message and help
- [ ] Invalid arguments - Shows appropriate error messages

### Source Option Tests
| Test Case | Expected Result | Status |
|-----------|----------------|---------|
| `--source stampworld` | ✓ Valid | ⏳ |
| `--source touchstamps` | ✓ Valid | ⏳ |
| `--source extractfile` | ✓ Valid | ⏳ |
| `--source colnectexport` | ✓ Valid | ⏳ |
| `--source invalid` | Error + suggestions | ⏳ |
| `--source stamworld` (typo) | Suggest "stampworld" | ⏳ |

### Database Path Tests
| Test Case | Expected Result | Status |
|-----------|----------------|---------|
| `--db-path valid.db` | Load database | ⏳ |
| `--db-path nonexistent.db` | Create if parent exists | ⏳ |
| `--db-path /invalid/path/db` | Error message | ⏳ |
| No `--db-path` with scraping | Use default path | ⏳ |

### Year Range Tests
| Test Case | Expected Result | Status |
|-----------|----------------|---------|
| `--start-year 1900 --end-year 2000` | Valid range | ⏳ |
| `--start-year 2000 --end-year 1900` | Error: invalid range | ⏳ |
| `--start-year -1` | Validate behavior | ⏳ |
| `--end-year 9999` | Validate behavior | ⏳ |

### Boolean Flag Tests
| Test Case | Expected Result | Status |
|-----------|----------------|---------|
| `--overwrite-album` | Sets flag to true | ⏳ |
| `--overwrite-images` | Sets flag to true | ⏳ |
| `--yes` | Auto-accept suggestions | ⏳ |
| `-y` (short form) | Same as --yes | ⏳ |

### Combination Tests
| Test Case | Expected Result | Status |
|-----------|----------------|---------|
| DB mode: `--db-path X --album-name Y` | Output-only mode | ⏳ |
| Scrape mode: `--source stampworld --site-name X` | Scraping mode | ⏳ |
| Invalid combo: `--db-path X --source touchstamps` | Error or override | ⏳ |
| Missing required: `--source stampworld` (no site-name) | Error message | ⏳ |

## Error Handling Tests

### User Experience Tests
- [ ] Friendly error messages for common mistakes
- [ ] Helpful suggestions when options are misspelled
- [ ] Clear guidance when required options are missing
- [ ] Proper validation of file paths and permissions

### Edge Cases
- [ ] Very long command lines
- [ ] Special characters in paths/names
- [ ] Unicode characters in album names
- [ ] Network timeouts during scraping
- [ ] Disk space issues during file creation

## Documentation Checks

### Help Text Validation
- [ ] All options documented in `--help`
- [ ] Examples provided for common use cases
- [ ] Default values clearly indicated
- [ ] Required vs optional parameters marked

### README.md Updates Needed
- [ ] Complete CLI reference section
- [ ] Example commands for all use cases
- [ ] Troubleshooting guide for common errors
- [ ] Migration guide for config changes

## Missing Features Analysis

### Potentially Missing CLI Options
- [ ] `--config PATH` - Custom config file path
- [ ] `--verbose` / `--quiet` - Logging control
- [ ] `--dry-run` - Preview without changes
- [ ] `--output PATH` - Custom output file path
- [ ] `--format` - Output format selection
- [ ] `--list-sites` - Show available sites for scraping

### Implementation Priority
1. **High**: Add missing critical options (config path, output path)
2. **Medium**: Improve user experience (verbose, dry-run)
3. **Low**: Convenience features (list-sites, format options)

---
**Status**: CLI audit in progress - main options documented
**Next**: Complete config parameter audit
**Last Updated**: 2025-11-01