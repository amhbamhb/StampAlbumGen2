//! Editable AlbumEasy layouts. The command database, not the copied TXT, is authoritative.

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

mod parser;
mod reflow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Argument {
    pub value: String,
    pub quoted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayoutCommand {
    pub name: String,
    pub args: Vec<Argument>,
    pub source_line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayoutPage {
    pub index: usize,
    pub commands: Vec<LayoutCommand>,
}

#[derive(Debug, Serialize)]
pub struct Inspection {
    pub files: usize,
    pub bytes: u64,
    pub pages: usize,
    pub placements: usize,
    pub unique_images: usize,
    pub commands: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AlbumSummary {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub revision: i64,
}

#[derive(Debug, Serialize)]
pub struct AlbumDetails {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub revision: i64,
    pub pages: Vec<LayoutPage>,
    pub globals: Vec<LayoutCommand>,
    pub warnings: Vec<String>,
    pub reference_pdf: String,
    pub generated_pdf: String,
    pub message: String,
}

type Result<T> = std::result::Result<T, String>;

fn err(context: &str, error: impl std::fmt::Display) -> String {
    format!("{context}: {error}")
}

fn json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| err("Encode layout", e))
}

fn sql_integer(value: usize) -> Result<i64> {
    i64::try_from(value).map_err(|e| err("Layout index exceeds SQLite range", e))
}

fn sql_usize(row: &rusqlite::Row<'_>, column: usize) -> rusqlite::Result<usize> {
    let value: i64 = row.get(column)?;
    usize::try_from(value).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(column, value))
}

fn slug(name: &str) -> Result<String> {
    let slug = name
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    validate_slug(&slug)?;
    Ok(slug)
}

fn validate_slug(name: &str) -> Result<()> {
    if name.is_empty()
        || name.len() > 80
        || !name
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
        || windows_reserved(name)
    {
        return Err(
            "Album name must be a nonempty portable folder slug (maximum 80 characters)".into(),
        );
    }
    Ok(())
}

fn windows_reserved(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or("").to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && matches!(stem.as_bytes()[3], b'1'..=b'9'))
}

fn relative(value: &str) -> Result<PathBuf> {
    if value.is_empty() || value.contains('\0') || value.contains(':') {
        return Err(format!("Unsafe relative path: {value:?}"));
    }
    // Interpret both separators on both platforms, including Windows drive/UNC paths.
    let normalized = value.replace('\\', "/");
    let path = Path::new(&normalized);
    if path.is_absolute()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
    {
        return Err(format!("Unsafe relative path: {value:?}"));
    }
    Ok(path.to_owned())
}

fn portable_path(path: &Path) -> Result<String> {
    let text = path
        .to_str()
        .ok_or_else(|| format!("Non-UTF-8 path: {}", path.display()))?;
    Ok(text.replace('\\', "/"))
}

fn no_links(path: &Path) -> Result<()> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()
            .map_err(|e| err("Resolve current directory", e))?
            .join(path)
    };
    for ancestor in absolute.ancestors() {
        if fs::symlink_metadata(ancestor)
            .map_err(|e| err(&format!("Inspect path {}", ancestor.display()), e))?
            .file_type()
            .is_symlink()
        {
            return Err(format!("Symlinks are not allowed: {}", ancestor.display()));
        }
    }
    Ok(())
}

#[derive(Clone)]
struct Inventory {
    directories: Vec<PathBuf>,
    files: Vec<(PathBuf, u64, Option<String>)>,
}

fn digest(path: &Path) -> Result<String> {
    let mut file = File::open(path).map_err(|e| err("Open asset for checksum", e))?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| err("Read asset checksum", e))?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn inventory(source: &Path) -> Result<Inventory> {
    scan_inventory(source, true)
}

fn scan_inventory(source: &Path, checksum: bool) -> Result<Inventory> {
    let source = source
        .canonicalize()
        .map_err(|e| err("Resolve selected source directory", e))?;
    let source = source.as_path();
    if !source.is_dir() {
        return Err("Source must be a directory".into());
    }
    let mut result = Inventory {
        directories: Vec::new(),
        files: Vec::new(),
    };
    let mut pending = vec![PathBuf::new()];
    let mut names = HashSet::new();
    while let Some(directory) = pending.pop() {
        for entry in
            fs::read_dir(source.join(&directory)).map_err(|e| err("Read source directory", e))?
        {
            let entry = entry.map_err(|e| err("Read source entry", e))?;
            let path = directory.join(entry.file_name());
            let text = portable_path(&path)?;
            let filename = entry.file_name();
            let filename = filename
                .to_str()
                .ok_or("Non-UTF-8 filenames are not portable")?;
            if filename.ends_with(['.', ' '])
                || filename
                    .chars()
                    .any(|c| c.is_control() || "<>:\"\\|?*".contains(c))
                || windows_reserved(filename)
                || !names.insert(text.to_lowercase())
            {
                return Err(format!("Nonportable filename or case collision: {text}"));
            }
            let metadata =
                fs::symlink_metadata(entry.path()).map_err(|e| err("Inspect source entry", e))?;
            if metadata.is_dir() {
                result.directories.push(path.clone());
                pending.push(path);
            } else if metadata.is_file() {
                result.files.push((
                    path,
                    metadata.len(),
                    if checksum {
                        Some(digest(&entry.path())?)
                    } else {
                        None
                    },
                ));
            } else {
                return Err(format!(
                    "Symlinks and special files are not allowed: {text}"
                ));
            }
        }
    }
    result.directories.sort();
    result.files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(result)
}

fn entry_relative(source: &Path, entry: &Path) -> Result<PathBuf> {
    let candidate = if entry.is_absolute() {
        entry
            .strip_prefix(source)
            .map_err(|_| "Entry TXT must be within the source folder")?
            .to_owned()
    } else {
        relative(&portable_path(entry)?)?
    };
    relative(&portable_path(&candidate)?)?;
    if !candidate
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("txt"))
    {
        return Err("Entry must be an AlbumEasy TXT file".into());
    }
    Ok(candidate)
}

fn asset_errors(
    base: &Path,
    globals: &[LayoutCommand],
    pages: &[LayoutPage],
) -> Result<Vec<String>> {
    let mut errors = Vec::new();
    let mut checked = HashMap::new();
    for command in globals.iter().chain(pages.iter().flat_map(|p| &p.commands)) {
        if command.name == "STAMP_ADD_IMG" {
            if command.args.len() < 3 {
                continue;
            }
            let path = relative(&command.args[2].value)?;
            let is_file = if let Some(&exists) = checked.get(&path) {
                exists
            } else {
                let full = base.join(&path);
                if full.exists() {
                    no_links(&full)?;
                }
                let exists = full.is_file();
                checked.insert(path, exists);
                exists
            };
            if !is_file {
                errors.push(format!(
                    "Line {}: missing image {}",
                    command.source_line, command.args[2].value
                ));
            }
        }
    }
    Ok(errors)
}

fn uploaded_image(command: &LayoutCommand) -> Result<Option<PathBuf>> {
    if command.name != "STAMP_ADD_IMG" {
        return Ok(None);
    }
    let path = relative(&command.args[2].value)?;
    if path
        .components()
        .next()
        .is_some_and(|c| c.as_os_str() == "imported-assets")
    {
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

fn uploaded_hash(album: &Path, pages: &[LayoutPage]) -> Result<String> {
    let mut assets = BTreeSet::new();
    if !album.join("imported-assets").exists() {
        return Ok(layout_hash(&json(&assets)?));
    }
    let mut seen = HashSet::new();
    for command in pages.iter().flat_map(|page| &page.commands) {
        if let Some(path) = uploaded_image(command)? {
            if !seen.insert(path.clone()) {
                continue;
            }
            let full = album.join(&path);
            no_links(&full)?;
            if !full.is_file() {
                return Err(format!("Missing uploaded image {}", full.display()));
            }
            assets.insert((portable_path(&path)?, digest(&full)?));
        }
    }
    Ok(layout_hash(&json(&assets)?))
}

fn copy_uploaded_assets(album: &Path, entry_base: &Path, pages: &[LayoutPage]) -> Result<()> {
    let mut referenced = false;
    for command in pages.iter().flat_map(|p| &p.commands) {
        referenced |= uploaded_image(command)?.is_some();
    }
    if !referenced || !album.join("imported-assets").exists() {
        return Ok(());
    }
    let uploads = album.join("imported-assets");
    let target = entry_base.join("imported-assets");
    if target.exists() {
        return Err("Uploaded assets conflict with an original imported-assets folder; this folder name is reserved for uploads".into());
    }
    copy_tree(&uploads, &target, &inventory(&uploads)?)
}

fn editable_asset_errors(
    album: &Path,
    entry: &Path,
    globals: &[LayoutCommand],
    pages: &[LayoutPage],
) -> Result<Vec<String>> {
    let mut errors = Vec::new();
    let source_base = album
        .join("source")
        .join(entry.parent().unwrap_or(Path::new("")));
    for command in globals.iter().chain(pages.iter().flat_map(|p| &p.commands)) {
        let base = if uploaded_image(command)?.is_some() && album.join("imported-assets").exists() {
            album
        } else {
            &source_base
        };
        errors.extend(asset_errors(base, std::slice::from_ref(command), &[])?);
    }
    Ok(errors)
}

pub fn inspect(source: &Path, entry: &Path) -> Result<Inspection> {
    let entry = entry_relative(source, entry)?;
    let source = source
        .canonicalize()
        .map_err(|e| err("Resolve selected source directory", e))?;
    let source = source.as_path();
    let files = scan_inventory(source, false)?;
    let text = fs::read_to_string(source.join(&entry))
        .map_err(|e| err("Read entry TXT (UTF-8 required)", e))?;
    let (globals, pages, mut errors) = parser::parse(&text);
    errors.extend(asset_errors(
        &source.join(entry.parent().unwrap_or(Path::new(""))),
        &globals,
        &pages,
    )?);
    let commands: BTreeSet<_> = globals
        .iter()
        .chain(pages.iter().flat_map(|p| &p.commands))
        .map(|c| c.name.clone())
        .collect();
    let placements: Vec<_> = pages
        .iter()
        .flat_map(|p| &p.commands)
        .filter(|c| c.name == "STAMP_ADD_IMG")
        .collect();
    let images: HashSet<_> = placements
        .iter()
        .filter_map(|c| c.args.get(2))
        .map(|a| a.value.replace('\\', "/"))
        .collect();
    Ok(Inspection {
        files: files.files.len(),
        bytes: files.files.iter().map(|f| f.1).sum(),
        pages: pages.len(),
        placements: placements.len(),
        unique_images: images.len(),
        warnings: if commands.contains("ALBUM_STAMP_IMG_SETTING") {
            vec!["Legacy image padding is preserved in 0.35mm units; it is not converted to the new command.".into()]
        } else {
            vec![]
        },
        commands: commands.into_iter().collect(),
        errors,
    })
}

fn copy_tree(source: &Path, target: &Path, files: &Inventory) -> Result<()> {
    fs::create_dir(target).map_err(|e| err("Create copied source directory", e))?;
    for directory in &files.directories {
        fs::create_dir(target.join(directory)).map_err(|e| err("Copy directory", e))?;
    }
    for (path, size, hash) in &files.files {
        let hash = hash
            .as_ref()
            .ok_or("Copying requires a checksummed inventory")?;
        let original = source.join(path);
        no_links(&original)?;
        let output = target.join(path);
        let mut input = File::open(&original).map_err(|e| err("Open source", e))?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&output)
            .map_err(|e| err("Create asset copy", e))?;
        let copied = std::io::copy(&mut input, &mut file).map_err(|e| err("Copy asset", e))?;
        file.sync_all().map_err(|e| err("Sync asset copy", e))?;
        if copied != *size || digest(&output)? != *hash {
            return Err(format!(
                "Source changed during copy: {}",
                original.display()
            ));
        }
    }
    Ok(())
}

fn schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA foreign_keys=ON;
         CREATE TABLE IF NOT EXISTS external_album (
           id INTEGER PRIMARY KEY CHECK(id=1), display_name TEXT NOT NULL,
           status TEXT NOT NULL CHECK(status IN ('draft','validated','ready')),
           revision INTEGER NOT NULL, entry TEXT NOT NULL, globals TEXT NOT NULL,
           warnings TEXT NOT NULL, message TEXT NOT NULL DEFAULT '',
           reference_pdf TEXT NOT NULL DEFAULT '', generated_pdf TEXT NOT NULL DEFAULT '',
           reference_hash TEXT NOT NULL DEFAULT '', generated_hash TEXT NOT NULL DEFAULT '',
           layout_hash TEXT NOT NULL DEFAULT '', uploaded_hash TEXT NOT NULL DEFAULT '');
         CREATE TABLE IF NOT EXISTS external_layout_commands (
           id INTEGER PRIMARY KEY, page_index INTEGER NOT NULL, ordinal INTEGER NOT NULL,
           name TEXT NOT NULL, args TEXT NOT NULL, source_line INTEGER NOT NULL,
           UNIQUE(page_index,ordinal));
         CREATE TABLE IF NOT EXISTS external_layout_placements (
           command_id INTEGER PRIMARY KEY REFERENCES external_layout_commands(id) ON DELETE CASCADE,
           stamp_id INTEGER NOT NULL REFERENCES stamps(id), image_path TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS external_layout_assets (
           path TEXT PRIMARY KEY, bytes INTEGER NOT NULL, sha256 TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS external_layout_sources (
           path TEXT PRIMARY KEY, content TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS external_layout_compatibility (
           source_line INTEGER NOT NULL, severity TEXT NOT NULL, message TEXT NOT NULL);",
    )
    .map_err(|e| err("Create additive external layout schema", e))?;
    let has_upload_hash: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM pragma_table_info('external_album') WHERE name='uploaded_hash')",
        [], |r| r.get(0),
    ).map_err(|e| err("Inspect uploaded image metadata migration", e))?;
    if !has_upload_hash {
        conn.execute_batch(
            "ALTER TABLE external_album ADD COLUMN uploaded_hash TEXT NOT NULL DEFAULT '';",
        )
        .map_err(|e| err("Add uploaded image checksum metadata", e))?;
    }
    let zero_based: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM external_layout_commands WHERE page_index=0)",
            [],
            |r| r.get(0),
        )
        .map_err(|e| err("Inspect legacy page numbering", e))?;
    if zero_based {
        // Two phases avoid page/ordinal uniqueness collisions while retaining row identities.
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| err("Begin page numbering migration", e))?;
        tx.execute_batch(
            "UPDATE external_layout_commands SET page_index=-page_index-1;
             UPDATE external_layout_commands SET page_index=-page_index;",
        )
        .map_err(|e| err("Migrate legacy pages to one-based indices", e))?;
        tx.commit()
            .map_err(|e| err("Commit page numbering migration", e))?;
    }
    Ok(())
}

pub fn is_imported(conn: &Connection) -> Result<bool> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='external_album')",
        [], |r| r.get(0),
    ).map_err(|e| err("Inspect imported album schema", e))?;
    if !exists {
        return Ok(false);
    }
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM external_album WHERE id=1)",
        [],
        |r| r.get(0),
    )
    .map_err(|e| err("Inspect imported album metadata", e))
}

fn paths(root: &Path, name: &str) -> Result<(PathBuf, PathBuf)> {
    validate_slug(name)?;
    let root = root
        .canonicalize()
        .map_err(|e| err("Resolve selected album root", e))?;
    let album = root.join(name);
    no_links(&album)?;
    let db = album.join(format!("{name}.db"));
    no_links(&db)?;
    Ok((album, db))
}

fn open(root: &Path, name: &str) -> Result<(PathBuf, Connection)> {
    let (album, db) = paths(root, name)?;
    let conn = Connection::open_with_flags(db, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(|e| err("Open imported album", e))?;
    if !is_imported(&conn)? {
        return Err("Not an imported layout album".into());
    }
    schema(&conn)?;
    Ok((album, conn))
}

fn details(conn: &Connection, name: &str) -> Result<AlbumDetails> {
    let zero_based: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM external_layout_commands WHERE page_index=0)",
            [],
            |r| r.get(0),
        )
        .map_err(|e| err("Read public page numbering", e))?;
    let mut result = conn.query_row(
        "SELECT display_name,status,revision,globals,warnings,reference_pdf,generated_pdf,message
         FROM external_album WHERE id=1", [], |row| {
            Ok(AlbumDetails {
                name: name.into(), display_name: row.get(0)?, status: row.get(1)?, revision: row.get(2)?,
                globals: serde_json::from_str(&row.get::<_, String>(3)?).map_err(|e|
                    rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?,
                warnings: serde_json::from_str(&row.get::<_, String>(4)?).map_err(|e|
                    rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?,
                pages: vec![], reference_pdf: row.get(5)?, generated_pdf: row.get(6)?, message: row.get(7)?,
            })
        },
    ).map_err(|e| err("Read external album", e))?;
    let mut stmt = conn.prepare("SELECT page_index,name,args,source_line FROM external_layout_commands ORDER BY page_index,ordinal")
        .map_err(|e| err("Read layout commands", e))?;
    let commands = stmt
        .query_map([], |r| {
            Ok((
                sql_usize(r, 0)?,
                LayoutCommand {
                    name: r.get(1)?,
                    args: serde_json::from_str(&r.get::<_, String>(2)?).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?,
                    source_line: sql_usize(r, 3)?,
                },
            ))
        })
        .map_err(|e| err("Query layout commands", e))?;
    for command in commands {
        let (index, command) = command.map_err(|e| err("Decode layout command", e))?;
        let index = if zero_based {
            index.checked_add(1).ok_or("Page index overflow")?
        } else {
            index
        };
        if result.pages.last().is_none_or(|p| p.index != index) {
            result.pages.push(LayoutPage {
                index,
                commands: vec![],
            });
        }
        result
            .pages
            .last_mut()
            .ok_or("Layout page missing")?
            .commands
            .push(command);
    }
    parser::validate_document(&result.globals, &result.pages)?;
    Ok(result)
}

pub fn load_album(root: &Path, name: &str) -> Result<AlbumDetails> {
    let (_, conn) = open(root, name)?;
    details(&conn, name)
}

pub fn asset_directory(root: &Path, name: &str) -> Result<PathBuf> {
    let (album, _) = open(root, name)?;
    let directory = album.join("imported-assets");
    match fs::symlink_metadata(&directory) {
        Ok(_) => no_links(&directory)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(&directory).map_err(|e| err("Create uploaded asset directory", e))?;
        }
        Err(e) => return Err(err("Inspect uploaded asset directory", e)),
    }
    no_links(&directory)?;
    if !directory.is_dir() {
        return Err("Uploaded asset path is not a directory".into());
    }
    Ok(directory)
}

pub fn list_albums(root: &Path) -> Result<Vec<AlbumSummary>> {
    let root = root
        .canonicalize()
        .map_err(|e| err("Resolve selected album root", e))?;
    let mut result = Vec::new();
    for entry in fs::read_dir(&root).map_err(|e| err("List album root", e))? {
        let entry = entry.map_err(|e| err("Read album listing", e))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or("Non-UTF-8 album folder")?
            .to_owned();
        if validate_slug(&name).is_err()
            || !entry
                .file_type()
                .map_err(|e| err("Inspect album folder", e))?
                .is_dir()
        {
            continue;
        }
        let db = entry.path().join(format!("{name}.db"));
        if !db.exists() {
            continue;
        }
        no_links(&db)?;
        let conn = Connection::open_with_flags(db, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|e| err("Inspect album database", e))?;
        if is_imported(&conn)? {
            let album = details(&conn, &name)?;
            result.push(AlbumSummary {
                name,
                display_name: album.display_name,
                status: album.status,
                revision: album.revision,
            });
        }
    }
    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}

fn persist_pages(conn: &Connection, pages: &[LayoutPage], selected: Option<usize>) -> Result<()> {
    for page in pages
        .iter()
        .filter(|p| selected.is_none_or(|i| p.index == i))
    {
        let page_index = sql_integer(page.index)?;
        let old_commands: Vec<(i64, LayoutCommand, Option<i64>)> = {
            let mut stmt = conn.prepare("SELECT c.id,c.name,c.args,c.source_line,p.stamp_id FROM external_layout_commands c LEFT JOIN external_layout_placements p ON p.command_id=c.id WHERE c.page_index=? ORDER BY c.ordinal")
                .map_err(|e| err("Read existing page identities", e))?;
            let rows = stmt
                .query_map([page_index], |r| {
                    let args = serde_json::from_str(&r.get::<_, String>(2)?).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;
                    Ok((
                        r.get(0)?,
                        LayoutCommand {
                            name: r.get(1)?,
                            args,
                            source_line: sql_usize(r, 3)?,
                        },
                        r.get(4)?,
                    ))
                })
                .map_err(|e| err("Query existing page identities", e))?;
            rows.collect::<rusqlite::Result<Vec<_>>>()
                .map_err(|e| err("Decode existing page identities", e))?
        };
        let mut next_id: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(id),0) FROM external_layout_commands",
                [],
                |r| r.get(0),
            )
            .map_err(|e| err("Allocate layout identity", e))?;
        let mut reused = HashSet::new();
        conn.execute(
            "DELETE FROM external_layout_commands WHERE page_index=?",
            [page_index],
        )
        .map_err(|e| err("Replace page commands", e))?;
        for (ordinal, command) in page.commands.iter().enumerate() {
            // Original source locations identify editable placements; new commands use line zero.
            // Exact matching also keeps zero-line commands stable on subsequent saves.
            let existing = old_commands
                .iter()
                .find(|(id, old, _)| !reused.contains(id) && old == command)
                .or_else(|| {
                    old_commands.iter().find(|(id, old, _)| {
                        !reused.contains(id)
                            && command.source_line != 0
                            && old.source_line == command.source_line
                            && old.name == command.name
                    })
                });
            let (command_id, existing_stamp) = if let Some((id, _, stamp)) = existing {
                reused.insert(*id);
                (*id, *stamp)
            } else {
                next_id = next_id.checked_add(1).ok_or("Layout identity exhausted")?;
                (next_id, None)
            };
            conn.execute("INSERT INTO external_layout_commands(id,page_index,ordinal,name,args,source_line) VALUES(?,?,?,?,?,?)",
                params![command_id, page_index, sql_integer(ordinal)?, command.name, json(&command.args)?, sql_integer(command.source_line)?])
                .map_err(|e| err("Save layout command", e))?;
            if command.name == "STAMP_ADD_IMG" {
                // Each placement is distinct, even when image paths repeat. Box sizes are not physical stamp dimensions.
                let stamp_id = if let Some(id) = existing_stamp {
                    conn.execute(
                        "UPDATE stamps SET image_path=?,description=? WHERE id=?",
                        params![command.args[2].value, command.args[4].value, id],
                    )
                    .map_err(|e| err("Update existing imported placement", e))?;
                    id
                } else {
                    conn.execute(
                        "INSERT INTO stamps(image_path,description,type,uniq_key) VALUES(?,?,'external-layout',?)",
                        params![command.args[2].value, command.args[4].value, format!("external-layout-command:{command_id}")],
                    ).map_err(|e| err("Save imported stamp placement", e))?;
                    conn.last_insert_rowid()
                };
                conn.execute("INSERT INTO external_layout_placements(command_id,stamp_id,image_path) VALUES(?,?,?)",
                    params![command_id, stamp_id, command.args[2].value])
                    .map_err(|e| err("Link imported placement", e))?;
            }
        }
        for (_, _, id) in old_commands {
            if let Some(id) = id {
                conn.execute("DELETE FROM stamps WHERE id=? AND NOT EXISTS(SELECT 1 FROM external_layout_placements WHERE stamp_id=stamps.id)", [id])
                    .map_err(|e| err("Remove deleted imported placement", e))?;
            }
        }
    }
    Ok(())
}

pub fn import_album(
    root: &Path,
    name: &str,
    source: &Path,
    entry: &Path,
    cli: &str,
    timeout: Duration,
) -> Result<AlbumDetails> {
    let entry = entry_relative(source, entry)?;
    let source = source
        .canonicalize()
        .map_err(|e| err("Resolve selected source directory", e))?;
    let source = source.as_path();
    let entry = entry.as_path();
    if root.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("Album root must not contain parent traversal".into());
    }
    let name_slug = slug(name)?;
    let inspection = inspect(source, entry)?;
    if !inspection.errors.is_empty() {
        return Err(inspection.errors.join("\n"));
    }
    let absolute_root = if root.is_absolute() {
        root.to_owned()
    } else {
        std::env::current_dir()
            .map_err(|e| err("Resolve album root", e))?
            .join(root)
    };
    let mut existing = absolute_root.as_path();
    while !existing.exists() {
        existing = existing
            .parent()
            .ok_or("Album root has no existing ancestor")?;
    }
    let canonical_root = existing
        .canonicalize()
        .map_err(|e| err("Resolve album root", e))?
        .join(
            absolute_root
                .strip_prefix(existing)
                .map_err(|e| err("Resolve album root", e))?,
        );
    let canonical_source = source
        .canonicalize()
        .map_err(|e| err("Resolve source folder", e))?;
    let destination = canonical_root.join(&name_slug);
    if destination.starts_with(&canonical_source) || canonical_source.starts_with(&destination) {
        return Err("Source and album destination must not contain one another".into());
    }
    fs::create_dir_all(root).map_err(|e| err("Create album root", e))?;
    let root = root
        .canonicalize()
        .map_err(|e| err("Resolve selected album root", e))?;
    let root = root.as_path();
    for existing in fs::read_dir(root).map_err(|e| err("Check destination collision", e))? {
        if existing
            .map_err(|e| err("Read destination entry", e))?
            .file_name()
            .to_string_lossy()
            .eq_ignore_ascii_case(&name_slug)
        {
            return Err(format!("Album destination already exists: {name_slug}"));
        }
    }
    let entry = entry_relative(source, entry)?;
    let files = inventory(source)?;
    let text = fs::read_to_string(source.join(&entry)).map_err(|e| err("Read entry source", e))?;
    let (globals, pages, errors) = parser::parse(&text);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    let stage = tempfile::Builder::new()
        .prefix(".external-import-")
        .tempdir_in(root)
        .map_err(|e| err("Create import staging folder", e))?;
    copy_tree(source, &stage.path().join("source"), &files)?;
    let db = stage.path().join(format!("{name_slug}.db"));
    let conn = crate::db_utils::init_db(db.to_str().ok_or("Non-UTF-8 database path")?)
        .map_err(|e| err("Initialize imported stamp database", e))?;
    schema(&conn)?;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| err("Begin import transaction", e))?;
    tx.execute("INSERT INTO external_album(id,display_name,status,revision,entry,globals,warnings,message) VALUES(1,?,'draft',1,?,?,?,'Awaiting successful reference and reconstructed renders')",
        params![name.trim(), portable_path(&entry)?, json(&globals)?, json(&inspection.warnings)?])
        .map_err(|e| err("Save imported album metadata", e))?;
    persist_pages(&tx, &pages, None)?;
    for (path, bytes, hash) in &files.files {
        let hash = hash
            .as_ref()
            .ok_or("Import persistence requires asset checksums")?;
        tx.execute(
            "INSERT INTO external_layout_assets(path,bytes,sha256) VALUES(?,?,?)",
            params![
                portable_path(path)?,
                i64::try_from(*bytes).map_err(|e| err("Asset size exceeds SQLite range", e))?,
                hash
            ],
        )
        .map_err(|e| err("Save source inventory", e))?;
    }
    tx.execute(
        "INSERT INTO external_layout_sources(path,content) VALUES(?,?)",
        params![portable_path(&entry)?, text],
    )
    .map_err(|e| err("Save source snapshot", e))?;
    for warning in &inspection.warnings {
        tx.execute("INSERT INTO external_layout_compatibility(source_line,severity,message) VALUES(0,'warning',?)", [warning])
            .map_err(|e| err("Save compatibility record", e))?;
    }
    tx.commit().map_err(|e| err("Commit import database", e))?;
    let current = details(&conn, &name_slug)?;
    match render_pair(stage.path(), &conn, &current, cli, timeout) {
        Ok(rendered) => {
            set_rendered(&conn, &rendered)?;
            let _ = rendered.workspace.keep();
        }
        Err(message) => {
            log::warn!("External album {name_slug} remains draft: {message}");
            conn.execute("UPDATE external_album SET message=? WHERE id=1", [message])
                .map_err(|e| err("Save draft render failure", e))?;
        }
    }
    drop(conn);
    // Keep TempDir cleanup armed until publication succeeds.
    fs::rename(stage.path(), root.join(&name_slug))
        .map_err(|e| err("Publish imported draft", e))?;
    load_album(root, &name_slug)
}

struct Rendered {
    reference: String,
    generated: String,
    reference_hash: String,
    generated_hash: String,
    layout_hash: String,
    uploaded_hash: String,
    workspace: tempfile::TempDir,
}

fn render_pair(
    album: &Path,
    conn: &Connection,
    current: &AlbumDetails,
    cli: &str,
    timeout: Duration,
) -> Result<Rendered> {
    let entry: String = conn
        .query_row("SELECT entry FROM external_album WHERE id=1", [], |r| {
            r.get(0)
        })
        .map_err(|e| err("Read entry path", e))?;
    let entry = relative(&entry)?;
    let source = album.join("source");
    verify_source(conn, &source)?;
    let files = inventory(&source)?;
    let stage = tempfile::Builder::new()
        .prefix("render-")
        .tempdir_in(album)
        .map_err(|e| err("Create rendering workspace", e))?;
    copy_tree(&source, &stage.path().join("reference"), &files)?;
    copy_tree(&source, &stage.path().join("reconstructed"), &files)?;
    let reference = stage.path().join("reference").join(&entry);
    let generated = stage.path().join("reconstructed").join(&entry);
    let uploaded_hash = uploaded_hash(album, &current.pages)?;
    copy_uploaded_assets(
        album,
        generated
            .parent()
            .ok_or("Missing reconstructed entry directory")?,
        &current.pages,
    )?;
    let source_text: String = conn
        .query_row(
            "SELECT content FROM external_layout_sources WHERE path=?",
            [portable_path(&entry)?],
            |r| r.get(0),
        )
        .map_err(|e| err("Load reference source snapshot", e))?;
    fs::write(&reference, source_text).map_err(|e| err("Write reference render input", e))?;
    let layout = parser::serialize(&current.globals, &current.pages)?;
    fs::write(&generated, &layout).map_err(|e| err("Write reconstructed render input", e))?;
    for base in [generated.parent()].into_iter().flatten() {
        let errors = asset_errors(base, &current.globals, &current.pages)?;
        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }
    }
    render(&reference, cli, timeout)?;
    render(&generated, cli, timeout)?;
    let reference = reference.with_extension("pdf");
    let generated = generated.with_extension("pdf");
    let result = Rendered {
        reference: portable_path(
            reference
                .strip_prefix(album)
                .map_err(|e| err("Reference PDF path", e))?,
        )?,
        generated: portable_path(
            generated
                .strip_prefix(album)
                .map_err(|e| err("Reconstructed PDF path", e))?,
        )?,
        reference_hash: digest(&reference)?,
        generated_hash: digest(&generated)?,
        layout_hash: layout_hash(&layout),
        uploaded_hash,
        workspace: stage,
    };
    Ok(result)
}

fn verify_source(conn: &Connection, source: &Path) -> Result<()> {
    let mut stmt = conn
        .prepare("SELECT path,bytes,sha256 FROM external_layout_assets")
        .map_err(|e| err("Read source inventory", e))?;
    let assets = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| err("Query source inventory", e))?;
    for asset in assets {
        let (path, bytes, hash) = asset.map_err(|e| err("Decode source inventory", e))?;
        let path = source.join(relative(&path)?);
        no_links(&path)?;
        let bytes = u64::try_from(bytes).map_err(|e| err("Invalid stored asset size", e))?;
        if fs::metadata(&path)
            .map_err(|e| err("Inspect preserved asset", e))?
            .len()
            != bytes
            || digest(&path)? != hash
        {
            return Err(format!(
                "Preserved source asset changed: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn pdf_valid(path: &Path) -> Result<()> {
    if !path.is_file() {
        return Err(format!("Fresh PDF missing: {}", path.display()));
    }
    no_links(path)?;
    let mut bytes = [0u8; 5];
    File::open(path)
        .map_err(|e| err("Fresh PDF missing", e))?
        .read_exact(&mut bytes)
        .map_err(|e| err("Fresh PDF is empty or truncated", e))?;
    if &bytes != b"%PDF-" {
        return Err(format!("Renderer output is not a PDF: {}", path.display()));
    }
    Ok(())
}

fn render(input: &Path, cli: &str, timeout: Duration) -> Result<()> {
    if cli.trim().is_empty() {
        return Err("AlbumEasy executable is required".into());
    }
    if timeout.is_zero() {
        return Err("Renderer timeout must be positive".into());
    }
    let pdf = input.with_extension("pdf");
    if pdf.exists() {
        no_links(&pdf)?;
        fs::remove_file(&pdf)
            .map_err(|e| err("Remove copied stale PDF from rendering workspace", e))?;
    }
    let parent = input.parent().ok_or("Renderer input has no parent")?;
    // Files, not pipes, prevent full pipe buffers or inherited handles from defeating the deadline.
    let stdout = tempfile::tempfile().map_err(|e| err("Create renderer stdout log", e))?;
    let stderr = tempfile::tempfile().map_err(|e| err("Create renderer stderr log", e))?;
    let read_stdout = stdout
        .try_clone()
        .map_err(|e| err("Clone renderer stdout log", e))?;
    let read_stderr = stderr
        .try_clone()
        .map_err(|e| err("Clone renderer stderr log", e))?;
    let executable = if Path::new(cli).components().count() > 1 {
        Path::new(cli)
            .canonicalize()
            .map_err(|e| err("Resolve AlbumEasy executable", e))?
    } else {
        PathBuf::from(cli)
    };
    let mut child = Command::new(executable)
        .arg("-c")
        .arg(input.file_name().ok_or("Missing entry filename")?)
        .current_dir(parent)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .map_err(|e| err("Launch AlbumEasy", e))?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() < timeout => std::thread::sleep(
                Duration::from_millis(20).min(timeout.saturating_sub(started.elapsed())),
            ),
            outcome => {
                let reason = match outcome {
                    Err(e) => err("Wait for AlbumEasy", e),
                    _ => format!(
                        "AlbumEasy renderer timed out after {}ms",
                        timeout.as_millis()
                    ),
                };
                child.kill().map_err(|e| err(&reason, e))?;
                child
                    .wait()
                    .map_err(|e| err("Reap timed-out AlbumEasy", e))?;
                return Err(reason);
            }
        }
    };
    use std::io::{Seek, SeekFrom};
    let mut diagnostics = String::new();
    for mut file in [read_stdout, read_stderr] {
        if file
            .metadata()
            .map_err(|e| err("Inspect renderer diagnostics", e))?
            .len()
            > 1024 * 1024
        {
            return Err("Renderer diagnostics exceeded 1 MiB; validation is blocked rather than truncating errors".into());
        }
        file.seek(SeekFrom::Start(0))
            .map_err(|e| err("Seek renderer diagnostics", e))?;
        let mut bytes = Vec::new();
        file.take(1024 * 1024)
            .read_to_end(&mut bytes)
            .map_err(|e| err("Read renderer diagnostics", e))?;
        diagnostics.push_str(&String::from_utf8_lossy(&bytes));
        diagnostics.push('\n');
    }
    let reported_error = diagnostics.lines().any(|line| {
        let line = line.trim().to_ascii_lowercase();
        line.starts_with("error:")
            || line.starts_with("error in line ")
            || line.starts_with("fatal")
    });
    if !status.success() || reported_error {
        return Err(format!(
            "AlbumEasy render failed ({status}): {}",
            diagnostics.trim()
        ));
    }
    pdf_valid(&pdf).map_err(|e| format!("{e}\n{}", diagnostics.trim()))
}

fn layout_hash(layout: &str) -> String {
    Sha256::digest(layout.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn verify_layout(conn: &Connection, current: &AlbumDetails) -> Result<()> {
    let saved: String = conn
        .query_row(
            "SELECT layout_hash FROM external_album WHERE id=1",
            [],
            |r| r.get(0),
        )
        .map_err(|e| err("Read validated layout checksum", e))?;
    if saved != layout_hash(&parser::serialize(&current.globals, &current.pages)?) {
        return Err(
            "Saved layout changed since validation; retry rendering and approve fidelity again"
                .into(),
        );
    }
    Ok(())
}

fn set_rendered(conn: &Connection, rendered: &Rendered) -> Result<()> {
    conn.execute("UPDATE external_album SET status='validated',message='Awaiting user fidelity approval',reference_pdf=?,generated_pdf=?,reference_hash=?,generated_hash=?,layout_hash=?,uploaded_hash=? WHERE id=1",
        params![rendered.reference, rendered.generated, rendered.reference_hash, rendered.generated_hash, rendered.layout_hash, rendered.uploaded_hash])
        .map_err(|e| err("Save render validation", e))?;
    Ok(())
}

fn verify_uploaded(conn: &Connection, album: &Path, current: &AlbumDetails) -> Result<()> {
    let saved: String = conn
        .query_row(
            "SELECT uploaded_hash FROM external_album WHERE id=1",
            [],
            |r| r.get(0),
        )
        .map_err(|e| err("Read uploaded image validation checksum", e))?;
    if saved != uploaded_hash(album, &current.pages)? {
        return Err(
            "Uploaded images changed since validation; retry rendering before approval or export"
                .into(),
        );
    }
    Ok(())
}

pub fn retry_album(root: &Path, name: &str, cli: &str, timeout: Duration) -> Result<AlbumDetails> {
    let (album, conn) = open(root, name)?;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| err("Lock album for retry", e))?;
    let current = details(&tx, name)?;
    let rendered = match render_pair(&album, &tx, &current, cli, timeout) {
        Ok(rendered) => {
            tx.execute(
                "UPDATE external_album SET revision=revision+1 WHERE id=1",
                [],
            )
            .map_err(|e| err("Invalidate prior approval", e))?;
            set_rendered(&tx, &rendered)?;
            Some(rendered)
        }
        Err(message) => {
            log::warn!("External album retry failed: {message}");
            tx.execute("UPDATE external_album SET status='draft',revision=revision+1,message=?,reference_pdf='',generated_pdf='',reference_hash='',generated_hash='' WHERE id=1", [message])
                .map_err(|e| err("Save retry failure", e))?;
            None
        }
    };
    tx.commit().map_err(|e| err("Commit retry", e))?;
    if let Some(rendered) = rendered {
        let _ = rendered.workspace.keep();
    }
    details(&conn, name)
}

pub fn approve_album(root: &Path, name: &str, revision: i64) -> Result<AlbumDetails> {
    let (album, conn) = open(root, name)?;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| err("Begin approval", e))?;
    let current = details(&tx, name)?;
    if current.revision != revision {
        return Err("Revision conflict: reload the album before approving".into());
    }
    if current.status != "validated" {
        return Err("Approval requires successful reference and reconstructed renders".into());
    }
    verify_layout(&tx, &current)?;
    verify_uploaded(&tx, &album, &current)?;
    verify_source(&tx, &album.join("source"))?;
    let hashes: (String, String) = tx
        .query_row(
            "SELECT reference_hash,generated_hash FROM external_album WHERE id=1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| err("Read validated PDF checksums", e))?;
    for (path, hash) in [
        (&current.reference_pdf, hashes.0),
        (&current.generated_pdf, hashes.1),
    ] {
        let path = album.join(relative(path)?);
        pdf_valid(&path)?;
        if digest(&path)? != hash {
            return Err("Validated PDF has changed; retry validation before approval".into());
        }
    }
    tx.execute(
        "UPDATE external_album SET status='ready',message='',revision=revision+1 WHERE id=1",
        [],
    )
    .map_err(|e| err("Approve imported album", e))?;
    tx.commit().map_err(|e| err("Commit approval", e))?;
    details(&conn, name)
}

pub fn edit_page(
    root: &Path,
    name: &str,
    page: usize,
    commands: Vec<LayoutCommand>,
    revision: i64,
    cli: &str,
    timeout: Duration,
) -> Result<AlbumDetails> {
    let (album, conn) = open(root, name)?;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| err("Begin page edit", e))?;
    let mut current = details(&tx, name)?;
    if current.revision != revision {
        return Err("Revision conflict: reload the album before editing".into());
    }
    let approved_reference = if current.status == "ready" {
        let hash: String = tx
            .query_row(
                "SELECT reference_hash FROM external_album WHERE id=1",
                [],
                |r| r.get(0),
            )
            .map_err(|e| err("Read approved reference checksum", e))?;
        let path = album.join(relative(&current.reference_pdf)?);
        pdf_valid(&path)?;
        if digest(&path)? != hash {
            return Err(
                "Approved reference PDF changed; restore or revalidate it before editing".into(),
            );
        }
        Some((current.reference_pdf.clone(), hash))
    } else {
        None
    };
    if page == 0 {
        current.globals = commands;
        parser::validate_document(&current.globals, &current.pages)?;
        // Global geometry affects every page; existing page boundaries remain fixed.
        current.pages = current
            .pages
            .into_iter()
            .map(|candidate| reflow::reflow(&current.globals, candidate))
            .collect::<Result<Vec<_>>>()?;
    } else {
        let index = current
            .pages
            .iter()
            .position(|p| p.index == page)
            .ok_or("Page does not exist")?;
        let candidate = LayoutPage {
            index: page,
            commands,
        };
        parser::validate_page(&candidate)?;
        current.pages[index] = reflow::reflow(&current.globals, candidate)?;
    }
    parser::validate_document(&current.globals, &current.pages)?;
    let errors =
        editable_asset_errors(&album, &read_entry(&tx)?, &current.globals, &current.pages)?;
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    current.revision += 1;
    let rendered = render_pair(&album, &tx, &current, cli, timeout);
    persist_pages(
        &tx,
        &current.pages,
        if page == 0 { None } else { Some(page) },
    )?;
    tx.execute(
        "UPDATE external_album SET revision=?,globals=? WHERE id=1",
        params![current.revision, json(&current.globals)?],
    )
    .map_err(|e| err("Save edit revision", e))?;
    let rendered = match rendered {
        Ok(rendered) => {
            set_rendered(&tx, &rendered)?;
            if let Some((reference, hash)) = &approved_reference {
                tx.execute(
                    "UPDATE external_album SET status='ready',message='',reference_pdf=?,reference_hash=? WHERE id=1",
                    params![reference, hash],
                ).map_err(|e| err("Preserve approved album state after intentional edit", e))?;
            }
            Some(rendered)
        }
        Err(message) => {
            log::warn!("Saved external album edit remains draft: {message}");
            tx.execute("UPDATE external_album SET status='draft',message=?,reference_pdf='',generated_pdf='',reference_hash='',generated_hash='',layout_hash='' WHERE id=1", [message])
                .map_err(|e| err("Save edited draft rendering failure", e))?;
            None
        }
    };
    tx.commit().map_err(|e| err("Commit page edit", e))?;
    if let Some(rendered) = rendered {
        let _ = rendered.workspace.keep();
    }
    details(&conn, name)
}

fn read_entry(conn: &Connection) -> Result<PathBuf> {
    let entry: String = conn
        .query_row("SELECT entry FROM external_album WHERE id=1", [], |r| {
            r.get(0)
        })
        .map_err(|e| err("Read entry path", e))?;
    relative(&entry)
}

/// Export the saved layout with its assets; never invoke the ordinary country generator.
pub fn export_layout(conn: &Connection) -> Result<PathBuf> {
    if !is_imported(conn)? {
        return Err("Not an imported layout database".into());
    }
    let db: String = conn
        .query_row(
            "SELECT file FROM pragma_database_list WHERE name='main'",
            [],
            |r| r.get(0),
        )
        .map_err(|e| err("Locate imported database", e))?;
    let album = Path::new(&db)
        .parent()
        .ok_or("Imported database has no album directory")?;
    let album = album
        .canonicalize()
        .map_err(|e| err("Resolve imported database directory", e))?;
    let album = album.as_path();
    no_links(album)?;
    let current = details(conn, "export")?;
    if current.status != "ready" {
        return Err(
            "Imported album is not ready; validate both PDFs and approve fidelity first".into(),
        );
    }
    verify_layout(conn, &current)?;
    verify_uploaded(conn, album, &current)?;
    let source = album.join("source");
    verify_source(conn, &source)?;
    let stage = tempfile::Builder::new()
        .prefix("export-")
        .tempdir_in(album)
        .map_err(|e| err("Create export workspace", e))?;
    let target = stage.path().join("layout");
    copy_tree(&source, &target, &inventory(&source)?)?;
    let output = target.join(read_entry(conn)?);
    copy_uploaded_assets(
        album,
        output.parent().ok_or("Missing export entry directory")?,
        &current.pages,
    )?;
    fs::write(
        &output,
        parser::serialize(&current.globals, &current.pages)?,
    )
    .map_err(|e| err("Export saved layout", e))?;
    let stale_pdf = output.with_extension("pdf");
    if stale_pdf.exists() {
        fs::remove_file(&stale_pdf).map_err(|e| err("Remove copied stale export PDF", e))?;
    }
    let _ = stage.keep();
    Ok(output)
}

pub(crate) fn generate_artifacts(
    conn: &Connection,
    pdf_override: Option<bool>,
) -> Result<crate::album_globals::GeneratedAlbumArtifacts> {
    let output = export_layout(conn)?;
    let cfg = crate::config::load_effective_config("config.toml");
    let generate_pdf = pdf_override.unwrap_or_else(|| cfg.albumeasy_auto_pdf_enabled());
    let mut result = crate::album_globals::GeneratedAlbumArtifacts {
        output_txt_path: output.clone(),
        generated_pdf_path: None,
        pdf_failure_reason: None,
    };
    if generate_pdf {
        match cfg.effective_albumeasy_cli_path() {
            Some(cli) => match render(
                &output,
                cli,
                Duration::from_secs(u64::from(cfg.albumeasy_pdf_timeout_secs())),
            ) {
                Ok(()) => result.generated_pdf_path = Some(output.with_extension("pdf")),
                Err(e) => result.pdf_failure_reason = Some(e),
            },
            None => {
                result.pdf_failure_reason = Some("AlbumEasy executable is not configured".into())
            }
        }
    }
    Ok(result)
}
