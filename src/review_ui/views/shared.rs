pub(super) const FOOTER_YEAR_UPDATE_JS: &str = "document.querySelectorAll('.copyrightCurrentYear').forEach((el) => {\n    el.textContent = String(new Date().getFullYear());\n});";

pub(super) fn render_app_footer_html() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "<div class='appFooter' style='margin-top:16px; font-size:12px; color:#5b6f81; text-align:center;'>\
<div>Copyright &copy; 2025 &#x2013; <span class='copyrightCurrentYear'></span> Laser Spider LLC. All rights reserved.</div>\
<div style='margin-top:4px;'>Stamp Album Generator 2 | Version {} </div>\
</div>",
        version
    )
}

pub(super) fn render_footer_year_script() -> String {
    format!("<script>\n{}\n</script>", FOOTER_YEAR_UPDATE_JS)
}
/// Params for `render_reconnect_banner_html`
pub struct ReconnectBannerParams<'a> {
    pub banner_id: &'a str,
    pub title_id: &'a str,
    pub text_id: Option<&'a str>,
    pub text_html: Option<&'a str>,
    pub extra_html: &'a str,
    pub retry_fn: &'a str,
    pub reload_fn: &'a str,
    pub meta_id: &'a str,
}

pub(super) fn render_reconnect_banner_html(params: &ReconnectBannerParams) -> String {
    let text_section = match (params.text_id, params.text_html) {
        (Some(text_id), Some(text_html)) => {
            format!("\n        <div id='{text_id}'>{text_html}</div>")
        }
        _ => String::new(),
    };
    format!(
        r#"<div id='{banner_id}' class='reconnectBanner' role='status' aria-live='polite'>
        <div id='{title_id}' class='title'>Connection lost, retrying...</div>{text_section}{extra_html}
        <div class='actions'>
            <button type='button' onclick='{retry_fn}()'>Retry Now</button>
            <button type='button' onclick='{reload_fn}()'>Reload Page</button>
            <span id='{meta_id}' class='meta'></span>
        </div>
    </div>"#,
        banner_id = params.banner_id,
        title_id = params.title_id,
        text_section = text_section,
        extra_html = params.extra_html,
        retry_fn = params.retry_fn,
        reload_fn = params.reload_fn,
        meta_id = params.meta_id,
    )
}

/// Params for `render_runtime_log_panel_html`
pub struct RuntimeLogPanelParams<'a> {
    pub panel_id: &'a str,
    pub panel_style: &'a str,
    pub header_style: &'a str,
    pub controls_style: &'a str,
    pub body_id: &'a str,
    pub textarea_id: &'a str,
    pub checkbox_id: &'a str,
    pub toggle_fn: &'a str,
    pub copy_fn: &'a str,
}

pub(super) fn render_runtime_log_panel_html(params: &RuntimeLogPanelParams) -> String {
    format!(
        r#"<div id='{panel_id}' style='{panel_style}'>
        <div style='{header_style}'>
            <strong>Runtime Log</strong>
            <div style='{controls_style}'>
                <label style='font-size:12px;font-weight:600;display:flex;align-items:center;gap:6px;'>
                    <input id='{checkbox_id}' type='checkbox' checked onchange='{toggle_fn}()' />
                    Show Log
                </label>
                <button onclick='{copy_fn}()'>Copy Log</button>
            </div>
        </div>
        <div id='{body_id}'>
            <textarea id='{textarea_id}' readonly style='width:100%;min-height:120px;max-height:220px;resize:vertical;padding:8px;border:1px solid #c8d5e2;border-radius:8px;font-family:monospace;font-size:12px;'></textarea>
        </div>
    </div>"#,
        panel_id = params.panel_id,
        panel_style = params.panel_style,
        header_style = params.header_style,
        controls_style = params.controls_style,
        body_id = params.body_id,
        textarea_id = params.textarea_id,
        checkbox_id = params.checkbox_id,
        toggle_fn = params.toggle_fn,
        copy_fn = params.copy_fn,
    )
}

pub(super) fn render_update_dialog_html() -> &'static str {
    r#"<dialog id='updateAppDialog' class='updateAppDialog' style='padding:22px;border-radius:8px;border:1px solid #cbd5e1;max-width:560px;width:90%;box-sizing:border-box;box-shadow:0 10px 25px -5px rgba(0,0,0,0.1), 0 8px 10px -6px rgba(0,0,0,0.1);font-family:inherit;'>
    <div style='display:flex;flex-direction:column;gap:14px;'>
        <div style='display:flex;justify-content:space-between;align-items:center;'>
            <h3 style='margin:0;color:#1e3a8a;display:flex;align-items:center;gap:8px;font-size:17px;'>
                <span>✨</span> Software Update Available
            </h3>
            <button type='button' onclick='closeUpdateDialog()' style='background:none;border:none;font-size:20px;cursor:pointer;color:#64748b;line-height:1;'>&times;</button>
        </div>
        <div style='display:flex;align-items:center;gap:10px;background:#eff6ff;border:1px solid #bfdbfe;border-radius:6px;padding:10px 14px;'>
            <div>
                <span style='font-size:11px;color:#1e40af;font-weight:700;display:block;'>CURRENT</span>
                <span id='updateCurrentVer' style='font-weight:bold;color:#1e293b;font-size:13px;'>v1.15.0</span>
            </div>
            <span style='color:#3b82f6;font-weight:bold;font-size:16px;'>&rarr;</span>
            <div>
                <span style='font-size:11px;color:#15803d;font-weight:700;display:block;'>LATEST</span>
                <span id='updateLatestVer' style='font-weight:bold;color:#15803d;font-size:13px;'>v1.15.0</span>
            </div>
            <span id='updateBadgePill' style='margin-left:auto;background:#dcfce7;color:#166534;font-size:11px;font-weight:700;padding:2px 8px;border-radius:9999px;'>AVAILABLE</span>
        </div>
        <div id='updateReleaseNotesContainer' style='background:#f8fafc;border:1px solid #e2e8f0;border-radius:6px;padding:12px;max-height:180px;overflow-y:auto;'>
            <div style='font-size:12px;font-weight:600;color:#475467;margin-bottom:6px;'>Release Notes:</div>
            <div id='updateReleaseNotes' style='font-size:12px;color:#334155;line-height:1.45;white-space:pre-wrap;overflow-wrap:anywhere;'>Loading release notes...</div>
        </div>
        <div id='updateStatusBox' style='display:none;background:#f0fdf4;border:1px solid #bbf7d0;border-radius:6px;padding:12px;color:#166534;font-size:13px;'>
            <div id='updateStatusMsg' style='display:flex;align-items:center;gap:8px;'>
                <span>⏳</span> <span>Downloading update and verifying SHA256 checksum...</span>
            </div>
        </div>
        <div id='updateActionRow' style='display:flex;gap:10px;justify-content:flex-end;margin-top:6px;'>
            <button id='btnRemindLater' type='button' class='neutral' onclick='closeUpdateDialog()' style='padding:6px 14px;font-size:13px;border-radius:4px;border:1px solid #cbd5e1;background:#fff;cursor:pointer;'>Remind Me Later</button>
            <button id='btnDoUpdate' type='button' class='primary' onclick='executeApplicationUpdate()' style='padding:6px 14px;font-size:13px;border-radius:4px;background:#16a34a;color:#fff;border:1px solid #15803d;font-weight:600;cursor:pointer;'>Update &amp; Restart Now</button>
        </div>
    </div>
</dialog>"#
}

pub(super) const UPDATE_CLIENT_JS: &str = r#"
function checkAppUpdateStatus() {
    fetch('/api/update-status')
        .then(function(r) { return r.json(); })
        .then(function(data) {
            if (data && data.update_available) {
                window._updateInfo = data;
                var badge = document.getElementById('btnUpdateBadge');
                if (badge) {
                    badge.style.display = 'inline-block';
                    badge.textContent = '✨ Update to v' + data.latest_version + ' Available';
                }
                var banner = document.getElementById('updateBanner');
                if (banner) {
                    banner.style.display = 'block';
                    var bVer = document.getElementById('updateBannerVer');
                    if (bVer) bVer.textContent = 'v' + data.latest_version;
                }
            }
        })
        .catch(function() {
            // Silently ignore offline or connection errors
        });
}

function openUpdateDialog() {
    var dialog = document.getElementById('updateAppDialog');
    if (!dialog) return;
    var data = window._updateInfo;
    if (data) {
        var cur = document.getElementById('updateCurrentVer');
        if (cur) cur.textContent = 'v' + (data.current_version || '');
        var lat = document.getElementById('updateLatestVer');
        if (lat) lat.textContent = 'v' + (data.latest_version || '');
        var notes = document.getElementById('updateReleaseNotes');
        if (notes) notes.textContent = data.release_notes || 'Performance enhancements and bug fixes.';
    }
    if (typeof dialog.showModal === 'function') {
        dialog.showModal();
    } else {
        dialog.style.display = 'block';
    }
}

function closeUpdateDialog() {
    var dialog = document.getElementById('updateAppDialog');
    if (!dialog) return;
    if (typeof dialog.close === 'function') {
        dialog.close();
    } else {
        dialog.style.display = 'none';
    }
}

function executeApplicationUpdate() {
    var btn = document.getElementById('btnDoUpdate');
    var btnLater = document.getElementById('btnRemindLater');
    if (btn) btn.disabled = true;
    if (btnLater) btnLater.disabled = true;

    var statusBox = document.getElementById('updateStatusBox');
    var statusMsg = document.getElementById('updateStatusMsg');
    if (statusBox) statusBox.style.display = 'block';
    if (statusMsg) statusMsg.innerHTML = '<span>⏳</span> <span>Downloading update from GitHub and verifying SHA256 checksum...</span>';

    fetch('/api/install-update', { method: 'POST', headers: { 'Content-Type': 'application/json' } })
        .then(function(r) { return r.json(); })
        .then(function(data) {
            if (data.ok) {
                if (statusMsg) statusMsg.innerHTML = '<span>✅</span> <span>' + data.message + ' Restarting application...</span>';
                pollAndReloadAfterRestart();
            } else {
                if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">Update failed: ' + (data.message || 'unknown error') + '</span>';
                if (btn) btn.disabled = false;
                if (btnLater) btnLater.disabled = false;
            }
        })
        .catch(function(err) {
            if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">Request failed: ' + err + '</span>';
            if (btn) btn.disabled = false;
            if (btnLater) btnLater.disabled = false;
        });
}

function pollAndReloadAfterRestart() {
    var attempts = 0;
    var interval = setInterval(function() {
        attempts++;
        fetch('/api/ping')
            .then(function(r) {
                if (r.ok) {
                    clearInterval(interval);
                    window.location.reload();
                }
            })
            .catch(function() {
                // Server is restarting, keep waiting
            });
        if (attempts > 30) {
            clearInterval(interval);
            window.location.reload();
        }
    }, 1000);
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', checkAppUpdateStatus);
} else {
    checkAppUpdateStatus();
}
"#;

