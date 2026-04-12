# SG to Catalog Number Wording Migration Plan (Deferred)

Date: 2026-04-09
Status: Deferred (planning complete, implementation pending)
Owner: TBD

## Goal

Replace user-facing wording that currently says "SG" with a clearer label such as "Catalog Number" (or a final selected variant), while keeping database schema and internal storage fields unchanged.

## Constraints

- Do not rename DB columns such as `sg_number`.
- Do not change matching/storage semantics unless explicitly approved.
- Keep behavior stable; this is primarily a user-facing language cleanup.

## Recommended Final Wording

- Primary label: `Catalog Number`
- Optional short label where space is limited: `Catalog #`
- Keep specific catalog family names where meaningful (for example: "Stanley Gibbons (SG)", "Scott (SN)").

## Scope

### In Scope

- Review UI labels, placeholders, headings, and alerts.
- Config file defaults/comments that are user-facing.
- Placeholder image title text shown to users.
- User-facing service messages in review UI backend.

### Out of Scope

- DB schema, migrations, and DB field names.
- Internal variable names and code keys that are behavior-critical (for example `f_replace_sg`, sort mode keys like `year_sg`).
- Historical backup docs under `docs/versions` unless specifically requested.

## Primary Files To Update

- `src/review_ui/views.rs`
- `src/review_ui/matcher_service.rs`
- `src/placeholder_image.rs`
- `config.toml`
- `config_template.toml`

## Phase Plan

### Phase 1: UX Copy Decision (15-30 min)

- Confirm final wording set:
  - `SG` -> `Catalog Number`
  - `SG / Ref Code` -> `Catalog Number / Ref Code`
  - `replace SG number` -> `replace catalog number`
- Confirm whether to keep "Ord SG" as-is (because it maps to a specific Colnect field) or display as "Ord Catalog (SG)".

### Phase 2: UI Text-Only Changes (1.5-3 hours)

- Update visible text in review UI only.
- Preserve IDs/keys/values used by logic and backend contracts.
- Examples of safe edits:
  - labels
  - placeholders
  - section text
  - table row titles
  - alerts and validation messages

### Phase 3: Config Wording Cleanup (20-45 min)

- Update comments in config templates where wording is SG-specific.
- Recommended default in `config.toml`:
  - `stamp_heading_label = "auto"`
  - This avoids forcing "SG" in generated headings.

### Phase 4: Quick Validation (45-90 min)

- Start review UI and verify:
  - No broken actions (assign, apply, save edits).
  - Sort options still work (even if labels changed).
  - Catalog default update workflow still works.
- Run targeted checks/tests relevant to review UI and config parsing.

### Phase 5: Optional Follow-Up Sweep (30-90 min)

- Clean up additional user-facing non-UI strings if desired.
- Leave debug/internal log text for a separate technical-debt pass.

## Effort Estimate

- Safe, user-facing only path: 2 to 5 hours total.
- With broader user-facing message cleanup: 1 working day.
- With key/identifier renaming across logic contracts: 1-2 days (higher risk, not recommended in first pass).

## Risk Register

1. Hidden coupling to UI element IDs/keys.

- Mitigation: change display text only, not ID/value attributes.

1. Sort/filter behavior regressions.

- Mitigation: keep sort mode values (`sg`, `year_sg`, `series_sg`) unchanged in code; only relabel display text.

1. Confusion between generic catalog number and specific SG ordinal fields.

- Mitigation: use explicit labels where SG is a specific source field, for example `Ordinal (SG)`.

## Acceptance Criteria

- No visible "SG" remains in targeted review UI contexts except where SG is intentionally catalog-family specific.
- Config defaults/comments align with generic catalog wording.
- No DB schema or migration changes.
- Core review workflows behave identically before/after.

## Rollback Strategy

- Single commit for this wording migration.
- Revert commit if any UI regression appears.

## Implementation Checklist

- [ ] Confirm final wording glossary.
- [ ] Update `src/review_ui/views.rs` user-facing text.
- [ ] Update `src/review_ui/matcher_service.rs` user-facing validation/status messages.
- [ ] Update `src/placeholder_image.rs` placeholder title text.
- [ ] Update `config.toml` and `config_template.toml` comments/default label setting.
- [ ] Run targeted validation and smoke test review UI.
- [ ] Commit with clear message.
