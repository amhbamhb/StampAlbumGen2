---
type: Rust Function
title: fetch_and_save_summary_with_fetcher
resource: src/wikipedia/legacy.rs#L42-L69
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/wikipedia/legacy/fetch_and_save_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/e2e_fake_fetcher_wikipedia/e2e_wikipedia_fetcher_fake
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn fetch_and_save_summary_with_fetcher( topic: &str, album_dir: &str, fetcher: &dyn Fetcher, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [fetch_and_save_summary](../../../../functions/src/wikipedia/legacy/fetch_and_save_summary.md)
- [e2e_wikipedia_fetcher_fake](../../../../functions/tests/e2e_fake_fetcher_wikipedia/e2e_wikipedia_fetcher_fake.md)