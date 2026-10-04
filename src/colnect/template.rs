use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub const TEMPLATE_CSV: &str = include_str!("manual_import_template.csv");
pub const TEMPLATE_GUIDE: &str = include_str!("../../docs/guides/USER_DEFINED_CSV_IMPORT.md");

/// Create missing manual-import files under the runtime folder, preserving existing copies.
pub fn ensure_manual_import_files(runtime_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let directory = runtime_dir.join("UserImport");
    std::fs::create_dir_all(&directory)?;
    let mut created = Vec::new();
    for (name, contents) in [
        ("manual_import_template.csv", TEMPLATE_CSV),
        ("README.md", TEMPLATE_GUIDE),
    ] {
        let path = directory.join(name);
        match std::fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() => continue,
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Manual import file path is not a file: {}", path.display()),
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }

        // Publish only complete files, without overwriting a concurrent launch's copy.
        let mut temporary = tempfile::NamedTempFile::new_in(&directory)?;
        temporary.write_all(contents.as_bytes())?;
        temporary.flush()?;
        match temporary.persist_noclobber(&path) {
            Ok(_) => created.push(path),
            Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists && path.is_file() => {}
            Err(error) => return Err(error.error),
        }
    }
    Ok(created)
}
