use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct TemplateKey {
    key: String,
    full_key: String,
    line_number: usize,
}

fn parse_template_keys(template: &str) -> Vec<TemplateKey> {
    let mut out = Vec::new();
    let mut section = String::new();

    for (idx, raw_line) in template.lines().enumerate() {
        let line_number = idx + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].trim().to_string();
            continue;
        }

        if let Some(eq_idx) = line.find('=') {
            let lhs = line[..eq_idx].trim();
            if lhs.is_empty() {
                continue;
            }

            // Ignore non-key syntax edge cases; template uses normal key-value pairs.
            if lhs.contains(' ') || lhs.contains('\t') {
                continue;
            }

            let full_key = if section.is_empty() {
                lhs.to_string()
            } else {
                format!("{}.{}", section, lhs)
            };

            out.push(TemplateKey {
                key: lhs.to_string(),
                full_key,
                line_number,
            });
        }
    }

    out
}

fn collect_rs_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

#[test]
fn config_template_key_usage_audit() {
    let template = include_str!("../config_template.toml");
    let keys = parse_template_keys(template);

    let mut unique = HashSet::new();
    let unique_keys: Vec<TemplateKey> = keys
        .into_iter()
        .filter(|k| unique.insert(k.full_key.clone()))
        .collect();

    assert!(
        !unique_keys.is_empty(),
        "No keys parsed from config_template.toml"
    );

    let config_rs = fs::read_to_string("src/config.rs").expect("Failed to read src/config.rs");

    let mut rs_files = Vec::new();
    collect_rs_files(Path::new("src"), &mut rs_files);
    assert!(!rs_files.is_empty(), "No Rust files found under src/");

    let mut disconnected = Vec::new();
    let mut no_runtime_refs = Vec::new();
    let mut low_runtime_refs = Vec::new();

    for item in &unique_keys {
        let declared_in_config = config_rs.contains(&format!("pub {}:", item.key));
        if !declared_in_config {
            disconnected.push(item.clone());
            continue;
        }

        let mut total_refs = 0usize;
        let mut non_config_refs = 0usize;

        for path in &rs_files {
            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let refs = count_occurrences(&content, &item.key);
            total_refs += refs;

            if path != Path::new("src/config.rs") {
                non_config_refs += refs;
            }
        }

        if non_config_refs == 0 {
            no_runtime_refs.push((item.clone(), total_refs));
        } else if non_config_refs == 1 {
            low_runtime_refs.push((item.clone(), total_refs));
        }
    }

    println!("\\n=== CONFIG TEMPLATE KEY USAGE AUDIT ===");
    println!("keys parsed          : {}", unique_keys.len());
    println!("disconnected keys    : {}", disconnected.len());
    println!("no runtime refs      : {}", no_runtime_refs.len());
    println!("single runtime ref   : {}", low_runtime_refs.len());

    if !disconnected.is_empty() {
        println!("\\nDisconnected keys (present in template, not declared in src/config.rs):");
        for k in &disconnected {
            println!("  - {} (line {})", k.full_key, k.line_number);
        }
    }

    if !no_runtime_refs.is_empty() {
        println!("\\nLikely-unused keys (declared, but no non-config source references):");
        for (k, total_refs) in &no_runtime_refs {
            println!(
                "  - {} (line {}, total_refs_in_src={})",
                k.full_key, k.line_number, total_refs
            );
        }
    }

    if !low_runtime_refs.is_empty() {
        println!("\\nLow-reference keys (only 1 non-config reference; inspect manually):");
        for (k, total_refs) in &low_runtime_refs {
            println!(
                "  - {} (line {}, total_refs_in_src={})",
                k.full_key, k.line_number, total_refs
            );
        }
    }

    // Optional strict mode:
    // - fail if any template key is not declared in src/config.rs
    // - fail if any key appears unused outside src/config.rs
    // This is intentionally opt-in because some keys are used indirectly via helper methods.
    let strict_usage = env::var("STAMPGEN_STRICT_CONFIG_USAGE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if strict_usage {
        assert!(
            disconnected.is_empty(),
            "Strict mode: found disconnected config keys (template key not declared in src/config.rs)"
        );
        assert!(
            no_runtime_refs.is_empty(),
            "Strict mode: found keys with no non-config source references"
        );
    }
}
