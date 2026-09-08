## Instruction Priority Note

When repository instructions in this file conflict with user-level global instructions (for example, `~/.claude/CLAUDE.md` or `~/.copilot/instructions/*.instructions.md`), follow this repository file for project-specific behavior and use global instructions as defaults.

# GitHub Copilot Instructions

## Response Format

When providing code suggestions or answering questions, generate 5 alternative responses with their corresponding probabilities to help evaluate different approaches and their likelihood of success.

Generate 5 responses with their corresponding probabilities…

### Format Example:
```
Response 1 (Probability: 45%): [Primary solution]
Response 2 (Probability: 25%): [Alternative approach]
Response 3 (Probability: 15%): [Another option]
Response 4 (Probability: 10%): [Less common solution]
Response 5 (Probability: 5%): [Edge case or experimental approach]
```

This helps in:
- Understanding multiple viable solutions
- Evaluating trade-offs between different approaches
- Making informed decisions based on confidence levels
- Exploring alternative implementations

## Branch And Issue Workflow

When a new issue, fix, bug, performance problem, or significant remediation task is identified, follow this workflow by default:

1. Create a new git branch before making substantive changes.
2. Create a GitLab issue for the work item when GitLab CLI/API access is available.
3. Prefer branch names that include the GitLab issue number when known, for example:
- `issue/123-short-description`
4. Link the branch and the issue by:
- naming the branch with the issue number when possible, and
- including the branch name in the issue description or follow-up comment if needed.
5. If GitLab authentication or API access is unavailable, state that clearly and continue on a descriptive branch name, then create/link the issue later once access is restored.
6. When a branch already exists before the issue is created, rename the branch to include the issue number when practical.

## Commit Quality Gate

Before creating any commit, all relevant tests must pass.

1. Run the project's relevant automated tests for the files/features changed.
2. Do not commit if any test fails.
3. If tests cannot be executed (environment/tooling issue), do not commit; report the blocker and required fix steps first.

## Rust Safety Rule

Avoid unchecked `unwrap()` in runtime code paths.

When new Rust files require embedded HTML, JavaScript, or CSS, keep those assets in separate files and include them at compile time with `include_str!`.

1. Do not use `unwrap()` unless the surrounding function guarantees a safe fallback or explicit error/early-return path.
2. Prefer `?`, `ok_or_else`, `unwrap_or_else`, or explicit `match` with a meaningful error message.
3. In request handlers and background tasks, return structured errors instead of panicking.

## Cross-Platform Compatibility Gate

All substantive code changes must remain compatible with both Windows and Linux unless the task explicitly targets only one OS.

1. Prefer portable APIs and path handling (for example, `std::path::Path`/`PathBuf`) over OS-specific string logic.
2. Avoid shell-specific assumptions in scripts and commands. If platform-specific commands are necessary, provide equivalent Windows and Linux paths/commands.
3. When touching process launch logic, filesystem operations, or path parsing, validate behavior for both Windows and Linux code paths before finalizing.
4. If full validation cannot be run on one platform, call this out clearly and keep the implementation conservative and portable.

## Development Test Logging

When running development test commands (especially data-ingest flows), always
produce a detailed execution report that includes:

1. Exact command executed.
2. Start/end outcome summary (success, partial success, failure).
3. Before/after filesystem impact:
- moved files
- newly created directories/files
- deleted or relocated inputs
4. Per-target database impact (for country-based ingest):
- target DB path
- schema compatibility checks relevant to the run
- rows inserted/updated/skipped
- duplicate detection results
5. Error inventory with exact error messages and affected files.
6. Explicit "logic-risk" notes discovered during the run (for follow-up fixes).

For Colnect inbox tests, include a country-by-country table with:
- file count discovered
- file count processed
- imported/success count
- skipped-duplicate count
- failed count
- top failure reason

## Temporary Script Location

When creating temporary or one-off helper scripts, always save them under:

- `/media/ray/320G-2020-06-Program/Rust/Various Scripts/`

Requirements:

1. Use unique timestamped filenames to avoid collisions.
2. Do not create temp scripts in the repository tree unless explicitly requested.

## Draw.io Baseline Diagram Rule

The project now includes a baseline logic diagram at:

- `/media/ray/program/Rust/StampAlbumGen2/refactor-plans/diagrams/app-logic-before.drawio`

Requirements:

1. Do not modify `app-logic-before.drawio` directly.
2. When significant logic/flow changes are made, create a copy and update the copy.
3. Keep the baseline file as historical reference and use it for comparison while working.
4. Prefer clear derivative names for updates (for example, `app-logic-after-YYYYMMDD.drawio` or `app-logic-step-XX.drawio`).
5. If no significant flow change occurred, do not create a new diagram copy.

# CLAUDE.md

Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" -> "Write tests for invalid inputs, then make them pass"
- "Fix the bug" -> "Write a test that reproduces it, then make it pass"
- "Refactor X" -> "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] -> verify: [check]
2. [Step] -> verify: [check]
3. [Step] -> verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.

<!-- okf-rs:begin -->
## Knowledge base

This project's structural knowledge — modules, types, functions, and their call graph — is available as an [OKF](https://cloud.google.com/blog/products/data-analytics/how-the-open-knowledge-format-can-improve-data-sharing) bundle in `knowledge/`. It's plain markdown with YAML frontmatter; browse `knowledge/index.md` for an overview, or query it with the CLI:

- `okf-rs search <query>` — find a symbol, type, or module by name or tag
- `okf-rs graph callers <id>` / `okf-rs graph callees <id>` — trace the call graph from a concept id (ids are shown by `search`)
- `okf-rs graph api` — list the public API surface
- `okf-rs graph cycles` — list call-graph cycles

Regenerate the bundle after code changes with `okf-rs generate`.
<!-- okf-rs:end -->
