use super::*;

pub(super) struct BootstrapHandlersState {
    pub selected: Option<(String, String)>,
    pub logout_requested: bool,
    pub reload_requested: bool,
}

pub(super) struct ReviewHandlersState {
    pub reload_requested: bool,
    pub return_to_start_requested: bool,
}

pub(super) fn serve_bootstrap_requests(server: &Server, html: &String) -> BootstrapHandlersState {
    let mut selected: Option<(String, String)> = None;
    let mut logout_requested = false;
    let reload_requested = false;

    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let req_url = request.url().to_string();

        if method == Method::Get && req_url == "/" {
            let mut resp = Response::from_string(html.clone());
            if let Ok(h) =
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
            {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/bootstrap-state" {
            let state = BootstrapStateResponse {
                countries: scan_bootstrap_countries(),
                albums_root: current_bootstrap_albums_root(),
            };
            let mut resp = Response::from_string(
                serde_json::to_string(&state).unwrap_or_else(|_| "{\"countries\":[]}".to_string()),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/bootstrap-set-albums-root" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<BootstrapSetAlbumsRootRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => match set_bootstrap_albums_root_override(&req.path) {
                    Ok(albums_root) => {
                        let countries_count = scan_bootstrap_countries().len();
                        BootstrapSetAlbumsRootResponse {
                            ok: true,
                            message: if req.path.trim().is_empty() {
                                "Albums root override cleared; using auto-detection".to_string()
                            } else {
                                "Albums root updated".to_string()
                            },
                            albums_root,
                            countries_count,
                        }
                    }
                    Err(error) => BootstrapSetAlbumsRootResponse {
                        ok: false,
                        message: error,
                        albums_root: current_bootstrap_albums_root(),
                        countries_count: scan_bootstrap_countries().len(),
                    },
                },
                Err(error) => BootstrapSetAlbumsRootResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    albums_root: current_bootstrap_albums_root(),
                    countries_count: scan_bootstrap_countries().len(),
                },
            };

            let mut resp = Response::from_string(
                serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"albums_root\":null,\"countries_count\":0}".to_string()
                }),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/bootstrap-pick-albums-root" {
            let response = match pick_bootstrap_albums_root_via_dialog() {
                Ok(Some(path)) => match set_bootstrap_albums_root_override(&path) {
                    Ok(albums_root) => BootstrapPickAlbumsRootResponse {
                        ok: true,
                        picked: true,
                        message: "Albums root selected".to_string(),
                        albums_root,
                        countries_count: scan_bootstrap_countries().len(),
                    },
                    Err(error) => BootstrapPickAlbumsRootResponse {
                        ok: false,
                        picked: true,
                        message: error,
                        albums_root: current_bootstrap_albums_root(),
                        countries_count: scan_bootstrap_countries().len(),
                    },
                },
                Ok(None) => BootstrapPickAlbumsRootResponse {
                    ok: true,
                    picked: false,
                    message: "Folder selection canceled".to_string(),
                    albums_root: current_bootstrap_albums_root(),
                    countries_count: scan_bootstrap_countries().len(),
                },
                Err(error) => BootstrapPickAlbumsRootResponse {
                    ok: false,
                    picked: false,
                    message: error,
                    albums_root: current_bootstrap_albums_root(),
                    countries_count: scan_bootstrap_countries().len(),
                },
            };

            let mut resp = Response::from_string(
                serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"picked\":false,\"message\":\"serialization failed\",\"albums_root\":null,\"countries_count\":0}".to_string()
                }),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/runtime-log" {
            let mut resp = Response::from_string(
                serde_json::json!({
                    "ok": true,
                    "lines": get_runtime_log_lines()
                })
                .to_string(),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/health" {
            let mut resp = Response::from_string(
                serde_json::json!({
                    "ok": true,
                    "mode": "bootstrap"
                })
                .to_string(),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/stampworld-status" {
            let response = {
                let state = stampworld_job_state()
                    .lock()
                    .ok()
                    .map(|s| s.clone())
                    .unwrap_or(StampWorldJobState::Idle);

                match state {
                    StampWorldJobState::Idle => BootstrapStampWorldStatusResponse {
                        ok: true,
                        running: false,
                        finished: false,
                        success: None,
                        message: "No StampWorld job has started yet.".to_string(),
                        country: None,
                        db_path: None,
                    },
                    StampWorldJobState::Running { country, db_path } => {
                        BootstrapStampWorldStatusResponse {
                            ok: true,
                            running: true,
                            finished: false,
                            success: None,
                            message: format!("StampWorld pull is running for {}...", country),
                            country: Some(country),
                            db_path: Some(db_path),
                        }
                    }
                    StampWorldJobState::Finished {
                        success,
                        country,
                        db_path,
                        message,
                    } => BootstrapStampWorldStatusResponse {
                        ok: true,
                        running: false,
                        finished: true,
                        success: Some(success),
                        message,
                        country: Some(country),
                        db_path: Some(db_path),
                    },
                }
            };

            let mut resp = Response::from_string(
                serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"running\":false,\"finished\":false,\"success\":null,\"message\":\"serialization failed\",\"country\":null,\"db_path\":null}".to_string()
                }),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/stampworld-log-tail" {
            let lines = tail_current_log(50); // Get last 50 lines
            let response = StampWorldLogTailResponse { ok: true, lines };
            let mut resp = Response::from_string(
                serde_json::to_string(&response)
                    .unwrap_or_else(|_| "{\"ok\":false,\"lines\":[]}".to_string()),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/stampworld-run" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<BootstrapStampWorldRunRequest, _> = serde_json::from_str(&body);
            let (response, status_code) = match parsed {
                Ok(req) => match start_bootstrap_stampworld_job(req) {
                    Ok(msg) => (
                        BootstrapStampWorldRunResponse {
                            ok: true,
                            message: msg,
                            started: true,
                        },
                        StatusCode(200),
                    ),
                    Err(error) => (
                        BootstrapStampWorldRunResponse {
                            ok: false,
                            message: error,
                            started: false,
                        },
                        StatusCode(409),
                    ),
                },
                Err(error) => (
                    BootstrapStampWorldRunResponse {
                        ok: false,
                        message: format!("invalid request: {}", error),
                        started: false,
                    },
                    StatusCode(400),
                ),
            };

            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"started\":false}"
                        .to_string()
                }))
                .with_status_code(status_code);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/albumeasy-status" {
            let cfg = crate::config::load_config("config.toml");
            let cli_path = cfg.effective_albumeasy_cli_path().map(str::to_owned);
            let path_exists = cli_path
                .as_deref()
                .map(|p| Path::new(p).exists())
                .unwrap_or(false);
            let configured = cli_path
                .as_deref()
                .map(|p| !p.trim().is_empty())
                .unwrap_or(false);

            let message = if configured && path_exists {
                "AlbumEasy CLI is configured and path exists.".to_string()
            } else if configured {
                "AlbumEasy CLI path is set but file was not found.".to_string()
            } else {
                "AlbumEasy CLI is not configured.".to_string()
            };

            let response = BootstrapAlbumEasyStatusResponse {
                ok: true,
                configured,
                cli_path,
                path_exists,
                message,
                download_url: "https://www.thestampweb.com/albumeasy/download".to_string(),
            };

            let mut resp = Response::from_string(
                serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"configured\":false,\"cli_path\":null,\"path_exists\":false,\"message\":\"serialization failed\",\"download_url\":\"https://www.thestampweb.com/albumeasy/download\"}".to_string()
                }),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/albumeasy-set-path" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<BootstrapAlbumEasySetPathRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => match save_runtime_albumeasy_cli_path(&req.cli_path) {
                    Ok(_) => BootstrapAlbumEasySetPathResponse {
                        ok: true,
                        message: "Saved runtime.albumeasy_cli_path in config.toml".to_string(),
                    },
                    Err(error) => BootstrapAlbumEasySetPathResponse {
                        ok: false,
                        message: error,
                    },
                },
                Err(error) => BootstrapAlbumEasySetPathResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                },
            };

            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\"}".to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/config-editor" {
            let resp_json = match build_config_editor_data("config.toml", "config_template.toml") {
                Ok(sections) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: true,
                    message: "Config editor data loaded".to_string(),
                    sections,
                    album_name: String::new(),
                    has_album_override: false,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
                Err(error) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: false,
                    message: format!("Failed to load config editor data: {}", error),
                    sections: Vec::new(),
                    album_name: String::new(),
                    has_album_override: false,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"failed to load config editor data\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-save" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<SaveConfigEditorRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => match save_full_config_values(
                    "config.toml",
                    "config_template.toml",
                    &req.values,
                ) {
                    Ok(_) => SaveConfigEditorResponse {
                        ok: true,
                        message:
                            "config.toml updated. New startup actions will use the saved values immediately."
                                .to_string(),
                        reload_required: false,
                    },
                    Err(error) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to save config.toml: {}", error),
                        reload_required: false,
                    },
                },
                Err(error) => SaveConfigEditorResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    reload_required: false,
                },
            };

            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/start-review" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<BootstrapStartRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => {
                    let countries = scan_bootstrap_countries();
                    let found = countries
                        .into_iter()
                        .find(|c| c.country.eq_ignore_ascii_case(req.country.trim()));
                    match found {
                        Some(c) => {
                            log_runtime_event(&format!(
                                "Review UI: country selected '{}'; preparing compare view from {}",
                                c.country, c.db_path
                            ));
                            selected = Some((c.db_path.clone(), c.country.clone()));
                            BootstrapStartResponse {
                                ok: true,
                                message: format!(
                                    "Selected {}. Preparing Review UI now...",
                                    c.country
                                ),
                                country: Some(c.country),
                                db_path: Some(c.db_path),
                            }
                        }
                        None => BootstrapStartResponse {
                            ok: false,
                            message: "Selected country not found or missing DB".to_string(),
                            country: None,
                            db_path: None,
                        },
                    }
                }
                Err(e) => BootstrapStartResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    country: None,
                    db_path: None,
                },
            };

            let mut resp = Response::from_string(
                serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"country\":null,\"db_path\":null}"
                        .to_string()
                }),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);

            if selected.is_some() {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/logout" {
            let response = serde_json::json!({
                "ok": true,
                "message": "Review UI startup closed"
            });
            let mut resp = Response::from_string(response.to_string());
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            logout_requested = true;
            break;
        }

        if method == Method::Post && req_url == "/api/generate-pdf-now" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let Some(_pdf_generation_guard) = PdfGenerationGuard::try_acquire() else {
                let resp_json = serde_json::to_string(&BootstrapGeneratePdfResponse {
                    ok: false,
                    message:
                        "PDF generation already in progress. Please wait for it to complete."
                            .to_string(),
                    generated_output_path: None,
                    generated_pdf_path: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"PDF generation already in progress\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                        .to_string()
                });

                let mut resp = Response::from_string(resp_json).with_status_code(StatusCode(409));
                if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                    resp = resp.with_header(h);
                }
                let _ = request.respond(resp);
                continue;
            };

            let parsed: Result<BootstrapGeneratePdfRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => {
                    let countries = scan_bootstrap_countries();
                    let found = countries
                        .into_iter()
                        .find(|c| c.country.eq_ignore_ascii_case(req.country.trim()));
                    match found {
                        Some(c) => {
                            match crate::db_utils::init_db(&c.db_path) {
                                Ok(conn) => {
                                    let album_dir = album_dir_for_ui(&c.db_path, &c.country);
                                    let artifacts = crate::config::with_album_config_override(
                                        &album_dir,
                                        || {
                                            crate::album_globals::generate_album_output_conn_with_pdf_override(
                                                &conn,
                                                &c.country,
                                                None::<fn(usize)>,
                                                req.start_year,
                                                req.end_year,
                                                Some(true),
                                            )
                                        },
                                    );
                                    if let Some(pdf_path) = artifacts.generated_pdf_path.as_ref() {
                                        open_local_path(pdf_path);
                                    }

                                    serde_json::to_string(&BootstrapGeneratePdfResponse {
                                        ok: true,
                                        message: format!("Generated album output for {}", c.country),
                                        generated_output_path: Some(artifacts.output_txt_path.display().to_string()),
                                        generated_pdf_path: artifacts
                                            .generated_pdf_path
                                            .as_ref()
                                            .map(|p| p.display().to_string()),
                                    })
                                    .unwrap_or_else(|_| {
                                        "{\"ok\":false,\"message\":\"serialization failed\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                                            .to_string()
                                    })
                                }
                                Err(e) => serde_json::to_string(&BootstrapGeneratePdfResponse {
                                    ok: false,
                                    message: format!("failed to open DB {}: {}", c.db_path, e),
                                    generated_output_path: None,
                                    generated_pdf_path: None,
                                })
                                .unwrap_or_else(|_| {
                                    "{\"ok\":false,\"message\":\"failed to open DB\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                                        .to_string()
                                }),
                            }
                        }
                        None => serde_json::to_string(&BootstrapGeneratePdfResponse {
                            ok: false,
                            message: "Selected country not found or missing DB".to_string(),
                            generated_output_path: None,
                            generated_pdf_path: None,
                        })
                        .unwrap_or_else(|_| {
                            "{\"ok\":false,\"message\":\"selected country not found\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                                .to_string()
                        }),
                    }
                }
                Err(e) => serde_json::to_string(&BootstrapGeneratePdfResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    generated_output_path: None,
                    generated_pdf_path: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/colnect-import" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ColnectImportRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                    Ok(_req) => serde_json::to_string(&ColnectImportResponse {
                        ok: false,
                        message: "Bootstrap stage: please select a country and open Review UI first, then use Import Colnect".to_string(),
                        files_processed: 0,
                        stamps_imported: 0,
                        stamps_skipped: 0,
                        errors: vec!["Import not available in bootstrap stage".to_string()],
                    })
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"files_processed\":0,\"stamps_imported\":0,\"stamps_skipped\":0,\"errors\":[]}"
                            .to_string()
                    }),
                    Err(e) => serde_json::to_string(&ColnectImportResponse {
                        ok: false,
                        message: format!("invalid request: {}", e),
                        files_processed: 0,
                        stamps_imported: 0,
                        stamps_skipped: 0,
                        errors: vec![format!("invalid request: {}", e)],
                    })
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"invalid request\",\"files_processed\":0,\"stamps_imported\":0,\"stamps_skipped\":0,\"errors\":[]}"
                            .to_string()
                    }),
                };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/bootstrap-pick-colnect-file" {
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            let resp_json = match serde_json::from_str::<BootstrapPickColnectFileRequest>(&body) {
                Ok(req) => {
                    let result = pick_and_copy_colnect_file_via_dialog(&req.country);
                    serde_json::to_string(&result).unwrap_or_else(|_| {
                        "{\"ok\":false,\"picked\":false,\"message\":\"serialization error\"}"
                            .to_string()
                    })
                }
                Err(e) => format!(
                    "{{\"ok\":false,\"picked\":false,\"message\":\"invalid request: {}\"}}",
                    e
                ),
            };
            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        let _ = request.respond(Response::empty(StatusCode(404)));
    }

    BootstrapHandlersState {
        selected,
        logout_requested,
        reload_requested,
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn serve_review_requests(
    server: &Server,
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    html: &String,
    right_map: &HashMap<String, RightCandidate>,
    default_catalog_code: &str,
    odd_row_color: &str,
    even_row_color: &str,
    series_default_checked: bool,
    description_default_checked: bool,
    watermark_default_checked: bool,
    design_default_checked: bool,
    engraving_default_checked: bool,
    perforation_default_checked: bool,
    context_default_checked: bool,
    colnect_links_default_checked: bool,
    colnect_image_url_default_checked: bool,
    colnect_metadata_default_checked: bool,
    replace_sg_number_default_checked: bool,
    right_unmatched_only_default_checked: bool,
    albumeasy_auto_generate_pdf_default: bool,
    albumeasy_cli_configured: bool,
) -> ReviewHandlersState {
    let reload_requested = false;
    let mut return_to_start_requested = false;

    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let req_url = request.url().to_string();

        if method == Method::Get && req_url == "/runtime-log" {
            let mut resp = Response::from_string(
                serde_json::json!({
                    "ok": true,
                    "lines": get_runtime_log_lines()
                })
                .to_string(),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/health" {
            let mut resp = Response::from_string(
                serde_json::json!({
                    "ok": true,
                    "mode": "review"
                })
                .to_string(),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/compare-ready" {
            let mut resp = Response::from_string("{\"ready\":true}".to_string());
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/config-editor" {
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let resp_json = match build_config_editor_data_for_album(
                "config.toml",
                "config_template.toml",
                &album_dir,
            ) {
                Ok((sections, has_override)) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: true,
                    message: "Config editor data loaded".to_string(),
                    sections,
                    album_name: album_name.to_string(),
                    has_album_override: has_override,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
                Err(error) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: false,
                    message: format!("Failed to load config editor data: {}", error),
                    sections: Vec::new(),
                    album_name: album_name.to_string(),
                    has_album_override: false,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"failed to load config editor data\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && (req_url == "/" || req_url.starts_with("/?")) {
            let show_imageless_left_rows = parse_query_flag(&req_url, "show_no_image");
            let html = if show_imageless_left_rows {
                match build_matcher_payload(
                    &conn,
                    db_path,
                    &album_name,
                    default_catalog_code,
                    odd_row_color,
                    even_row_color,
                    series_default_checked,
                    description_default_checked,
                    watermark_default_checked,
                    design_default_checked,
                    engraving_default_checked,
                    perforation_default_checked,
                    context_default_checked,
                    colnect_links_default_checked,
                    colnect_image_url_default_checked,
                    colnect_metadata_default_checked,
                    replace_sg_number_default_checked,
                    right_unmatched_only_default_checked,
                    albumeasy_auto_generate_pdf_default,
                    albumeasy_cli_configured,
                    show_imageless_left_rows,
                ) {
                    Ok(payload) => render_html(&payload),
                    Err(error) => {
                        log_runtime_event(&format!(
                            "Review UI: failed to reload compare payload with image-less rows: {}",
                            error
                        ));
                        html.clone()
                    }
                }
            } else {
                html.clone()
            };

            let mut resp = Response::from_string(html);
            if let Ok(h) =
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
            {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url.starts_with("/image?") {
            let Some(path) = parse_query_param(&req_url, "path") else {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            };
            let p = PathBuf::from(path);
            if !p.exists() {
                let _ = request.respond(Response::empty(StatusCode(404)));
                continue;
            }
            match fs::File::open(&p) {
                Ok(f) => {
                    let mut resp = Response::from_file(f);
                    if let Ok(h) =
                        Header::from_bytes(&b"Content-Type"[..], mime_from_path(&p).as_bytes())
                    {
                        resp = resp.with_header(h);
                    }
                    let _ = request.respond(resp);
                }
                Err(_) => {
                    let _ = request.respond(Response::empty(StatusCode(500)));
                }
            }
            continue;
        }

        if method == Method::Get && req_url.starts_with("/resolve-colnect-image?") {
            let Some(page_url) = parse_query_param(&req_url, "page_url") else {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            };

            let image_url = crate::colnect::image_fetcher::resolve_colnect_image_url(&page_url)
                .unwrap_or_default();
            let resp_json = serde_json::json!({
                "ok": !image_url.trim().is_empty(),
                "image_url": image_url,
            })
            .to_string();

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/apply-matches" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ApplyMatchesEnvelope, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&apply_matches(
                    &conn,
                    &db_path,
                    &album_name,
                    req.matches,
                    req.fields,
                    &right_map,
                ))
                .unwrap_or_else(|_| {
                    "{\"updated_rows\":0,\"skipped_rows\":0,\"skipped_details\":[],\"processed_colnect_ids\":[],\"errors\":[\"serialization failed\"]}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&ApplyMatchesResponse {
                    updated_rows: 0,
                    skipped_rows: 0,
                    skipped_details: Vec::new(),
                    processed_colnect_ids: Vec::new(),
                    errors: vec![format!("invalid request: {}", e)],
                })
                .unwrap_or_else(|_| {
                    "{\"updated_rows\":0,\"skipped_rows\":0,\"skipped_details\":[],\"processed_colnect_ids\":[],\"errors\":[\"invalid request\"]}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/save-left-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<SaveLeftStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&save_left_stamp(
                    &conn,
                    db_path,
                    &album_name,
                    req,
                ))
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"updated_left\":null}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&SaveLeftStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    updated_left: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"updated_left\":null}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/add-left-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<AddLeftStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&add_left_stamp(&conn, db_path, &album_name, req))
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"added_left\":null}"
                            .to_string()
                    }),
                Err(e) => serde_json::to_string(&AddLeftStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    added_left: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"added_left\":null}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/delete-left-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<DeleteLeftStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&delete_left_stamp(&conn, req)).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"deleted_rowid\":null,\"restored_colnect_id\":null}".to_string()
                }),
                Err(e) => serde_json::to_string(&DeleteLeftStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    deleted_rowid: None,
                    restored_colnect_id: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"deleted_rowid\":null,\"restored_colnect_id\":null}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/clear-left-colnect-link" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ClearLeftColnectLinkRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&clear_left_colnect_link(
                    &conn,
                    db_path,
                    &album_name,
                    req,
                ))
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"updated_left\":null,\"restored_colnect_id\":null}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&ClearLeftColnectLinkResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    updated_left: None,
                    restored_colnect_id: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"updated_left\":null,\"restored_colnect_id\":null}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/add-colnect-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<AddColnectStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&add_colnect_stamp_to_stamps(
                    &conn,
                    db_path,
                    &album_name,
                    req,
                ))
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"added_left\":null,\"processed_colnect_id\":null}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&AddColnectStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    added_left: None,
                    processed_colnect_id: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"added_left\":null,\"processed_colnect_id\":null}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/delete-colnect-candidate" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<DeleteColnectCandidateRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&delete_colnect_candidate(&conn, req)).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"deleted_stamp_rows\":0,\"deleted_code_rows\":0}".to_string()
                }),
                Err(e) => serde_json::to_string(&DeleteColnectCandidateResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    deleted_stamp_rows: 0,
                    deleted_code_rows: 0,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"deleted_stamp_rows\":0,\"deleted_code_rows\":0}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/undo-colnect-delete" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<UndoColnectDeleteRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => {
                    serde_json::to_string(&undo_colnect_delete(&conn, req)).unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"restored_rows\":0}"
                            .to_string()
                    })
                }
                Err(e) => serde_json::to_string(&UndoColnectDeleteResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    restored_rows: 0,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"restored_rows\":0}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/colnect-import" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ColnectImportRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&colnect_import_data(
                    &conn,
                    req,
                    db_path,
                    &album_name,
                ))
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"files_processed\":0,\"stamps_imported\":0,\"stamps_skipped\":0,\"errors\":[]}"
                            .to_string()
                    }),
                Err(e) => serde_json::to_string(&ColnectImportResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    files_processed: 0,
                    stamps_imported: 0,
                    stamps_skipped: 0,
                    errors: vec![format!("invalid request: {}", e)],
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"files_processed\":0,\"stamps_imported\":0,\"stamps_skipped\":0,\"errors\":[]}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/default-catalog-config" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<UpdateDefaultCatalogRequest, _> = serde_json::from_str(&body);
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let resp_json = match parsed {
                Ok(req) => {
                    match update_default_catalog_code_for_album(
                        &album_dir,
                        &album_name,
                        "config.toml",
                        "config_template.toml",
                        &req.default_catalog_code,
                    ) {
                        Ok(saved) => serde_json::to_string(&UpdateDefaultCatalogResponse {
                            ok: true,
                            message: format!("Updated album default_catalog_code to '{}'", saved),
                        })
                        .unwrap_or_else(|_| {
                            "{\"ok\":false,\"message\":\"serialization failed\"}".to_string()
                        }),
                        Err(e) => serde_json::to_string(&UpdateDefaultCatalogResponse {
                            ok: false,
                            message: format!("Failed to update config: {}", e),
                        })
                        .unwrap_or_else(|_| {
                            "{\"ok\":false,\"message\":\"failed to update config\"}".to_string()
                        }),
                    }
                }
                Err(e) => serde_json::to_string(&UpdateDefaultCatalogResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                })
                .unwrap_or_else(|_| "{\"ok\":false,\"message\":\"invalid request\"}".to_string()),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-save-album" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }
            let parsed: Result<SaveConfigEditorRequest, _> = serde_json::from_str(&body);
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let response = match parsed {
                Ok(req) => match save_album_config_diff(
                    &album_dir,
                    &album_name,
                    "config.toml",
                    "config_template.toml",
                    &req.values,
                ) {
                    Ok(n) => SaveConfigEditorResponse {
                        ok: true,
                        message: format!(
                            "Album override saved ({} changed field{}) for {}. New generation actions will use it immediately.",
                            n,
                            if n == 1 { "" } else { "s" },
                            album_name
                        ),
                        reload_required: false,
                    },
                    Err(error) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to save album config override: {}", error),
                        reload_required: false,
                    },
                },
                Err(error) => SaveConfigEditorResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    reload_required: false,
                },
            };
            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-delete-album-override" {
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let override_path = album_dir.join("config.toml");
            let response = if override_path.exists() {
                match fs::remove_file(&override_path) {
                    Ok(_) => SaveConfigEditorResponse {
                        ok: true,
                        message: format!(
                            "Album config override for {} deleted. Refresh the page if you want to reload the current view.",
                            album_name
                        ),
                        reload_required: false,
                    },
                    Err(e) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to delete album override: {}", e),
                        reload_required: false,
                    },
                }
            } else {
                SaveConfigEditorResponse {
                    ok: false,
                    message: format!("No album config override found for {}.", album_name),
                    reload_required: false,
                }
            };
            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-save" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<SaveConfigEditorRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => match save_full_config_values(
                    "config.toml",
                    "config_template.toml",
                    &req.values,
                ) {
                    Ok(_) => SaveConfigEditorResponse {
                        ok: true,
                        message:
                            "config.toml updated. New generation actions will use the saved values immediately."
                                .to_string(),
                        reload_required: false,
                    },
                    Err(error) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to save config.toml: {}", error),
                        reload_required: false,
                    },
                },
                Err(error) => SaveConfigEditorResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    reload_required: false,
                },
            };

            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/return-to-start" {
            return_to_start_requested = true;
            let mut resp = Response::from_string(
                "{\"ok\":true,\"message\":\"Returning to Start Review page...\"}".to_string(),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            break;
        }

        if method == Method::Post && req_url == "/generate-pdf-now" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let Some(_pdf_generation_guard) = PdfGenerationGuard::try_acquire() else {
                let resp_json = serde_json::to_string(&BootstrapGeneratePdfResponse {
                    ok: false,
                    message:
                        "PDF generation already in progress. Please wait for it to complete."
                            .to_string(),
                    generated_output_path: None,
                    generated_pdf_path: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"PDF generation already in progress\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                        .to_string()
                });

                let mut resp = Response::from_string(resp_json).with_status_code(StatusCode(409));
                if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                    resp = resp.with_header(h);
                }
                let _ = request.respond(resp);
                continue;
            };

            let parsed_body: Result<serde_json::Value, _> = serde_json::from_str(&body);

            let (req_start_year, req_end_year) = parsed_body
                .as_ref()
                .ok()
                .map(|v| {
                    let sy = v
                        .get("start_year")
                        .and_then(|x| x.as_i64())
                        .map(|x| x as i32);
                    let ey = v.get("end_year").and_then(|x| x.as_i64()).map(|x| x as i32);
                    (sy, ey)
                })
                .unwrap_or((None, None));
            let mut generated_output_path = None;
            let mut generated_pdf_path = None;
            let message = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let album_dir = album_dir_for_ui(db_path, &album_name);
                crate::config::with_album_config_override(&album_dir, || {
                    crate::album_globals::generate_album_output_conn_with_pdf_override(
                        &conn,
                        &album_name,
                        None::<fn(usize)>,
                        req_start_year,
                        req_end_year,
                        Some(true),
                    )
                })
            })) {
                Ok(artifacts) => {
                    generated_output_path = Some(artifacts.output_txt_path.display().to_string());
                    if let Some(pdf_path) = artifacts.generated_pdf_path {
                        open_local_path(&pdf_path);
                        generated_pdf_path = Some(pdf_path.display().to_string());
                        "Album text and PDF generation completed.".to_string()
                    } else {
                        "Album text generated, but no PDF was produced. Check AlbumEasy CLI configuration/logs."
                            .to_string()
                    }
                }
                Err(_) => "Album/PDF generation failed unexpectedly (panic).".to_string(),
            };

            let resp_json = serde_json::to_string(&BootstrapGeneratePdfResponse {
                ok: generated_output_path.is_some(),
                message,
                generated_output_path,
                generated_pdf_path,
            })
            .unwrap_or_else(|_| {
                "{\"ok\":false,\"message\":\"serialization failed\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                    .to_string()
            });

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/shutdown" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ShutdownRequest, _> = serde_json::from_str(&body);
            let mut generated_output_path = None;
            let mut generated_pdf_path = None;
            let message = match parsed {
                Ok(req) => {
                    if req.generate_album {
                        let album_dir = album_dir_for_ui(db_path, &album_name);
                        let generated = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                            || {
                                crate::config::with_album_config_override(&album_dir, || {
                                    crate::album_globals::generate_album_output_conn_with_pdf_override(
                                        &conn,
                                        &album_name,
                                        None::<fn(usize)>,
                                        req.start_year,
                                        req.end_year,
                                        Some(req.generate_pdf),
                                    )
                                })
                            },
                        ));
                        match generated {
                            Ok(artifacts) => {
                                generated_output_path =
                                    Some(artifacts.output_txt_path.display().to_string());
                                if let Some(pdf_path) = artifacts.generated_pdf_path {
                                    open_local_path(&pdf_path);
                                    generated_pdf_path = Some(pdf_path.display().to_string());
                                }
                                match (req.generate_pdf, generated_pdf_path.is_some()) {
                                    (true, true) => {
                                        "Review UI closed. Album text and PDF generation completed."
                                            .to_string()
                                    }
                                    (true, false) => {
                                        "Review UI closed. Album text generation completed, but no PDF was opened. Check AlbumEasy CLI configuration or logs."
                                            .to_string()
                                    }
                                    (false, _) => {
                                        "Review UI closed. Album text generation completed without PDF generation."
                                            .to_string()
                                    }
                                }
                            }
                            Err(_) => {
                                "Review UI closed. Album generation failed unexpectedly (panic)."
                                    .to_string()
                            }
                        }
                    } else {
                        "Review UI closed without album generation.".to_string()
                    }
                }
                Err(e) => format!("Review UI closed (invalid shutdown payload: {}).", e),
            };

            let resp_json = serde_json::to_string(&ShutdownResponse {
                ok: true,
                message,
                generated_output_path,
                generated_pdf_path,
            })
            .unwrap_or_else(|_| {
                "{\"ok\":true,\"message\":\"Review UI closed\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                    .to_string()
            });

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            break;
        }

        let _ = request.respond(Response::empty(StatusCode(404)));
    }

    ReviewHandlersState {
        reload_requested,
        return_to_start_requested,
    }
}
