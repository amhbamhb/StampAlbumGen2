# Documentation Versioning Guide

## Overview
This guide outlines best practices for maintaining versioned documentation for the Stamp Album Generator project.

## Current Files
- `.github/instructions` - Project guidelines and structure reference
- `README.md` - Main project documentation
- `docs/versions/` - Historical versions of documentation

## Versioning System

### File Naming Convention
```
{filename}_{YYYY-MM-DD}_v{version}.md
```
Example: `README_2025-09-18_v1.md`

### Version Headers
Each documentation file includes:
- Version number (incremented with each significant update)
- Last updated date
- Reference to previous version

## Best Practices

### When to Create a New Version
- Major structural changes to the project
- Significant updates to dependencies or features
- Changes to core functionality or API
- Updates to project organization or conventions

### When to Update In-Place
- Minor fixes and corrections
- Additional examples or clarifications
- Updated contact information or links
- Small formatting improvements

### Maintenance Process
1. Before making significant changes, create a backup version
2. Update the version header in the main file
3. Update the "Previous Versions" reference
4. Test that all links and references still work

## Version History

### Instructions File
- v1.0: Initial version (outdated structure)
- v1.1: Updated to match current project structure (2025-09-18)

### README File
- v1.0: Initial comprehensive documentation
- v1.1: Updated project structure and dependencies
- v1.2: Added version headers and core functions documentation

## Tools
- Use `cp` to create backups: `cp file.md docs/versions/file_$(date +%Y-%m-%d)_v1.md`
- Version headers are maintained in the main files
- Historical versions are stored in `docs/versions/` for reference