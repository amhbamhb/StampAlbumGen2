use super::*;
use crate::external_album;
use serde_json::{Value, json};
use std::io::Read;
use std::sync::atomic::AtomicU64;

const PREFIX: &str = "/api/external-album/";
static NEXT_JOB: AtomicU64 = AtomicU64::new(1);
static JOB: OnceLock<Mutex<Option<ImportJob>>> = OnceLock::new();

#[derive(Clone, Serialize)]
struct ImportJob {
    id: u64,
    running: bool,
    result: Option<Value>,
    error: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceRequest {
    source: String,
    entry: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ImportRequest {
    source: String,
    entry: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AlbumRequest {
    name: String,
    revision: Option<i64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EditRequest {
    name: String,
    page: usize,
    commands: Vec<external_album::LayoutCommand>,
    revision: i64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageRequest {
    name: String,
    data_url: String,
}

fn body<T: serde::de::DeserializeOwned>(request: &mut tiny_http::Request) -> Result<T, String> {
    let mut text = String::new();
    request
        .as_reader()
        .take(8_000_001)
        .read_to_string(&mut text)
        .map_err(|e| format!("Cannot read request: {e}"))?;
    if text.len() > 8_000_000 {
        return Err("Request exceeds 8 MB.".to_string());
    }
    serde_json::from_str(&text).map_err(|e| format!("Invalid request: {e}"))
}

fn root() -> Result<PathBuf, String> {
    resolve_bootstrap_albums_root()
        .ok_or_else(|| "Select Albums Root before importing or opening an album.".to_string())
}

fn renderer() -> Result<(String, Duration), String> {
    let config = crate::config::load_config("config.toml");
    let cli = config
        .effective_albumeasy_cli_path()
        .ok_or_else(|| {
            "Configure the AlbumEasy executable first. Import requires fresh PDF validation."
                .to_string()
        })?
        .to_string();
    Ok((
        cli,
        Duration::from_secs(config.albumeasy_pdf_timeout_secs().into()),
    ))
}

fn start_job<F>(work: F) -> Result<Value, String>
where
    F: FnOnce() -> Result<external_album::AlbumDetails, String> + Send + 'static,
{
    let slot = JOB.get_or_init(|| Mutex::new(None));
    let mut job = slot
        .lock()
        .map_err(|e| format!("Import job lock failed: {e}"))?;
    if job.as_ref().is_some_and(|j| j.running) {
        return Err(
            "An import or layout edit is already running. Wait for it to finish.".to_string(),
        );
    }
    let id = NEXT_JOB.fetch_add(1, Ordering::Relaxed);
    *job = Some(ImportJob {
        id,
        running: true,
        result: None,
        error: None,
    });
    thread::Builder::new()
        .name("external-album".to_string())
        .spawn(move || {
            let outcome = work().and_then(|details| {
                serde_json::to_value(details).map_err(|e| format!("Cannot serialize album: {e}"))
            });
            match slot.lock() {
                Ok(mut job) => {
                    if let Some(job) = job.as_mut().filter(|j| j.id == id) {
                        job.running = false;
                        match outcome {
                            Ok(details) => {
                                log_runtime_event(&format!("External album job {id} finished."));
                                job.result = Some(details);
                            }
                            Err(error) => {
                                log_runtime_event(&format!(
                                    "External album job {id} failed: {error}"
                                ));
                                job.error = Some(error);
                            }
                        }
                    }
                }
                Err(error) => {
                    log_runtime_event(&format!("Import job completion lock failed: {error}"))
                }
            }
        })
        .map_err(|e| {
            *job = None;
            format!("Cannot start import worker: {e}")
        })?;
    Ok(json!({"job": id}))
}

fn parameter(url: &str, name: &str) -> Result<String, String> {
    let query = url.split_once('?').map_or("", |(_, q)| q);
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == name {
                return urlencoding::decode(value)
                    .map(|v| v.into_owned())
                    .map_err(|e| format!("Invalid query parameter: {e}"));
            }
        }
    }
    Err(format!("Missing query parameter: {name}"))
}

fn same_origin(request: &tiny_http::Request) -> Result<(), String> {
    let header = |name: &'static str| {
        request
            .headers()
            .iter()
            .find(|h| h.field.equiv(name))
            .map(|h| h.value.as_str())
    };
    if let Some(origin) = header("Origin") {
        let host = header("Host").ok_or_else(|| "Missing Host header.".to_string())?;
        if origin != format!("http://{host}") {
            return Err("External-origin requests are not allowed.".to_string());
        }
    }
    if request.method() == &Method::Post
        && !header("Content-Type").is_some_and(|v| v.starts_with("application/json"))
    {
        return Err("Use application/json for album requests.".to_string());
    }
    Ok(())
}

fn route(request: &mut tiny_http::Request, action: &str) -> Result<Value, String> {
    match (request.method(), action) {
        (&Method::Get, "list") => Ok(json!({"albums": external_album::list_albums(&root()?)?})),
        (&Method::Get, "load") => Ok(
            json!({"album": external_album::load_album(&root()?, &parameter(request.url(), "name")?)?}),
        ),
        (&Method::Get, "job") => {
            let id: u64 = parameter(request.url(), "id")?
                .parse()
                .map_err(|e| format!("Invalid job ID: {e}"))?;
            let job = JOB
                .get_or_init(|| Mutex::new(None))
                .lock()
                .map_err(|e| format!("Import job lock failed: {e}"))?;
            let job = job
                .as_ref()
                .filter(|j| j.id == id)
                .ok_or_else(|| "Job is no longer available. Refresh the album list.".to_string())?;
            Ok(json!({"job": job}))
        }
        (&Method::Post, "pick") => {
            let selected = pick_folder_via_dialog("Select external AlbumEasy album folder", ".")?;
            let entries = match &selected {
                Some(path) => {
                    let mut entries = Vec::new();
                    for item in
                        fs::read_dir(path).map_err(|e| format!("Cannot read folder: {e}"))?
                    {
                        let item = item.map_err(|e| format!("Cannot read folder entry: {e}"))?;
                        if item.file_type().map_err(|e| e.to_string())?.is_file()
                            && item
                                .path()
                                .extension()
                                .is_some_and(|e| e.eq_ignore_ascii_case("txt"))
                        {
                            entries.push(item.file_name().to_string_lossy().into_owned());
                        }
                    }
                    entries.sort();
                    entries
                }
                None => Vec::new(),
            };
            Ok(json!({"source": selected, "entries": entries}))
        }
        (&Method::Post, "inspect") => {
            let input: SourceRequest = body(request)?;
            Ok(
                json!({"inspection": external_album::inspect(Path::new(&input.source), Path::new(&input.entry))?}),
            )
        }
        (&Method::Post, "import") => {
            let input: ImportRequest = body(request)?;
            let root = root()?;
            let (cli, timeout) = renderer()?;
            start_job(move || {
                external_album::import_album(
                    &root,
                    &input.name,
                    Path::new(&input.source),
                    Path::new(&input.entry),
                    &cli,
                    timeout,
                )
            })
        }
        (&Method::Post, "retry") => {
            let input: AlbumRequest = body(request)?;
            let root = root()?;
            let (cli, timeout) = renderer()?;
            start_job(move || external_album::retry_album(&root, &input.name, &cli, timeout))
        }
        (&Method::Post, "approve") => {
            let input: AlbumRequest = body(request)?;
            let revision = input
                .revision
                .ok_or_else(|| "Approval requires the current revision.".to_string())?;
            Ok(json!({"album": external_album::approve_album(&root()?, &input.name, revision)?}))
        }
        (&Method::Post, "edit") => {
            let input: EditRequest = body(request)?;
            let root = root()?;
            let (cli, timeout) = renderer()?;
            start_job(move || {
                external_album::edit_page(
                    &root,
                    &input.name,
                    input.page,
                    input.commands,
                    input.revision,
                    &cli,
                    timeout,
                )
            })
        }
        (&Method::Post, "image") => {
            let input: ImageRequest = body(request)?;
            let root = root()?;
            let album = external_album::load_album(&root, &input.name)?;
            if album.status != "ready" {
                return Err("Approve the imported album before adding images.".to_string());
            }
            let job = JOB
                .get_or_init(|| Mutex::new(None))
                .lock()
                .map_err(|e| format!("Import job lock failed: {e}"))?;
            if job.as_ref().is_some_and(|j| j.running) {
                return Err(
                    "Wait for the current album operation before adding images.".to_string()
                );
            }
            let assets = external_album::asset_directory(&root, &album.name)?;
            let album_dir = assets
                .parent()
                .ok_or_else(|| "Imported image directory has no album parent.".to_string())?;
            let path = store_image(album_dir, &input.data_url)?;
            Ok(json!({"path": path}))
        }
        _ => Err("Unknown album action or unsupported HTTP method.".to_string()),
    }
}

fn store_image(album: &Path, data_url: &str) -> Result<String, String> {
    let (header, data) = data_url
        .split_once(',')
        .ok_or_else(|| "Invalid image data URL.".to_string())?;
    if !matches!(header, "data:image/png;base64" | "data:image/jpeg;base64") {
        return Err("Select a PNG or JPEG image.".to_string());
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Invalid image bytes: {e}"))?;
    if bytes.len() > 5_000_000 {
        return Err("Image exceeds the 5 MB upload limit.".to_string());
    }
    let format = image::guess_format(&bytes).map_err(|e| format!("Cannot identify image: {e}"))?;
    let extension = match format {
        image::ImageFormat::Png => "png",
        image::ImageFormat::Jpeg => "jpg",
        _ => return Err("Image must contain PNG or JPEG data.".to_string()),
    };
    let mut reader = image::ImageReader::with_format(std::io::Cursor::new(&bytes), format);
    let mut limits = image::Limits::default();
    limits.max_alloc = Some(64 * 1024 * 1024);
    reader.limits(limits);
    reader
        .decode()
        .map_err(|e| format!("Invalid or oversized image: {e}"))?;
    let folder = album.join("imported-assets");
    for entry in fs::read_dir(album).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.eq_ignore_ascii_case("imported-assets") && name != "imported-assets" {
            return Err(
                "An existing imported-assets name has incompatible capitalization.".to_string(),
            );
        }
    }
    if folder.exists() {
        let meta = fs::symlink_metadata(&folder).map_err(|e| e.to_string())?;
        if meta.file_type().is_symlink() || !meta.is_dir() {
            return Err("Imported asset destination is not a regular directory.".to_string());
        }
    } else {
        fs::create_dir(&folder).map_err(|e| format!("Cannot create image folder: {e}"))?;
    }
    let file_name = format!(
        "{}.{}",
        chrono::Utc::now()
            .timestamp_nanos_opt()
            .ok_or_else(|| "Cannot determine image timestamp.".to_string())?,
        extension
    );
    let path = folder.join(&file_name);
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| format!("Cannot create uploaded image: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Cannot save image {}: {e}", path.display()))?;
    log_runtime_event(&format!("Imported album image saved: {}", path.display()));
    Ok(format!("imported-assets/{file_name}"))
}

fn pdf_path(root: &Path, name: &str, which: &str) -> Result<PathBuf, String> {
    let details = external_album::load_album(root, name)?;
    let relative = match which {
        "reference" => &details.reference_pdf,
        "generated" => &details.generated_pdf,
        _ => return Err("Unknown PDF selection.".to_string()),
    };
    let album = root
        .join(&details.name)
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let path = album
        .join(relative)
        .canonicalize()
        .map_err(|e| format!("PDF unavailable: {e}"))?;
    if !path.starts_with(&album)
        || !path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("pdf"))
    {
        return Err("PDF path is outside the imported album.".to_string());
    }
    Ok(path)
}

pub(super) fn handle(mut request: tiny_http::Request) -> Option<tiny_http::Request> {
    let url = request.url().to_string();
    let Some(action) = url.strip_prefix(PREFIX) else {
        return Some(request);
    };
    let action = action.split('?').next().unwrap_or("");
    if let Err(error) = same_origin(&request) {
        respond(request, Err(error), 403);
        return None;
    }
    if action == "pdf" && request.method() == &Method::Get {
        let result = root()
            .and_then(|root| pdf_path(&root, &parameter(&url, "name")?, &parameter(&url, "which")?))
            .and_then(|path| fs::File::open(path).map_err(|e| e.to_string()));
        match result {
            Ok(file) => {
                let response = Response::from_file(file)
                    .with_header(
                        Header::from_bytes("Content-Type", "application/pdf")
                            .expect("static header"),
                    )
                    .with_header(
                        Header::from_bytes("Cache-Control", "no-store").expect("static header"),
                    );
                if let Err(error) = request.respond(response) {
                    log_runtime_event(&format!("Cannot send imported PDF: {error}"));
                }
            }
            Err(error) => respond(request, Err(error), 404),
        }
        return None;
    }
    let result = route(&mut request, action);
    respond(request, result, 400);
    None
}

fn respond(request: tiny_http::Request, result: Result<Value, String>, error_status: u16) {
    let (mut data, status) = match result {
        Ok(value) => (value, 200),
        Err(error) => {
            log_runtime_event(&format!("External album API: {error}"));
            (json!({"message": error}), error_status)
        }
    };
    data["ok"] = json!(status == 200);
    let response = Response::from_string(data.to_string())
        .with_status_code(StatusCode(status))
        .with_header(Header::from_bytes("Content-Type", "application/json").expect("static header"))
        .with_header(Header::from_bytes("Cache-Control", "no-store").expect("static header"));
    if let Err(error) = request.respond(response) {
        log_runtime_event(&format!("Cannot send external album response: {error}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_album_query_decodes_names_without_interpreting_paths() {
        assert_eq!(
            parameter("/load?name=Brazil%20Album", "name").expect("name"),
            "Brazil Album"
        );
        assert!(parameter("/load", "name").is_err());
    }

    #[test]
    fn external_album_image_upload_preserves_original_and_rejects_invalid_data() {
        let directory = tempfile::tempdir().expect("album folder");
        let original = directory.path().join("Images");
        fs::create_dir(&original).expect("original images");
        fs::write(original.join("original.jpg"), b"original").expect("original file");
        let png = image::DynamicImage::new_rgb8(2, 2);
        let mut encoded = std::io::Cursor::new(Vec::new());
        png.write_to(&mut encoded, image::ImageFormat::Png)
            .expect("PNG");
        let data = format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(encoded.into_inner())
        );
        let path = store_image(directory.path(), &data).expect("upload");
        assert!(directory.path().join(path).is_file());
        assert_eq!(
            fs::read(original.join("original.jpg")).expect("original"),
            b"original"
        );
        assert!(store_image(directory.path(), "data:image/svg+xml;base64,AA==").is_err());
        assert!(store_image(directory.path(), "data:image/png;base64,AA==").is_err());
    }

    #[test]
    fn external_album_image_upload_rejects_case_collision() {
        let directory = tempfile::tempdir().expect("album folder");
        fs::create_dir(directory.path().join("Imported-Assets")).expect("existing case");
        let mut encoded = std::io::Cursor::new(Vec::new());
        image::DynamicImage::new_rgb8(1, 1)
            .write_to(&mut encoded, image::ImageFormat::Png)
            .expect("PNG");
        let data = format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(encoded.into_inner())
        );
        assert!(
            store_image(directory.path(), &data)
                .expect_err("case conflict")
                .contains("capitalization")
        );
    }

    #[test]
    fn external_album_inspect_endpoint_reports_sample_and_rejects_cross_origin() {
        let server = Server::http("127.0.0.1:0").expect("test server");
        let address = server.server_addr().to_ip().expect("IP");
        let worker = thread::spawn(move || {
            for request in server.incoming_requests().take(3) {
                assert!(handle(request).is_none());
            }
        });
        let client = reqwest::blocking::Client::new();
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("Brazil");
        // The user's album is optional outside this development checkout.
        let folder = tempfile::tempdir().expect("source fixture");
        let fixture = if source.is_dir() {
            source
        } else {
            fs::write(
                folder.path().join("album.txt"),
                "PAGE_START\nPAGE_TEXT_CENTRE (HN 12 \"Example\")\n",
            )
            .expect("source");
            folder.path().to_path_buf()
        };
        let entry = if fixture.join("Brazil.txt").exists() {
            "Brazil.txt"
        } else {
            "album.txt"
        };
        let response = client
            .post(format!("http://{address}{PREFIX}inspect"))
            .json(&json!({"source": fixture, "entry": entry}))
            .send()
            .expect("inspect");
        assert_eq!(response.status().as_u16(), 200);
        let data: Value = response.json().expect("inspection");
        assert_eq!(data["ok"], true);
        if entry == "Brazil.txt" {
            assert_eq!(data["inspection"]["pages"], 85);
            assert_eq!(data["inspection"]["placements"], 655);
            assert_eq!(data["inspection"]["unique_images"], 649);
        }
        let response = client
            .post(format!("http://{address}{PREFIX}inspect"))
            .header("Origin", "https://unrelated.example")
            .json(&json!({"source": fixture, "entry": entry}))
            .send()
            .expect("cross origin");
        assert_eq!(response.status().as_u16(), 403);
        let response = client
            .post(format!("http://{address}{PREFIX}inspect"))
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .expect("invalid request");
        assert_eq!(response.status().as_u16(), 400);
        worker.join().expect("API worker");
    }
}
