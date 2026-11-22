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
        <div style='background:#f0fdf4;border:1px solid #bbf7d0;border-radius:6px;padding:9px 12px;font-size:12px;color:#166534;display:flex;align-items:center;gap:8px;'>
            <span style='font-size:16px;'>📘</span>
            <span>The latest Help Manual PDF (<code>StampAlbumGenerator2HelpManual.pdf</code>) will also be updated automatically.</span>
        </div>
        <div id='updateStatusBox' style='display:none;background:#f0fdf4;border:1px solid #bbf7d0;border-radius:6px;padding:12px;color:#166534;font-size:13px;'>
            <div id='updateStatusMsg' style='display:flex;align-items:center;gap:8px;'>
                <span>⏳</span> <span>Starting update...</span>
            </div>
            <progress id='updateProgressBar' value='0' max='100' style='display:none;width:100%;height:10px;border-radius:5px;overflow:hidden;margin-top:10px;'></progress>
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
    var progressBar = document.getElementById('updateProgressBar');
    if (statusBox) statusBox.style.display = 'block';
    if (progressBar) {
        progressBar.style.display = 'none';
        progressBar.value = 0;
    }
    if (statusMsg) statusMsg.innerHTML = '<span>⏳</span> <span>Starting update...</span>';

    fetch('/api/install-update', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({})
    })
    .then(function(r) { return r.json(); })
    .then(function(data) {
        if (!data.ok) {
            if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">Update failed to start: ' + (data.message || 'unknown error') + '</span>';
            if (btn) btn.disabled = false;
            if (btnLater) btnLater.disabled = false;
            return;
        }
        pollUpdateProgress();
    })
    .catch(function(err) {
        if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">Request failed: ' + err + '</span>';
        if (btn) btn.disabled = false;
        if (btnLater) btnLater.disabled = false;
    });
}

function pollUpdateProgress() {
    var statusMsg = document.getElementById('updateStatusMsg');
    var progressBar = document.getElementById('updateProgressBar');
    var btn = document.getElementById('btnDoUpdate');
    var btnLater = document.getElementById('btnRemindLater');

    var pollInterval = setInterval(function() {
        fetch('/api/update-status')
            .then(function(r) { return r.json(); })
            .then(function(data) {
                var p = data && data.progress;
                if (!p) return;

                if (p.state === 'downloading') {
                    if (progressBar) {
                        progressBar.style.display = 'block';
                        if (typeof p.percent === 'number') {
                            progressBar.value = p.percent;
                        }
                    }
                    if (statusMsg) statusMsg.innerHTML = '<span>⏳</span> <span>' + (p.message || 'Downloading update from GitHub...') + '</span>';
                } else if (p.state === 'verifying') {
                    if (progressBar) progressBar.style.display = 'none';
                    if (statusMsg) statusMsg.innerHTML = '<span>🔍</span> <span>Verifying SHA256 checksum...</span>';
                } else if (p.state === 'replacing') {
                    if (progressBar) progressBar.style.display = 'none';
                    if (statusMsg) statusMsg.innerHTML = '<span>📦</span> <span>Installing update...</span>';
                } else if (p.state === 'restarting') {
                    clearInterval(pollInterval);
                    if (progressBar) progressBar.style.display = 'none';
                    if (statusMsg) statusMsg.innerHTML = '<span>✅</span> <span>' + (p.message || 'Update installed and Help Manual PDF refreshed! Restarting application...') + '</span>';
                    pollAndReloadAfterRestart();
                } else if (p.state === 'failed') {
                    clearInterval(pollInterval);
                    if (progressBar) progressBar.style.display = 'none';
                    if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">' + (p.error || p.message || 'Update failed') + '</span>';
                    if (btn) btn.disabled = false;
                    if (btnLater) btnLater.disabled = false;
                }
            })
            .catch(function(err) {
                // Ignore transient network errors while polling
            });
    }, 500);
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

pub(super) fn render_config_staleness_dialog_html() -> &'static str {
    r#"<dialog id='configStaleDialog' class='configStaleDialog' style='padding:22px;border-radius:8px;border:1px solid #cbd5e1;max-width:580px;width:92%;box-sizing:border-box;box-shadow:0 10px 25px -5px rgba(0,0,0,0.1), 0 8px 10px -6px rgba(0,0,0,0.1);font-family:inherit;'>
    <div style='display:flex;flex-direction:column;gap:14px;'>
        <div style='display:flex;justify-content:space-between;align-items:center;'>
            <h3 style='margin:0;color:#1e3a8a;display:flex;align-items:center;gap:8px;font-size:17px;'>
                <span>⚙️</span> New Configuration Settings Available
            </h3>
            <button type='button' onclick='closeConfigStalenessDialog()' style='background:none;border:none;font-size:20px;cursor:pointer;color:#64748b;line-height:1;'>&times;</button>
        </div>

        <div style='background:#f0fdf4;border:1px solid #bbf7d0;border-radius:6px;padding:12px;display:flex;align-items:center;gap:12px;'>
            <div style='font-size:28px;'>📋</div>
            <div>
                <strong style='color:#166534;font-size:14px;' id='configStaleTitle'>New Settings Available</strong>
                <div style='font-size:12px;color:#15803d;margin-top:2px;' id='configStaleSubtext'>Your config.toml is missing options added in this version.</div>
            </div>
        </div>

        <div style='background:#f8fafc;border:1px solid #e2e8f0;border-radius:6px;padding:12px;'>
            <strong style='font-size:13px;color:#1e293b;display:flex;align-items:center;gap:6px;'>
                <span>🔒</span> Why the app never modifies config.toml automatically on launch
            </strong>
            <p style='margin:6px 0 0 0;font-size:12px;color:#475569;line-height:1.45;'>
                StampAlbumGen2 treats your <code>config.toml</code> as your personal file. It intentionally avoids silently editing or overwriting it on startup so that your custom album directories, margins, fonts, and personal preferences are never erased or altered.
            </p>
        </div>

        <div style='background:#f8fafc;border:1px solid #e2e8f0;border-radius:6px;padding:12px;'>
            <strong style='font-size:13px;color:#1e293b;display:flex;align-items:center;gap:6px;'>
                <span>💡</span> Built-in Defaults Apply Automatically
            </strong>
            <p style='margin:6px 0 0 0;font-size:12px;color:#475569;line-height:1.45;'>
                You do not need to do anything. Even though <code>config.toml</code> does not list these new options, StampAlbumGen2 automatically applies built-in default values in memory. The app runs completely normally without any configuration loss.
            </p>
        </div>

        <details style='font-size:12px;border:1px solid #e2e8f0;border-radius:6px;padding:8px 12px;background:#fff;'>
            <summary style='cursor:pointer;font-weight:600;color:#2b4c7e;'>
                <span id='configMissingCountSummary'>View missing options</span>
                <span style='font-weight:normal;color:#64748b;font-size:11px;'>(click to expand)</span>
            </summary>
            <div id='configMissingKeysList' style='margin-top:8px;max-height:120px;overflow-y:auto;font-family:monospace;font-size:11px;color:#334155;background:#f1f5f9;padding:8px;border-radius:4px;word-break:break-all;'></div>
        </details>

        <div id='configUpgradeStatusBox' style='display:none;background:#f0fdf4;border:1px solid #bbf7d0;border-radius:6px;padding:12px;color:#166534;font-size:13px;'>
            <div id='configUpgradeStatusMsg'></div>
        </div>

        <div style='display:flex;gap:10px;justify-content:flex-end;margin-top:6px;flex-wrap:wrap;'>
            <button type='button' class='neutral' onclick='dismissConfigStalenessDialog(true)' style='padding:7px 14px;font-size:13px;border-radius:4px;border:1px solid #cbd5e1;background:#fff;cursor:pointer;'>Keep Current Settings &amp; Don't Ask Again</button>
            <button type='button' class='secondary' onclick='openConfigEditorFromStaleness()' style='padding:7px 14px;font-size:13px;border-radius:4px;border:1px solid #2b4c7e;color:#2b4c7e;background:#fff;font-weight:600;cursor:pointer;'>Open Config Editor</button>
            <button id='btnDoConfigUpgrade' type='button' class='primary' onclick='executeConfigUpgrade()' style='padding:7px 14px;font-size:13px;border-radius:4px;background:#2563eb;color:#fff;border:1px solid #1d4ed8;font-weight:600;cursor:pointer;'>Update config.toml Now</button>
        </div>
    </div>
</dialog>"#
}

pub(super) const CONFIG_STALENESS_CLIENT_JS: &str = r#"
function checkConfigStalenessStatus() {
    fetch('/api/config-staleness')
        .then(function(r) { return r.json(); })
        .then(function(data) {
            if (data && data.stale) {
                window._configStalenessInfo = data;
                var badge = document.getElementById('btnConfigStaleBadge');
                if (badge) {
                    badge.style.display = 'inline-block';
                    badge.textContent = '⚙️ ' + data.missing_count + ' New Settings Available';
                }
                var dismissedKey = 'stamp_dismissed_config_staleness';
                var dismissedVal = data.current_version + '_' + data.missing_count;
                var alreadyDismissed = false;
                try {
                    alreadyDismissed = (localStorage.getItem(dismissedKey) === dismissedVal);
                } catch (e) {}

                if (!alreadyDismissed) {
                    openConfigStalenessDialog();
                }
            }
        })
        .catch(function() {
            // Silently ignore errors
        });
}

function openConfigStalenessDialog() {
    var dialog = document.getElementById('configStaleDialog');
    if (!dialog) return;
    var data = window._configStalenessInfo;
    if (data) {
        var count = data.missing_count || (data.missing_keys ? data.missing_keys.length : 0);
        var title = document.getElementById('configStaleTitle');
        if (title) title.textContent = count + ' New Setting' + (count === 1 ? '' : 's') + ' Available';
        var subtext = document.getElementById('configStaleSubtext');
        if (subtext) subtext.textContent = 'Your config.toml is missing ' + count + ' option(s) added in v' + (data.current_version || '') + '.';
        var summary = document.getElementById('configMissingCountSummary');
        if (summary) summary.textContent = 'View missing options (' + count + ')';
        var list = document.getElementById('configMissingKeysList');
        if (list && data.missing_keys) {
            list.textContent = data.missing_keys.join(', ');
        }
    }
    if (typeof dialog.showModal === 'function') {
        dialog.showModal();
    } else {
        dialog.style.display = 'block';
    }
}

function closeConfigStalenessDialog() {
    var dialog = document.getElementById('configStaleDialog');
    if (!dialog) return;
    if (typeof dialog.close === 'function') {
        dialog.close();
    } else {
        dialog.style.display = 'none';
    }
}

function dismissConfigStalenessDialog(dontAskAgain) {
    if (dontAskAgain && window._configStalenessInfo) {
        var data = window._configStalenessInfo;
        var dismissedKey = 'stamp_dismissed_config_staleness';
        var dismissedVal = data.current_version + '_' + data.missing_count;
        try {
            localStorage.setItem(dismissedKey, dismissedVal);
        } catch (e) {}
    }
    closeConfigStalenessDialog();
}

function openConfigEditorFromStaleness() {
    closeConfigStalenessDialog();
    if (typeof openConfigEditor === 'function') {
        openConfigEditor();
    }
}

function executeConfigUpgrade() {
    var btn = document.getElementById('btnDoConfigUpgrade');
    if (btn) btn.disabled = true;

    var statusBox = document.getElementById('configUpgradeStatusBox');
    var statusMsg = document.getElementById('configUpgradeStatusMsg');
    if (statusBox) statusBox.style.display = 'block';
    if (statusMsg) statusMsg.innerHTML = '<span>⏳</span> <span>Updating config.toml and creating timestamped backup...</span>';

    fetch('/api/config-upgrade', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({})
    })
    .then(function(r) { return r.json(); })
    .then(function(data) {
        if (!data.ok) {
            if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">' + (data.message || 'Upgrade failed') + '</span>';
            if (btn) btn.disabled = false;
            return;
        }
        if (statusMsg) statusMsg.innerHTML = '<span>✅</span> <span>' + (data.message || 'Successfully updated config.toml!') + (data.backup_path ? '<br/><small style="color:#475569;">Backup: ' + data.backup_path + '</small>' : '') + '<br/>Reloading...</span>';
        setTimeout(function() {
            window.location.reload();
        }, 1600);
    })
    .catch(function(err) {
        if (statusMsg) statusMsg.innerHTML = '<span>❌</span> <span style="color:#b91c1c;">Request failed: ' + err + '</span>';
        if (btn) btn.disabled = false;
    });
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', checkConfigStalenessStatus);
} else {
    checkConfigStalenessStatus();
}
"#;
