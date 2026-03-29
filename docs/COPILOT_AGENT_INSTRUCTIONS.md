# Copilot Agent — DB import instructions

Purpose

- Ensure all DB import work for StampAlbumGen2 consistently references the sample StampWorld webpage provided in the repository.

Canonical sample file

- Path: samples/reference/Example-Stampworldwebpage.txt
- This file contains the canonical HTML structure and example data to be used when parsing and mapping stamp records into the database.

Rules for DB import tasks

1. Always load and reference the canonical sample file above before creating or modifying any import logic or parsers.
2. Use the HTML structure and classes/attributes in the sample file as the authoritative mapping for extraction. Key elements to map:
   - outer container: div.container-fluid.content_table with id "group_box_<group_id>"
   - header fields inside .table_header: title link text (year + description), date, designer, engraving, perforation
   - images: spans with class stamp_img and attributes data-stamp-type and data-stamp-group-id; image src and alt provide type and image URL
   - table rows: tr.stamp_tr containing data-stamp-group-id and data-stamp-type; per-row number in &lt;a name&gt; / id; type column; D column; color column; quantity in parentheses in the hidden-xs column; price columns and currency cells
3. Preserve stamp groupings: group-level summary rows (e.g. combined group 1040-1043) must be recognized separately from individual stamp rows.
4. Handle multiple stamp types per group (ANZ, AOA, AOB, AOC in the sample) and map them to separate DB records as appropriate.
5. Use the sample file to create unit tests for parsers so that future changes to parsing logic can be validated against this canonical example.

Notes

- If additional sample pages from stampworld.com are added, include them in docs/ and update this instruction file to reference them.
- The sample file is authoritative for field names and HTML structure for import tasks unless explicitly replaced by a newer canonical sample.

Debugging outside VS Code

-------------------------

If running with RUST_LOG=debug inside VS Code causes issues (e.g. terminal crashes), use the helper script instead which runs the app in the background and writes a timestamped debug log under `Logs/`:

```
./scripts/run_debug_no_vscode.sh -- --source stampworld --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976
```

This will write output to `Logs/debug-run-<timestamp>.log` and run the process in the background. Use `tail -f` on the logfile to follow progress.

Contact

- If the sample file appears to diverge from live site structure, document the differences and add an updated example file under docs/ before changing parser logic.
 
Agent tool usage and branching (important)

- DO NOT use GitHub or GitHub APIs for branching, pull requests, issues, or other interactions. This project does not use GitHub-hosted workflows or PRs, and the automated agent must not create or reference GitHub PRs, issues, or API calls.
- Prefer local git workflows: create local branches using the repository's configured git remote and branch naming conventions (e.g. `refactor/<file>`). Commit locally and then coordinate with the project maintainers for any pushes or remote PR creation using the project's preferred hosting. This project uses a self-hosted GitLab instance by default; prefer that as the remote host when instructed.

Preferred GitLab instance and repository (project-specific)

- Self-hosted GitLab instance: [gitinfo.laserspider.info](https://gitinfo.laserspider.info/)
- This repository on that instance: [rust-stamp_alblum_generate](https://gitinfo.laserspider.info/rust-apps/rust-stamp_alblum_generate)

Example remote URLs (confirm with maintainers before pushing):

- SSH: `git@gitinfo.laserspider.info:rust-apps/rust-stamp_alblum_generate.git`
- HTTPS: `https://gitinfo.laserspider.info/rust-apps/rust-stamp_alblum_generate.git`

When permitted to push branches or create merge requests on the self-hosted GitLab instance, prefer the SSH URL for authenticated pushes from developer machines. This repository's contributors typically have their SSH keys configured in their zsh environment; prefer SSH by default (for example, `git@gitinfo.laserspider.info:rust-apps/rust-stamp_alblum_generate.git`).

Notes on SSH usage

- Use the SSH remote when performing authenticated pushes or creating merge requests from developer machines or CI that has SSH keys configured.
- Do not store private keys or credentials in the repository. If CI needs access, ask maintainers for a secure mechanism (deploy keys, CI variables).
- If you detect SSH is not configured on the system, report that back and ask for permission before attempting any HTTPS push.
- If you need to create a remote branch or push commits, always ask the maintainer which remote/host to use before performing the push. Do not assume `origin` points to GitHub. When permitted to push, prefer the project's self-hosted GitLab instance; ask for the exact remote URL (for example `git@gitlab.example.local:group/repo.git`) and any required authentication details. Do not expose credentials in logs or the repo.
- Do not open or modify external-hosted issues on behalf of the project. For task tracking, update the repository's internal `todo`/docs files or send a direct report to the maintainers.
- Avoid calling any tools or services that implicitly interact with GitHub (for example, any automated GitHub PR creation or GitHub API wrappers). If such a tool appears necessary, ask for explicit permission and the exact remote/host to use.

If in doubt, prefer a local branch and a clear commit message and then notify the maintainers with the branch name and the changes to be pushed by a human.
