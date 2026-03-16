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
