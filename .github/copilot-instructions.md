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
