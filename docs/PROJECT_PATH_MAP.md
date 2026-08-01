# Project Path Map: App vs Website

This document defines path ownership after splitting the website into a separate repository.

## Scope Summary

- App repository: `StampAlbumGen2` (this repository)
- Website repository: `../StampAlbumGen2-website` (sibling checkout created from subtree history)

## Path Ownership

### App-Owned Paths (stay in this repository)

- `src/`
- `crates/`
- `tests/`
- `Albums/`
- `Cargo.toml` (app workspace excludes website member)
- `config*.toml`
- `docs/` (except website-only docs moved to website repo)

### Website-Owned Paths (moved out of this repository)

- `stampalbumgenerator2-site/`
- `scripts/prepare_site_release_bundle.sh`
- `scripts/deploy_site_to_opt.sh`
- Website release bundle output under `build/site-release-bundle*/` is generated from website repo workflows.

## New Website Repository

A separate repository was created at:

- `../StampAlbumGen2-website`

It was initialized from this repo's subtree history for `stampalbumgenerator2-site/`.

## Optional Linkage Patterns

### Option A: Sibling Repositories (recommended)

Keep two independent checkouts side-by-side:

- `../StampAlbumGen2`
- `../StampAlbumGen2-website`

### Option B: Add website as a submodule (optional)

If you want the app repo to reference a pinned website revision without mixing histories:

```bash
git submodule add <website-remote-url> website
```

Then update intentionally when needed:

```bash
git submodule update --remote --merge website
```

## Commit Discipline

- App changes: commit only in `StampAlbumGen2`
- Website changes: commit only in `StampAlbumGen2-website`
- Avoid mixed commits spanning both repositories.
