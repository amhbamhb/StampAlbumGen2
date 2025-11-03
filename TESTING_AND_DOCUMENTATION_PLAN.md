# Comprehensive Testing & Documentation Plan

## Overview

Systematic audit and testing of all CLI options, configuration parameters, and AlbumEasy compatibility.

## Phase 1: Discovery & Inventory

### 1.1 CLI Options Audit

**Goal**: Document every command line flag, argument, and option

**Current Known CLI Options** (to be expanded):

- [ ] `--database PATH` - SQLite database path
- [ ] `--output PATH` - Output file path  
- [ ] `--config PATH` - Config file path
- [ ] `--help` - Show help
- [ ] `--version` - Show version
- [ ] **TODO**: Scan main.rs and clap definitions for complete list

**Actions**:

- [ ] Extract all CLI options from source code
- [ ] Test each option individually
- [ ] Test invalid combinations
- [ ] Validate help text accuracy
- [ ] Check for missing implementations

### 1.2 Configuration Parameters Audit  

**Goal**: Document every config.toml parameter

**Current Known Config Parameters** (to be expanded):

- [ ] `max_stamps_per_row` - Controls stamps per row (default: 5)
- [ ] `row_start_es_spacing` - Horizontal spacing between stamps
- [ ] `image_spacing_mm` - Vertical spacing between images
- [ ] `reference_width_mm` - Default stamp width (default: 25.0)
- [ ] `reference_height_mm` - Default stamp height (default: 30.0)
- [ ] `force_dpi` - Override DPI calculation
- [ ] `preserve_real_image_size` - Don't scale images to stamp_box_height
- [ ] **TODO**: Scan config.rs for complete parameter list

**Actions**:

- [ ] Extract all config parameters from config.rs
- [ ] Document purpose, valid values, defaults for each
- [ ] Test parameter validation and error handling
- [ ] Check for parameter interactions/conflicts

### 1.3 AlbumEasy Command Reference

**Goal**: Validate against AlbumEasy_4.2.1.pdf manual

**AlbumEasy Commands Generated** (to be expanded):

- [ ] `ROW_START_ES (font size spacing vspace)` - Start row with equal spacing
- [ ] `ROW_START_FS (font size spacing vspace)` - Start row with fixed spacing  
- [ ] `STAMP_ADD_IMG (width height path heading text1 text2)` - Add stamp image
- [ ] `STAMP_HEADING(font size text)` - Add stamp heading
- [ ] `PAGE_TEXT_CENTRE (font size text scale)` - Centered page text
- [ ] `PAGE_VSPACE (amount)` - Vertical spacing
- [ ] `COLOR_STAMP_BACKGROUND(color)` - Set stamp background color
- [ ] **TODO**: Extract all generated commands and cross-reference with PDF

## Phase 2: Testing Framework

### 2.1 CLI Testing Matrix

```
Option          | Valid Values | Invalid Values | Expected Behavior
--database      | valid.db     | nonexistent    | Load DB / Error  
--output        | file.txt     | /invalid/path  | Write file / Error
--config        | config.toml  | invalid.toml   | Load config / Error
[... complete matrix ...]
```

### 2.2 Config Testing Matrix  

```
Parameter           | Valid Values | Invalid Values | Default | Interactions
max_stamps_per_row  | 1-10         | 0, -1, "abc"  | 5       | Affects row grouping
row_start_es_spacing| 0.1-10.0     | -1, "abc"      | 1.0     | Used in ROW_START_ES
[... complete matrix ...]
```

### 2.3 AlbumEasy Compatibility Matrix

```
Generated Command    | PDF Reference | Syntax Valid | Parameters Correct
ROW_START_ES        | Page X        | ✓/✗          | ✓/✗
STAMP_ADD_IMG       | Page Y        | ✓/✗          | ✓/✗  
[... complete matrix ...]
```

## Phase 3: Implementation Plan

### 3.1 Small Batch Approach

**Batch 1**: Core CLI options (--database, --output, --config, --help)
**Batch 2**: Configuration loading and validation  
**Batch 3**: Image and dimension parameters
**Batch 4**: AlbumEasy command generation
**Batch 5**: Advanced features and edge cases

### 3.2 Deliverables Per Batch

- [ ] Complete parameter inventory
- [ ] Test cases for all combinations
- [ ] Documentation updates
- [ ] Integration tests
- [ ] Error handling validation

## Questions for Clarification

1. **Scope**: Should we test only current functionality or also identify missing features?
2. **Priority**: Which aspects are most critical (CLI, config, AlbumEasy compatibility)?
3. **Output**: What format for final documentation (markdown, generated docs, etc.)?
4. **Testing**: Unit tests, integration tests, or manual test procedures?
5. **AlbumEasy**: Should we validate against other versions or just 4.2.1?

## Next Steps

1. Start with **Batch 1** - Core CLI options audit
2. Create systematic testing procedures  
3. Begin AlbumEasy PDF cross-reference
4. Set up documentation framework
5. Implement tests in small, manageable chunks

---
**Status**: Planning phase - awaiting clarification and approval to proceed
**Last Updated**: 2025-11-01
