use std::path::{Path, PathBuf};

struct RestoreCwd(PathBuf);

impl Drop for RestoreCwd {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.0).expect("restore test working directory");
    }
}

fn copy_directory(source: &Path, target: &Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(target)?;
    let mut count = 0;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let destination = target.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            count += copy_directory(&entry.path(), &destination)?;
        } else {
            std::fs::copy(entry.path(), destination)?;
            count += 1;
        }
    }
    Ok(count)
}

fn sanitized_config(path: &Path) -> String {
    fn scrub(value: &mut toml::Value) {
        match value {
            toml::Value::Table(table) => {
                for key in ["username", "password", "auto_login", "runtime"] {
                    table.remove(key);
                }
                for (_, value) in table.iter_mut() {
                    scrub(value);
                }
            }
            toml::Value::Array(values) => {
                for value in values {
                    scrub(value);
                }
            }
            _ => {}
        }
    }
    let mut value: toml::Value =
        toml::from_str(&std::fs::read_to_string(path).expect("read fixture configuration"))
            .expect("parse fixture configuration");
    scrub(&mut value);
    toml::to_string_pretty(&value).expect("serialize sanitized configuration")
}

#[tokio::test]
#[ignore = "requires a local Spain fixture; never modifies the source album"]
async fn spain_colnect_layout_packs_rich_rows_without_losing_stamps() {
    let source = PathBuf::from(
        std::env::var_os("STAMP_LAYOUT_SPAIN_FIXTURE")
            .expect("set STAMP_LAYOUT_SPAIN_FIXTURE to the Spain album directory"),
    )
    .canonicalize()
    .expect("resolve source album");
    let temp = tempfile::tempdir().expect("temporary fixture workspace");
    let workspace = std::env::var_os("STAMP_LAYOUT_OUTPUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| temp.path().to_path_buf());
    if workspace.exists() {
        assert!(
            std::fs::read_dir(&workspace)
                .expect("inspect output directory")
                .next()
                .is_none(),
            "STAMP_LAYOUT_OUTPUT_DIR must be empty; refusing to overwrite existing files"
        );
    }
    let existing_parent = workspace
        .ancestors()
        .find(|path| path.exists())
        .expect("output workspace must have an existing ancestor")
        .canonicalize()
        .expect("resolve output ancestor");
    assert!(
        !existing_parent.starts_with(&source),
        "output workspace must not be inside the source album"
    );
    let album = workspace.join("Albums").join("Spain");
    std::fs::create_dir_all(&album).expect("create fixture album");
    let workspace = workspace.canonicalize().expect("resolve output workspace");
    let album = workspace.join("Albums").join("Spain");
    assert!(
        !source.starts_with(&workspace),
        "output workspace must not contain the source album"
    );
    let db_path = album.join("Spain.db");
    std::fs::copy(source.join("Spain.db"), &db_path).expect("copy fixture database");
    let image_count =
        copy_directory(&source.join("images"), &album.join("images")).expect("copy fixture images");
    // Use the live layout configuration, but never copy login credentials or
    // runtime paths that could redirect generation back into the source album.
    let root = source
        .parent()
        .and_then(Path::parent)
        .expect("fixture project root");
    std::fs::write(
        workspace.join("config.toml"),
        sanitized_config(&root.join("config.toml")),
    )
    .expect("write current sanitized layout configuration");
    std::fs::write(
        album.join("config.toml"),
        sanitized_config(&source.join("config.toml")),
    )
    .expect("write sanitized album overrides");
    let _cwd = RestoreCwd(std::env::current_dir().expect("original working directory"));
    std::env::set_current_dir(&workspace).expect("enter fixture workspace");
    let output = stamp_album_gen2::config::with_album_config_override_async(&album, || async {
        stamp_album_gen2::album_globals::generate_album_output(
            db_path.to_str().expect("fixture DB path"),
            "Spain",
            None::<fn(usize)>,
            Some(1850),
            Some(1855),
        )
        .await
    })
    .await;
    let generated = std::fs::read_to_string(&output).expect("read fixture output");
    let original =
        std::fs::read_to_string(source.join("Spain-1850-1855.txt")).expect("read reference output");
    let stamp_count = |text: &str| {
        text.lines()
            .filter(|line| line.starts_with("STAMP_ADD_IMG") || line.starts_with("STAMP_ADD ("))
            .count()
    };
    let stamp_paths = |text: &str| {
        text.lines()
            .filter(|line| line.starts_with("STAMP_ADD_IMG"))
            .filter_map(|line| line.split('"').nth(1).map(str::to_string))
            .collect::<Vec<_>>()
    };
    let reference_paths = stamp_paths(&original);
    let generated_paths = stamp_paths(&generated);
    for path in &reference_paths {
        assert!(
            generated_paths.iter().filter(|p| *p == path).count()
                >= reference_paths.iter().filter(|p| *p == path).count(),
            "reference stamp image lost: {path}"
        );
    }
    assert!(
        stamp_count(&generated) >= stamp_count(&original),
        "all reference stamps must remain; newer database stamps are allowed"
    );
    let page_count = generated
        .lines()
        .filter(|line| *line == "PAGE_START")
        .count();
    assert!(
        page_count < 13,
        "expected fewer than the baseline 13 pages; got {page_count}"
    );
    assert!(
        generated
            .split("### Page ")
            .any(|page| page.contains("images/9-9-E2-s.jpg")
                && page.contains("images/12-12-F-s.jpg")),
        "the rows formerly on pages 3 and 4 should share a page"
    );
    for page in generated.split("### Page ").skip(1) {
        if page.contains("Flag ###") {
            continue;
        }
        let height: f64 = page
            .lines()
            .filter_map(|line| {
                line.strip_prefix("# ROW_HEIGHT mm=")
                    .or_else(|| line.strip_prefix("# HEADER_HEIGHT mm="))
                    .and_then(|v| v.parse::<f64>().ok())
                    .or_else(|| {
                        line.strip_prefix("PAGE_VSPACE (")
                            .and_then(|v| v.strip_suffix(')'))
                            .and_then(|v| v.parse::<f64>().ok())
                    })
            })
            .sum();
        assert!(
            height <= 251.71,
            "page exceeds effective usable height: {height}"
        );
    }
    println!(
        "Spain fixture: copied {image_count} images; generated {page_count} pages, {} stamp/cover entries; output {}",
        stamp_count(&generated),
        workspace.join(output).display()
    );
    println!(
        "Database impact: source untouched; fixture copy only; no ingest, inserts, or deletes requested."
    );
}
