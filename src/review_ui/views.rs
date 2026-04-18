use super::{render_config_editor_assets, MatcherPayload};

const BACKEND_NOT_RESPONDING_MESSAGE: &str = "The Review UI backend is not responding. Automatic retries are in progress.   This might be caused by loading of files, especially if you have many colnect files to import .  You can wait for recovery or reload now.";

fn render_app_footer_html() -> String {
    let version = env!("CARGO_PKG_VERSION");
    // let build = option_env!("BUILD_NUMBER").unwrap_or("local");
    format!(
        "<div class='appFooter' style='margin-top:16px; font-size:12px; color:#5b6f81; text-align:center;'>\
<div>Copyright &copy; 2025 &#x2013; <span class='copyrightCurrentYear'></span> Laser Spider LLC. All rights reserved.</div>\
<div style='margin-top:4px;'>Album Generator 2 | Version {} </div>\
</div>",
        version
    )
}

pub(super) fn render_bootstrap_html(default_inbox_path: Option<&str>) -> String {
    let inbox_hint = default_inbox_path.unwrap_or("samples/colnect_inbox");
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset='utf-8'>
<title>Stamp Album Gen 2 Setup</title>
<link rel='icon' type='image/png' href='/favicon.png'>
<style>
:root {{
    --bg-top: #fdf5ec;
    --bg-bottom: #e9f2fb;
    --card-bg: #ffffff;
    --card-border: #cfddeb;
    --accent-main: #1f6fa8;
    --accent-ink: #14374f;
    --accent-warn: #a36a24;
    --accent-good: #2f8f63;
    --accent-danger: #a34545;
    --muted-text: #36556b;
}}
body {{ font-family: "Noto Sans", "DejaVu Sans", sans-serif; margin: 20px; background: radial-gradient(circle at 15% -20%, var(--bg-top) 0%, #f7f4ef 34%, var(--bg-bottom) 100%); color: var(--accent-ink); opacity: 0; animation: bootstrapFadeIn 180ms ease-out forwards; }}
.card {{ max-width: 980px; background: var(--card-bg); border: 1px solid var(--card-border); border-radius: 14px; padding: 18px; box-shadow: 0 14px 36px rgba(28, 56, 82, 0.12); }}
h1 {{ margin: 0 0 8px 0; }}
p {{ margin: 0 0 10px 0; color: var(--muted-text); }}
.row {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.badge {{ display: inline-block; border: 1px solid #c6d8ea; background: #f0f8ff; border-radius: 999px; padding: 2px 8px; font-size: 12px; }}
.panel {{ border: 1px solid #d9e6f1; border-left: 5px solid var(--accent-main); border-radius: 10px; padding: 12px; background: #f9fcff; margin-top: 12px; }}
.warn {{ border-left-color: var(--accent-warn); background: #fff8ee; }}
.ok {{ border-left-color: var(--accent-good); background: #f2fbf6; }}
.albumsRootPanel {{ border: 1px solid #d8e3ed; border-left: 5px solid #2f6fa3; border-radius: 10px; background: linear-gradient(180deg, #f4f9ff 0%, #f9fcff 100%); padding: 12px; margin-top: 12px; }}
.albumsRootHead {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.albumsRootHead strong {{ color: #214d72; }}
.albumsRootHint {{ margin: 0; color: #35566d; font-size: 12px; line-height: 1.45; }}
.albumsRootActions {{ margin-top: 8px; }}
.albumsRootStatus {{ margin-top: 8px; min-height: 24px; font-size: 12px; border-radius: 7px; padding: 6px 10px; border: 1px solid #cfe0f0; background: #eff6ff; color: #2f4f67; }}
.albumsRootStatus.ok {{ border-color: #b9dec9; background: #f2fbf6; color: #245641; }}
.albumsRootStatus.fail {{ border-color: #e8b4b4; background: #fff3f3; color: #7a2020; }}
label {{ font-size: 13px; font-weight: 600; }}
select, input {{ padding: 7px 8px; border: 1px solid #bcd1e4; border-radius: 8px; min-width: 170px; }}
button {{ padding: 7px 11px; border-radius: 8px; border: 1px solid #bcd1e4; background: #fff; cursor: pointer; transition: transform 120ms ease, box-shadow 120ms ease, background 120ms ease; }}
button.primary {{ background: var(--accent-main); color: #fff; border-color: var(--accent-main); }}
button.danger {{ background: var(--accent-danger); color: #fff; border-color: var(--accent-danger); }}
button.accent {{ background: #0f8a6b; color: #fff; border-color: #0f8a6b; }}
button:hover {{ background: #edf6ff; transform: translateY(-1px); box-shadow: 0 3px 10px rgba(21, 66, 95, 0.12); }}
code {{ background: #edf5fb; border: 1px solid #d4e2ee; border-radius: 6px; padding: 2px 6px; }}
.status {{ margin-top: 10px; min-height: 18px; font-size: 13px; color: #35566d; }}
.loadingBox {{ display: none; margin-top: 12px; padding: 12px; border: 1px solid #d7e2eb; border-left: 5px solid #2f6fa3; border-radius: 10px; background: #f5fafe; }}
.spinner {{ width: 16px; height: 16px; border: 3px solid #d7e2eb; border-top-color: #2f6fa3; border-radius: 50%; display: inline-block; animation: spin 0.9s linear infinite; }}
.loadingRow {{ display: flex; align-items: center; gap: 10px; }}
.dots {{ display: inline-block; min-width: 18px; text-align: left; }}
body.busy {{ cursor: progress; }}
body.busy button, body.busy select, body.busy input {{ opacity: 0.65; pointer-events: none; }}
.reconnectBanner {{ display: none; margin-bottom: 12px; border: 1px solid #ddc79f; border-left: 5px solid #9b6a20; border-radius: 10px; background: #fff7e8; color: #694314; padding: 10px 12px; }}
.reconnectBanner.show {{ display: block; }}
.reconnectBanner.recovered {{ border-color: #b9dec9; border-left-color: #2f8f63; background: #f4fcf8; color: #245641; }}
.reconnectBanner .title {{ font-weight: 700; margin-bottom: 4px; }}
.reconnectBanner .meta {{ font-size: 12px; color: #5e7283; }}
.reconnectBanner .actions {{ display: flex; gap: 8px; margin-top: 8px; flex-wrap: wrap; }}
@keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@keyframes bootstrapFadeIn {{ from {{ opacity: 0; transform: translateY(2px); }} to {{ opacity: 1; transform: translateY(0); }} }}
.introPanel {{ border: 1px solid #cfe3f5; border-radius: 10px; background: linear-gradient(135deg,#f6fbff 0%,#eef5fd 100%); margin: 0 0 14px 0; overflow: hidden; }}
.introPanel summary {{ display: flex; align-items: center; gap: 8px; padding: 10px 14px; cursor: pointer; font-weight: 600; font-size: 14px; color: var(--accent-main); list-style: none; user-select: none; border-radius: 10px; transition: background 140ms; }}
.introPanel summary::-webkit-details-marker {{ display: none; }}
.introPanel summary:hover {{ background: rgba(31,111,168,0.07); }}
.introPanel summary .introArrow {{ font-size: 10px; transition: transform 200ms; flex-shrink: 0; }}
details.introPanel:not([open]) summary .introArrow {{ transform: rotate(-90deg); }}
.introPanel summary .introDimTag {{ font-size: 11px; font-weight: 400; color: var(--muted-text); margin-left: auto; }}
.introPanelBody {{ padding: 2px 14px 12px 38px; }}
.introPanelBody p {{ margin: 4px 0 8px 0; font-size: 13.5px; }}
.introPanelBody a {{ color: var(--accent-main); }}
.hintRow {{ display: flex; gap: 10px; align-items: flex-start; margin: 7px 0; padding: 8px 10px; background: rgba(255,255,255,0.75); border-radius: 7px; border: 1px solid #ddeef9; font-size: 13px; color: var(--muted-text); }}
.hintIcon {{ font-size: 15px; flex-shrink: 0; margin-top: 1px; }}
.colnectCopyPanel {{ border: 1px solid #d9e6f1; border-left: 5px solid #7b4ea3; border-radius: 10px; padding: 12px; background: #fdf8ff; margin-top: 12px; }}
.colnectCopyLog {{ margin-top: 8px; max-height: 130px; overflow-y: auto; font-size: 11px; font-family: ui-monospace, monospace; background: #f4f0fa; border: 1px solid #d5c8ea; border-radius: 6px; padding: 5px 7px; }}
.colnectCopyStatus {{ margin-top: 8px; font-size: 13px; padding: 6px 10px; border-radius: 6px; background: #ede8f8; border: 1px solid #c9baee; min-height: 26px; }}
.colnectCopyStatus.ok {{ background: #f2fbf6; border-color: #b9dec9; color: #245641; }}
.colnectCopyStatus.fail {{ background: #fff3f3; border-color: #e8b4b4; color: #7a2020; }}
#colnectDirPath {{ display: block; max-width: 100%; white-space: pre-wrap; overflow-wrap: anywhere; word-break: break-word; }}
#colnectCopyLog .line {{ border-bottom: 1px solid #e4dff1; padding: 2px 0; white-space: pre-wrap; word-break: break-word; }}
#colnectCopyLog .line:last-child {{ border-bottom: none; }}
</style>
</head>
<body>
<div class='card'>
    <div id='reconnectBanner' class='reconnectBanner' role='status' aria-live='polite'>
        <div id='reconnectBannerTitle' class='title'>Connection lost, retrying...</div>
        <div id='reconnectBannerText'>{backend_not_responding_message}</div>
        <div class='actions'>
            <button type='button' onclick='retryBackendHealthCheck()'>Retry Now</button>
            <button type='button' onclick='reloadCurrentPage()'>Reload Page</button>
            <span id='reconnectBannerMeta' class='meta'></span>
        </div>
    </div>
    <h1>Stamp Album Generator 2</h1>
    <h2>Start Review UI</h2>
    <details id='introPanel' class='introPanel' open>
      <summary><span class='introArrow'>&#9660;</span> About this app &amp; setup hints <span class='introDimTag'>click to expand / collapse</span></summary>
      <div class='introPanelBody'>
        <p><strong>Stamp Album Generation</strong> startup. Pick a country to open its Review UI.</p>
        <p>This program could not be completed without the incredible Album Easy program by Bundu Technology Ltd. <a href='https://www.thestampweb.com/albumeasy' target='_blank' rel='noopener noreferrer'>AlbumEasy</a></p>
        <div class='hintRow'><span class='hintIcon'>&#128193;</span><span>If you have not yet set up your Albums Root folder, you can do so now or let it auto-detect on launch. The Albums Root is where the generated albums and related data will be stored. You can change this later in the config editor.</span></div>
        <div class='hintRow'><span class='hintIcon'>&#128236;</span><span>If you have a Colnect inbox export and want to use it for the review, you can import it now or later from the config editor. The default expected format is the &ldquo;Export for AlbumEasy&rdquo; option from Colnect.</span></div>
      </div>
    </details>
    <div class='row'>
        <label for='countrySel'>Country</label>
        <select id='countrySel' onchange='onCountrySelectionChanged()'></select>
        <button onclick='refreshState()'>Refresh</button>
        <button class='primary' onclick='startReview()'>Open Review</button>
        <button onclick='generateAlbumPdfNow()'>Generate PDF Now</button>
        <button onclick='importColnectFiles()'>Import Colnect</button>
        <button class='danger' onclick='logoutApp()'>Logout</button>
    </div>
    <div class='albumsRootPanel'>
        <div class='albumsRootHead'>
            <strong>Albums Root</strong>
            <span id='albumsRootResolved' class='badge'>Albums root: auto</span>
        </div>
        <p class='albumsRootHint'>Select your Albums root folder (usually named <code>Albums</code>). All startup actions in this page use that location.</p>
        <div class='row albumsRootActions'>
            <label for='albumsRootPath'>Folder</label>
            <input id='albumsRootPath' type='text' placeholder='Type Albums path or pick with folder chooser' style='min-width:420px;' />
            <button onclick='browseAlbumsRootPath()'>&#128193; Pick Albums Root...</button>
            <button class='primary' onclick='setAlbumsRootPath()'>&#10003; Apply</button>
            <button onclick='clearAlbumsRootPath()'>&#8635; Reset</button>
        </div>
        <div id='albumsRootStatus' class='albumsRootStatus'>Ready</div>
    </div>
    <div class='row'>
        <label for='countrySearch'>Search</label>
        <input id='countrySearch' type='text' placeholder='Type to search or enter a new country...' list='countrySearchList' oninput='updateCountrySearchMatches(true)' onchange='applyCountrySearchSelection()' />
        <datalist id='countrySearchList'></datalist>
        <button onclick='stepCountrySearch(-1)'>Back</button>
        <button onclick='stepCountrySearch(1)'>Forward</button>
        <span id='countrySearchStatus' class='badge'>Matches: 0/0</span>
    </div>
    <div class='row' style='align-items:center;gap:0.5em;flex-wrap:wrap;'>
        <label for='pdfStartYear' style='margin:0;'>PDF Year Range</label>
        <input id='pdfStartYear' type='number' placeholder='Start year' min='1840' max='2100' style='width:100px'/>
        <span style='font-size:0.9em;'>to</span>
        <input id='pdfEndYear' type='number' placeholder='End year' min='1840' max='2100' style='width:100px'/>
        <span style='font-size:0.82em;color:#666;'>(leave blank = all years)</span>
        <span id='countryCount' class='badge'>Countries: 0</span>
    </div>

    <div id='countryPanel' class='panel'></div>
    <div id='stampworldPanel' class='panel warn' style='display:none;'>
        <strong id='swPanelLead'>StampWorld pull is available for this country.</strong>
        <p id='swPanelHint' style='margin-top:6px;'>Use this to pull additional StampWorld data for existing countries or create a new country.</p>
        <div class='row'>
            <label>StampWorld Country</label>
            <input id='swCountry' type='text' />
            <label>Start Year</label>
            <select id='swStartYear'></select>
            <label>End Year</label>
            <select id='swEndYear'></select>
        </div>
        <div class='row'>
            <label>Command Preset</label>
            <select id='swPreset'>
                <option value='basic'>Basic Pull</option>
                <option value='overwrite_album'>Overwrite Album Output</option>
                <option value='overwrite_all'>Overwrite Album + Images</option>
                <option value='bounded_pages'>Limit Pages (fast test)</option>
                <option value='accept_typo'>Auto Accept Source Typos</option>
            </select>
            <label>Extra Flags</label>
            <input id='swExtraFlags' type='text' placeholder='--max-pages 20 --stop-after-no-hits 5' style='min-width:290px;' />
        </div>
        <div class='row'>
            <span class='badge'>Colnect inbox hint: {inbox_hint}</span>
        </div>
        <div class='row'>
            <button class='accent' onclick='runStampWorldPull()'>Run StampWorld Pull</button>
            <button id='swOpenReviewBtn' class='primary' onclick='openReviewAfterStampWorld()' style='display:none;'>Open Review</button>
            <span id='swRunStatus' class='badge'>Not started</span>
        </div>
        <div class='row'>
            <strong>Suggested command:</strong>
            <code id='swCmd'></code>
            <button onclick='copySuggestedCommand()'>Copy</button>
        </div>
        <div class='row' style='font-size:12px;color:#37566d;'>
            <span>AlbumEasy not set? Add CLI path here or install from</span>
            <a href='https://www.thestampweb.com/albumeasy/download' target='_blank' rel='noopener noreferrer'>AlbumEasy download</a>
            <button onclick='configureAlbumeasyPath()'>Set AlbumEasy Path</button>
        </div>
        <div id='swPullLogPanel' style='display:none;margin-top:10px;border:1px solid #e3c27e;border-radius:6px;padding:10px;background:#fffbf0;'>
            <div style='font-size:13px;font-weight:600;color:#37566d;margin-bottom:8px;'>StampWorld Pull Progress: Real-Time Log</div>
            <textarea id='swPullLog' readonly style='width:100%;min-height:150px;max-height:280px;resize:vertical;padding:8px;border:1px solid #d7b896;border-radius:6px;font-family:monospace;font-size:11px;background:#fffcf7;color:#333;overflow-y:auto;overflow-x:hidden;'></textarea>
        </div>
    </div>
    <div id='colnectCopyPanel' class='colnectCopyPanel'>
        <strong>Import Colnect Files</strong>
        <div class='row' style='margin-top:8px;'>
            <button id='colnectPickBtn' class='accent' onclick='pickAndCopyColnectFile()'>Browse &amp; Copy Colnect File&#8230;</button>
            <span id='colnectCopyCountBadge' class='badge'>Files: 0</span>
        </div>
        <div style='margin-top:6px;font-size:12px;color:var(--muted-text);'>
            Destination: <code id='colnectDirPath'>(select a country)</code>
        </div>
        <div id='colnectCopyStatus' class='colnectCopyStatus'>Ready</div>
        <div id='colnectCopyLog' class='colnectCopyLog'></div>
    </div>
    <div id='status' class='status'></div>
    <div id='runtimeLogPanel' style='margin-top:10px;border:1px solid #d7e2eb;border-radius:10px;padding:10px;background:#fbfdff;'>
        <div class='row' style='margin-top:0;justify-content:space-between;'>
            <strong>Runtime Log</strong>
            <div class='row' style='margin-top:0;gap:8px;'>
                <label style='font-size:12px;font-weight:600;display:flex;align-items:center;gap:6px;'>
                    <input id='showRuntimeLog' type='checkbox' checked onchange='toggleRuntimeLogVisibility()' />
                    Show Log
                </label>
                <button onclick='copyRuntimeLog()'>Copy Log</button>
            </div>
        </div>
        <div id='runtimeLogBody'>
            <textarea id='runtimeLog' readonly style='width:100%;min-height:120px;max-height:220px;resize:vertical;padding:8px;border:1px solid #c8d5e2;border-radius:8px;font-family:monospace;font-size:12px;'></textarea>
        </div>
    </div>
    <div id='loadingBox' class='loadingBox'>
        <div class='loadingRow'>
            <span class='spinner'></span>
            <div>
                <strong>Preparing Review UI<span id='loadingDotsTitle' class='dots'></span></strong><br/>
                <span id='loadingText'>Loading country data and opening the compare view. This may take a few seconds.</span>
            </div>
        </div>
    </div>
</div>

<script>
let countries = [];
let loadingDotsTimer = null;
let loadingPhaseTimer = null;
let bootstrapPdfRequestInFlight = false;
let countrySearchMatches = [];
let countrySearchIndex = -1;
let runtimeLogTimer = null;
let backendHealthTimer = null;
let backendHealthFailures = 0;
let backendHealthCheckInFlight = false;
let backendMarkedDisconnected = false;
let backendRecoveredReloadPending = false;
let stampworldRunPollTimer = null;
let nativeDialogInFlight = false;
const runtimeLogVisibleStorageKey = 'reviewui.bootstrap.showRuntimeLog';
const introPanelStorageKey = 'reviewui.bootstrap.introPanel';
const backendHealthPollMs = 15000;
(function() {{
  try {{
    const ip = document.getElementById('introPanel');
    if (ip) {{
      if (localStorage.getItem(introPanelStorageKey) === 'closed') ip.removeAttribute('open');
      ip.addEventListener('toggle', function() {{
        try {{ localStorage.setItem(introPanelStorageKey, ip.open ? 'open' : 'closed'); }} catch(_) {{}}
      }});
    }}
  }} catch(_) {{}}
}})();
const backendHealthFailureThreshold = 2;

function reloadCurrentPage() {{
    window.location.reload();
}}

function setReconnectBanner(visible, title, text, recovered) {{
    const banner = document.getElementById('reconnectBanner');
    const titleEl = document.getElementById('reconnectBannerTitle');
    const textEl = document.getElementById('reconnectBannerText');
    const metaEl = document.getElementById('reconnectBannerMeta');
    if (!banner || !titleEl || !textEl || !metaEl) return;
    titleEl.innerText = title || 'Connection lost, retrying...';
    textEl.innerText = text || '{backend_not_responding_message}';
    metaEl.innerText = visible ? ('Checks every ' + Math.round(backendHealthPollMs / 1000) + 's') : '';
    banner.classList.toggle('show', !!visible);
    banner.classList.toggle('recovered', !!recovered);
}}

async function fetchBackendHealth() {{
    const controller = typeof AbortController !== 'undefined' ? new AbortController() : null;
    const timeoutId = controller ? setTimeout(() => controller.abort(), 4000) : null;
    try {{
        const res = await fetch('/health', {{ cache: 'no-store', signal: controller ? controller.signal : undefined }});
        if (!res.ok) throw new Error('HTTP ' + res.status);
        const j = await res.json();
        if (!j || j.ok !== true) throw new Error('invalid health response');
        return j;
    }} finally {{
        if (timeoutId) clearTimeout(timeoutId);
    }}
}}

async function checkBackendHealth() {{
    if (nativeDialogInFlight) return;
    if (backendHealthCheckInFlight) return;
    backendHealthCheckInFlight = true;
    try {{
        await fetchBackendHealth();
        backendHealthFailures = 0;
        if (backendMarkedDisconnected && !backendRecoveredReloadPending) {{
            backendRecoveredReloadPending = true;
            setReconnectBanner(true, 'Connection restored', 'The backend is responding again. Reloading this page now.', true);
            setTimeout(() => window.location.reload(), 900);
        }} else if (!backendRecoveredReloadPending) {{
            setReconnectBanner(false, '', '', false);
        }}
        backendMarkedDisconnected = false;
    }} catch (_) {{
        backendHealthFailures += 1;
        if (backendHealthFailures >= backendHealthFailureThreshold) {{
            backendMarkedDisconnected = true;
            backendRecoveredReloadPending = false;
            setReconnectBanner(
                true,
                'Connection lost, retrying...',
                '{backend_not_responding_message}',
                false
            );
        }}
    }} finally {{
        backendHealthCheckInFlight = false;
    }}
}}

function startBackendHealthPolling() {{
    if (backendHealthTimer) clearInterval(backendHealthTimer);
    checkBackendHealth();
    backendHealthTimer = setInterval(checkBackendHealth, backendHealthPollMs);
    window.addEventListener('online', checkBackendHealth);
    document.addEventListener('visibilitychange', () => {{
        if (!document.hidden) checkBackendHealth();
    }});
}}

function retryBackendHealthCheck() {{
    if (nativeDialogInFlight) return;
    checkBackendHealth();
}}

function beginNativeDialog(message) {{
    nativeDialogInFlight = true;
    backendHealthFailures = 0;
    backendMarkedDisconnected = false;
    backendRecoveredReloadPending = false;
    setReconnectBanner(false, '', '', false);
    setAlbumsRootStatus(message || 'Waiting for native folder picker...');
    const status = document.getElementById('status');
    if (status) status.innerText = message || 'Waiting for native folder picker...';
}}

function endNativeDialog() {{
    nativeDialogInFlight = false;
    backendHealthFailures = 0;
    setReconnectBanner(false, '', '', false);
}}

function isRuntimeLogVisible() {{
    const cb = document.getElementById('showRuntimeLog');
    return cb ? !!cb.checked : true;
}}

function applyRuntimeLogVisibility() {{
    const body = document.getElementById('runtimeLogBody');
    if (!body) return;
    body.style.display = isRuntimeLogVisible() ? '' : 'none';
}}

function toggleRuntimeLogVisibility() {{
    applyRuntimeLogVisibility();
    try {{
        localStorage.setItem(runtimeLogVisibleStorageKey, isRuntimeLogVisible() ? '1' : '0');
    }} catch (_) {{}}
}}

function copyTextToClipboard(text) {{
    if (navigator.clipboard && navigator.clipboard.writeText) {{
        return navigator.clipboard.writeText(text);
    }}
    return new Promise((resolve, reject) => {{
        try {{
            const ta = document.createElement('textarea');
            ta.value = text;
            ta.style.position = 'fixed';
            ta.style.left = '-9999px';
            document.body.appendChild(ta);
            ta.focus();
            ta.select();
            document.execCommand('copy');
            document.body.removeChild(ta);
            resolve();
        }} catch (e) {{
            reject(e);
        }}
    }});
}}

function ensureCopyableAlertDialog() {{
    if (document.getElementById('copyAlertDialog')) return;
    const dlg = document.createElement('dialog');
    dlg.id = 'copyAlertDialog';
    dlg.style.maxWidth = '760px';
    dlg.style.width = '92vw';
    dlg.innerHTML =
        "<div style='display:flex;flex-direction:column;gap:10px;'>" +
        "<h3 id='copyAlertTitle' style='margin:0;'>Message</h3>" +
        "<textarea id='copyAlertText' readonly style='width:100%;min-height:180px;resize:vertical;padding:8px;border:1px solid #c8d5e2;border-radius:8px;font-family:monospace;font-size:12px;'></textarea>" +
        "<div style='display:flex;gap:8px;justify-content:flex-end;'>" +
        "<button id='copyAlertCopyBtn'>Copy</button>" +
        "<button id='copyAlertCloseBtn'>Close</button>" +
        "</div></div>";
    document.body.appendChild(dlg);
    dlg.querySelector('#copyAlertCloseBtn').addEventListener('click', () => dlg.close());
    dlg.querySelector('#copyAlertCopyBtn').addEventListener('click', async () => {{
        const text = dlg.querySelector('#copyAlertText')?.value || '';
        try {{
            await copyTextToClipboard(text);
        }} catch (_) {{}}
    }});
}}

function showCopyableAlert(message, title) {{
    ensureCopyableAlertDialog();
    const dlg = document.getElementById('copyAlertDialog');
    const titleEl = document.getElementById('copyAlertTitle');
    const textEl = document.getElementById('copyAlertText');
    if (!dlg || !textEl || !titleEl) return;
    titleEl.innerText = title || 'Message';
    textEl.value = String(message || '');
    if (!dlg.open) dlg.showModal();
    textEl.focus();
    textEl.select();
}}

window.alert = function(message) {{
    showCopyableAlert(message, 'Message');
}};

async function refreshRuntimeLog() {{
    try {{
        if (nativeDialogInFlight) return;
        if (!isRuntimeLogVisible()) return;
        const r = await fetch('/runtime-log');
        const j = await r.json();
        const box = document.getElementById('runtimeLog');
        if (!box) return;
        const lines = Array.isArray(j?.lines) ? j.lines : [];
        const atBottom = (box.scrollTop + box.clientHeight + 20) >= box.scrollHeight;
        box.value = lines.join('\n');
        if (atBottom) box.scrollTop = box.scrollHeight;
    }} catch (_) {{}}
}}

function startRuntimeLogPolling() {{
    if (runtimeLogTimer) clearInterval(runtimeLogTimer);
    refreshRuntimeLog();
    runtimeLogTimer = setInterval(refreshRuntimeLog, 1200);
}}

async function copyRuntimeLog() {{
    const box = document.getElementById('runtimeLog');
    const status = document.getElementById('status');
    if (!box) return;
    try {{
        await copyTextToClipboard(box.value || '');
        if (status) status.innerText = 'Runtime log copied to clipboard.';
    }} catch (e) {{
        if (status) status.innerText = 'Failed to copy runtime log: ' + e;
    }}
}}

function setCountrySearchStatus(text) {{
    const status = document.getElementById('countrySearchStatus');
    if (status) status.innerText = text;
}}

function updateAlbumsRootHint(path) {{
    const badge = document.getElementById('albumsRootResolved');
    if (!badge) return;
    const display = String(path || '').trim();
    badge.innerText = display ? ('Albums root: ' + display) : 'Albums root: auto';
}}

function setAlbumsRootStatus(text, tone) {{
    const status = document.getElementById('albumsRootStatus');
    if (!status) return;
    status.className = 'albumsRootStatus';
    if (tone === 'ok') status.classList.add('ok');
    if (tone === 'fail') status.classList.add('fail');
    status.innerText = text || 'Ready';
}}

async function applyAlbumsRootResponse(j) {{
    const input = document.getElementById('albumsRootPath');
    const status = document.getElementById('status');
    updateAlbumsRootHint(j.albums_root || '');
    if (input) input.value = j.albums_root || '';
    setAlbumsRootStatus(
        j.message || (j.ok ? 'Albums root updated.' : 'Failed to update Albums root.'),
        j.ok ? 'ok' : 'fail'
    );
    if (status) status.innerText = j.message || (j.ok ? 'Albums root updated.' : 'Failed to update Albums root.');
    if (j.ok && j.picked !== false) await refreshState();
}}

async function browseAlbumsRootPath() {{
    const status = document.getElementById('status');
    beginNativeDialog('Opening native folder picker...');
    try {{
        const r = await fetch('/bootstrap-pick-albums-root', {{ method: 'POST' }});
        const j = await r.json();
        await applyAlbumsRootResponse(j);
    }} catch (e) {{
        setAlbumsRootStatus('Failed to open folder picker: ' + e, 'fail');
        if (status) status.innerText = 'Failed to open folder picker: ' + e;
    }} finally {{
        endNativeDialog();
    }}
}}

async function setAlbumsRootPath() {{
    const input = document.getElementById('albumsRootPath');
    const path = String(input?.value || '').trim();

    if (!path) {{
        setAlbumsRootStatus('Enter a folder path or click Pick Albums Root first.', 'fail');
        const status = document.getElementById('status');
        if (status) status.innerText = 'Enter a folder path or click Pick Albums Root first.';
        return;
    }}

    const status = document.getElementById('status');
    setAlbumsRootStatus('Applying Albums root...');
    try {{
        const r = await fetch('/bootstrap-set-albums-root', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ path }})
        }});
        const j = await r.json();
        await applyAlbumsRootResponse(j);
    }} catch (e) {{
        setAlbumsRootStatus('Failed to update Albums root: ' + e, 'fail');
        if (status) status.innerText = 'Failed to update Albums root: ' + e;
    }}
}}

async function clearAlbumsRootPath() {{
    const input = document.getElementById('albumsRootPath');
    if (input) input.value = '';
    const status = document.getElementById('status');
    setAlbumsRootStatus('Switching to auto-detect...');
    try {{
        const r = await fetch('/bootstrap-set-albums-root', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ path: '' }})
        }});
        const j = await r.json();
        await applyAlbumsRootResponse(j);
    }} catch (e) {{
        setAlbumsRootStatus('Failed to clear Albums root override: ' + e, 'fail');
        if (status) status.innerText = 'Failed to clear Albums root override: ' + e;
    }}
}}

function onCountrySelectionChanged() {{
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    if (searchInput && sel) searchInput.value = sel.value || '';
    renderCountryPanel();
    updateCountrySearchMatches(true);
}}

function refreshCountrySearchDatalist(query) {{
    const list = document.getElementById('countrySearchList');
    if (!list) return;
    const q = String(query || '').trim().toLowerCase();
    const source = q
        ? countries.filter(c => String(c.country || '').toLowerCase().includes(q))
        : countries;
    const top = source.slice(0, 40).map(c => c.country);
    list.innerHTML = top.map(name => `<option value="${{name}}"></option>`).join('');
}}

function applyCountrySearchSelection() {{
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    const status = document.getElementById('status');
    if (!sel || !searchInput) return;

    const typed = String(searchInput.value || '').trim();
    if (!typed) return;

    const exact = countries.find(c => String(c.country || '').toLowerCase() === typed.toLowerCase());
    if (exact) {{
        sel.value = exact.country;
        renderCountryPanel();
        updateCountrySearchMatches(true);
        if (status) status.innerText = `Selected ${{exact.country}}.`;
        return;
    }}

    const swCountry = document.getElementById('swCountry');
    if (swCountry) swCountry.value = typed;
    document.getElementById('stampworldPanel').style.display = '';
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(typed);
    setCountrySearchStatus('Matches: 0/0');
    if (status) status.innerText = `No exact DB match for "${{typed}}". You can run StampWorld pull for this typed country.`;
}}

function updateCountrySearchMatches(preserveCurrentSelection) {{
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    if (!sel || !searchInput) return;

    const query = String(searchInput.value || '').trim().toLowerCase();
    refreshCountrySearchDatalist(query);
    if (!query) {{
        countrySearchMatches = countries.map(c => c.country);
    }} else {{
        countrySearchMatches = countries
            .filter(c => String(c.country || '').toLowerCase().includes(query))
            .map(c => c.country);
    }}

    if (!countrySearchMatches.length) {{
        countrySearchIndex = -1;
        setCountrySearchStatus('Matches: 0/0');
        return;
    }}

    if (preserveCurrentSelection) {{
        const selectedCountry = String(sel.value || '');
        const selectedIdx = countrySearchMatches.findIndex(c => c === selectedCountry);
        if (selectedIdx >= 0) {{
            countrySearchIndex = selectedIdx;
            setCountrySearchStatus(`Matches: ${{countrySearchIndex + 1}}/${{countrySearchMatches.length}}`);
            return;
        }}
    }}

    countrySearchIndex = 0;
    sel.value = countrySearchMatches[0];
    renderCountryPanel();
    setCountrySearchStatus(`Matches: 1/${{countrySearchMatches.length}}`);
}}

function stepCountrySearch(delta) {{
    const sel = document.getElementById('countrySel');
    if (!sel) return;

    if (!countrySearchMatches.length) {{
        setCountrySearchStatus('Matches: 0/0');
        return;
    }}

    if (countrySearchIndex < 0) countrySearchIndex = 0;
    const total = countrySearchMatches.length;
    countrySearchIndex = (countrySearchIndex + delta + total) % total;
    sel.value = countrySearchMatches[countrySearchIndex];
    const searchInput = document.getElementById('countrySearch');
    if (searchInput) searchInput.value = sel.value;
    renderCountryPanel();
    setCountrySearchStatus(`Matches: ${{countrySearchIndex + 1}}/${{total}}`);
}}

function startLoadingPhases(country) {{
    const txt = document.getElementById('loadingText');
    if (!txt) return;

    if (loadingPhaseTimer) {{
        clearInterval(loadingPhaseTimer);
        loadingPhaseTimer = null;
    }}

    const phases = [
        `Preparing data from ${{country}}`,
        'Opening database...',
        'Syncing processed Colnect candidates...',
        'Loading left StampWorld rows...',
        'Loading right Colnect candidates...',
        'Starting local web server...',
        'Opening compare view in browser...'
    ];

    let idx = 0;
    txt.innerText = phases[idx];
    loadingPhaseTimer = setInterval(() => {{
        idx = (idx + 1) % phases.length;
        txt.innerText = phases[idx];
    }}, 900);
}}

function stopLoadingPhases() {{
    if (loadingPhaseTimer) {{
        clearInterval(loadingPhaseTimer);
        loadingPhaseTimer = null;
    }}
}}

function setMovingDots(active) {{
    const dots = document.getElementById('loadingDotsTitle');
    if (!dots) return;

    if (!active) {{
        if (loadingDotsTimer) {{
            clearInterval(loadingDotsTimer);
            loadingDotsTimer = null;
        }}
        stopLoadingPhases();
        dots.innerText = '';
        return;
    }}

    if (loadingDotsTimer) return;
    let idx = 0;
    const frames = ['', '.', '..', '...'];
    loadingDotsTimer = setInterval(() => {{
        idx = (idx + 1) % frames.length;
        dots.innerText = frames[idx];
    }}, 260);
}}

function setBusy(isBusy, message) {{
    document.body.classList.toggle('busy', isBusy);
    const box = document.getElementById('loadingBox');
    const txt = document.getElementById('loadingText');
    if (txt && message) txt.innerText = message;
    if (box) box.style.display = isBusy ? 'block' : 'none';
    setMovingDots(isBusy);
}}

function slugCountry(name) {{
    return String(name || '').trim().replaceAll(' ', '-');
}}

function populateYearSelect(id, startYear, endYear, defaultYear) {{
    const sel = document.getElementById(id);
    if (!sel) return;
    const options = [];
    for (let y = startYear; y <= endYear; y++) {{
        options.push(`<option value="${{y}}">${{y}}</option>`);
    }}
    sel.innerHTML = options.join('');
    const clampedDefault = Math.min(Math.max(defaultYear, startYear), endYear);
    sel.value = String(clampedDefault);
}}

function initYearSelectors() {{
    const minYear = 1840;
    const maxYear = new Date().getFullYear();
    populateYearSelect('swStartYear', minYear, maxYear, 1840);
    populateYearSelect('swEndYear', minYear, maxYear, 2026);
}}

function stampworldPresetFlags() {{
    const preset = document.getElementById('swPreset')?.value || 'basic';
    if (preset === 'overwrite_album') return '--overwrite-album';
    if (preset === 'overwrite_all') return '--overwrite-album --overwrite-images';
    if (preset === 'bounded_pages') return '--max-pages 25 --stop-after-no-hits 3';
    if (preset === 'accept_typo') return '--yes';
    return '';
}}

function suggestedStampworldCommand(country) {{
    const c = document.getElementById('swCountry').value || country || '';
    const y1 = document.getElementById('swStartYear').value || '1840';
    const y2 = document.getElementById('swEndYear').value || '2026';
    const presetFlags = stampworldPresetFlags();
    const extraFlags = (document.getElementById('swExtraFlags')?.value || '').trim();
    const suffix = [presetFlags, extraFlags].filter(Boolean).join(' ');
    return `./stamp_album_gen2 --source stampworld --site-name "${{c}}" --album-name "${{c}}" --start-year ${{y1}} --end-year ${{y2}}${{suffix ? ' ' + suffix : ''}}`;
}}

function updateStampWorldRunStatus(text, isRunning, canOpenReview) {{
    const badge = document.getElementById('swRunStatus');
    const openBtn = document.getElementById('swOpenReviewBtn');
    if (badge) badge.innerText = text || 'Not started';
    if (openBtn) openBtn.style.display = canOpenReview ? '' : 'none';
    if (isRunning) {{
        setBusy(true, 'Running StampWorld pull...');
    }}
}}

async function pollStampWorldStatus() {{
    try {{
        const r = await fetch('/api/stampworld-status');
        const j = await r.json();
        if (!j.ok) return;
        if (j.running) {{
            updateStampWorldRunStatus(j.message || 'StampWorld pull running...', true, false);
            
            // Fetch and display real-time log
            try {{
                const logR = await fetch('/api/stampworld-log-tail');
                const logJ = await logR.json();
                if (logJ.ok && logJ.lines && logJ.lines.length > 0) {{
                    const logPanel = document.getElementById('swPullLogPanel');
                    const logArea = document.getElementById('swPullLog');
                    if (logPanel && logArea) {{
                        logPanel.style.display = '';
                        logArea.innerText = logJ.lines.join('\\n');
                        logArea.scrollTop = logArea.scrollHeight;
                    }}
                }}
            }} catch (_) {{}}
            
            return;
        }}

        if (stampworldRunPollTimer) {{
            clearInterval(stampworldRunPollTimer);
            stampworldRunPollTimer = null;
        }}
        setBusy(false, '');
        
        // Hide log panel when pull finishes
        const logPanel = document.getElementById('swPullLogPanel');
        if (logPanel) logPanel.style.display = 'none';

        if (j.finished) {{
            const success = j.success === true;
            updateStampWorldRunStatus(j.message || (success ? 'Completed' : 'Failed'), false, success);
            const status = document.getElementById('status');
            if (status) status.innerText = j.message || (success ? 'StampWorld completed.' : 'StampWorld failed.');
            if (success && j.country) {{
                const sel = document.getElementById('countrySel');
                if (sel && Array.from(sel.options).some(o => o.value === j.country)) sel.value = j.country;
                renderCountryPanel();
                await checkAlbumeasyStatusAfterPull();
            }}
            await refreshState();
        }}
    }} catch (_) {{}}
}}

async function runStampWorldPull() {{
    const country = (document.getElementById('swCountry')?.value || '').trim();
    const startYear = parseInt(document.getElementById('swStartYear')?.value || '1840', 10);
    const endYear = parseInt(document.getElementById('swEndYear')?.value || '2026', 10);
    const preset = document.getElementById('swPreset')?.value || 'basic';
    const extraFlags = (document.getElementById('swExtraFlags')?.value || '').trim();
    const status = document.getElementById('status');

    if (!country) {{
        if (status) status.innerText = 'StampWorld country is required.';
        return;
    }}

    updateStampWorldRunStatus('Starting StampWorld pull...', true, false);
    if (status) status.innerText = 'Starting StampWorld pull for ' + country + '...';

    try {{
        const r = await fetch('/api/stampworld-run', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country: country, start_year: startYear, end_year: endYear, preset: preset, extra_flags: extraFlags }})
        }});
        const j = await r.json();
        if (!j.ok) {{
            setBusy(false, '');
            updateStampWorldRunStatus(j.message || 'Failed to start', false, false);
            if (status) status.innerText = j.message || 'Failed to start StampWorld pull.';
            return;
        }}

        if (status) status.innerText = j.message || 'StampWorld pull started.';
        if (stampworldRunPollTimer) clearInterval(stampworldRunPollTimer);
        stampworldRunPollTimer = setInterval(pollStampWorldStatus, 1400);
        await pollStampWorldStatus();
    }} catch (e) {{
        setBusy(false, '');
        updateStampWorldRunStatus('Failed to start', false, false);
        if (status) status.innerText = 'StampWorld request failed: ' + e;
    }}
}}

function openReviewAfterStampWorld() {{
    const country = (document.getElementById('swCountry')?.value || '').trim();
    if (!country) return;
    const sel = document.getElementById('countrySel');
    if (sel && Array.from(sel.options).some(o => o.value === country)) {{
        sel.value = country;
    }}
    startReview();
}}

async function checkAlbumeasyStatusAfterPull() {{
    try {{
        const r = await fetch('/api/albumeasy-status');
        const j = await r.json();
        if (!j.ok) return;
        if (j.configured && j.path_exists) return;

        const hint = j.cli_path ? ('Current path: ' + j.cli_path + '\n\n') : '';
        const answer = prompt(
            'AlbumEasy CLI is not configured for PDF generation.\n\n' +
            hint +
            'Enter full AlbumEasy CLI path now (or leave blank to skip):\n\n' +
            'Download: ' + (j.download_url || 'https://www.thestampweb.com/albumeasy/download')
        );
        if (answer === null || !String(answer).trim()) return;

        const saveResp = await fetch('/api/albumeasy-set-path', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ cli_path: String(answer).trim() }})
        }});
        const saveJson = await saveResp.json();
        const status = document.getElementById('status');
        if (status) status.innerText = saveJson.message || 'AlbumEasy path updated.';
    }} catch (_) {{}}
}}

async function configureAlbumeasyPath() {{
    await checkAlbumeasyStatusAfterPull();
}}

async function copySuggestedCommand() {{
    const cmd = document.getElementById('swCmd')?.innerText || '';
    const status = document.getElementById('status');
    if (!cmd.trim()) {{
        if (status) status.innerText = 'No command available to copy.';
        return;
    }}

    try {{
        if (navigator.clipboard && navigator.clipboard.writeText) {{
            await navigator.clipboard.writeText(cmd);
        }} else {{
            const ta = document.createElement('textarea');
            ta.value = cmd;
            ta.style.position = 'fixed';
            ta.style.left = '-9999px';
            document.body.appendChild(ta);
            ta.focus();
            ta.select();
            document.execCommand('copy');
            document.body.removeChild(ta);
        }}
        if (status) status.innerText = 'Suggested command copied to clipboard.';
    }} catch (e) {{
        if (status) status.innerText = `Copy failed: ${{e}}`;
    }}
}}

function renderCountryPanel() {{
    const sel = document.getElementById('countrySel');
    const c = countries.find(x => x.country === sel.value);
    const swLead = document.getElementById('swPanelLead');
    const swHint = document.getElementById('swPanelHint');

    if (!c) {{
        document.getElementById('countryPanel').innerHTML = '<em>No countries found under Albums/*/*.db</em>';
        document.getElementById('stampworldPanel').style.display = '';
        const dirPathEl = document.getElementById('colnectDirPath');
        const countBadge = document.getElementById('colnectCopyCountBadge');
        const copyStatus = document.getElementById('colnectCopyStatus');
        if (dirPathEl) dirPathEl.innerText = '(select a country)';
        if (countBadge) countBadge.innerText = 'Files: 0';
        if (copyStatus) {{
            copyStatus.className = 'colnectCopyStatus';
            copyStatus.style.display = '';
            copyStatus.textContent = 'Select a country to import Colnect CSV files.';
        }}
        if (swLead) swLead.innerText = 'Run StampWorld pull to create a country database.';
        if (swHint) swHint.innerText = 'Type any country name below and run StampWorld pull. This field is editable and not limited to existing countries.';
        document.getElementById('swCmd').innerText = suggestedStampworldCommand('');
        updateStampWorldRunStatus('Ready to run', false, false);
        return;
    }}

    document.getElementById('countryPanel').className = c.colnect_file_count > 0 || c.colnect_candidate_count > 0 ? 'panel ok' : 'panel';
    document.getElementById('countryPanel').innerHTML =
        `<strong>${{c.country}}</strong><br/>` +
        `<span class='badge'>DB: ${{c.db_path}}</span> ` +
        `<span class='badge'>Colnect files: ${{c.colnect_file_count}}</span> ` +
        `<span class='badge'>DB candidates: ${{c.colnect_candidate_count}}</span>`;

    document.getElementById('swCountry').value = c.country;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(c.country);
    document.getElementById('stampworldPanel').style.display = '';

    if (swLead) swLead.innerText = 'Run StampWorld pull for this country (including additional pull).';
    if (swHint) swHint.innerText = 'You can rerun StampWorld even when data already exists, or edit the country field to pull for a different/new country.';
    updateStampWorldRunStatus('Ready to run', false, false);

    const dirPathEl = document.getElementById('colnectDirPath');
    const countBadge = document.getElementById('colnectCopyCountBadge');
    if (dirPathEl) dirPathEl.innerText = c.colnect_dir || '(unknown)';
    if (countBadge) countBadge.innerText = 'Files: ' + (c.colnect_file_count || 0);
}}

async function pickAndCopyColnectFile() {{
    const country = document.getElementById('countrySel').value;
    if (!country) {{ appendColnectCopyLog('Select a country first.'); return; }}
    const btn = document.getElementById('colnectPickBtn');
    const statusEl = document.getElementById('colnectCopyStatus');
    const logEl = document.getElementById('colnectCopyLog');
    if (btn) btn.disabled = true;
    if (statusEl) {{ statusEl.className = 'colnectCopyStatus'; statusEl.style.display = ''; statusEl.textContent = 'Opening file picker\u2026'; }}
    if (logEl) logEl.style.display = '';
    try {{
        const r = await fetch('/api/bootstrap-pick-colnect-file', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country }})
        }});
        const j = await r.json();
        if (j.ok && j.picked) {{
            if (statusEl) {{ statusEl.className = 'colnectCopyStatus ok'; statusEl.textContent = '\u2714 Copied: ' + (j.file_name || ''); }}
            appendColnectCopyLog('\u2714 ' + (j.message || 'File copied'));
            await refreshState();
        }} else if (j.ok && !j.picked) {{
            if (statusEl) statusEl.textContent = 'Cancelled';
        }} else {{
            if (statusEl) {{ statusEl.className = 'colnectCopyStatus fail'; statusEl.textContent = '\u2718 ' + (j.message || 'Failed'); }}
            appendColnectCopyLog('\u2718 ' + (j.message || 'Failed'));
        }}
    }} catch (e) {{
        if (statusEl) {{ statusEl.className = 'colnectCopyStatus fail'; statusEl.textContent = '\u2718 Error: ' + e; }}
        appendColnectCopyLog('Error: ' + e);
    }} finally {{
        if (btn) btn.disabled = false;
    }}
}}

function appendColnectCopyLog(text) {{
    const logEl = document.getElementById('colnectCopyLog');
    if (!logEl) return;
    const el = document.createElement('div');
    el.className = 'line';
    el.textContent = new Date().toLocaleTimeString() + ' ' + text;
    logEl.appendChild(el);
    while (logEl.children.length > 120) logEl.removeChild(logEl.firstChild);
    logEl.scrollTop = logEl.scrollHeight;
}}

async function refreshState() {{
    const status = document.getElementById('status');
    status.innerText = 'Scanning Albums...';
    setBusy(false, '');
    try {{
        const r = await fetch('/bootstrap-state');
        const j = await r.json();
        countries = j.countries || [];
        const albumsRoot = String(j.albums_root || '').trim();
        updateAlbumsRootHint(albumsRoot);
        const rootInput = document.getElementById('albumsRootPath');
        if (rootInput) rootInput.value = albumsRoot;
        setAlbumsRootStatus(albumsRoot ? ('Using: ' + albumsRoot) : 'Using auto-detect Albums root', 'ok');
        const sel = document.getElementById('countrySel');
        const old = sel.value;
        sel.innerHTML = countries.map(c => `<option value="${{c.country}}">${{c.country}}</option>`).join('');
        if (old && countries.some(c => c.country === old)) sel.value = old;
        if (!sel.value && countries.length) sel.value = countries[0].country;
        const searchInput = document.getElementById('countrySearch');
        if (searchInput && sel.value) searchInput.value = sel.value;
        document.getElementById('countryCount').innerText = `Countries: ${{countries.length}}`;
        renderCountryPanel();
        updateCountrySearchMatches(true);
        if (countries.length) {{
            status.innerText = '';
        }} else {{
            status.innerText = albumsRoot
                ? ('No country DBs found under ' + albumsRoot + '. Use Albums Root to switch folders or run a StampWorld pull first.')
                : 'No country DBs found. The app could not auto-detect Albums; set Albums Root or run a StampWorld pull first.';
        }}
    }} catch (e) {{
        status.innerText = `Failed to load startup state: ${{e}}`;
    }}
}}

async function startReview() {{
    const sel = document.getElementById('countrySel');
    if (!sel.value) {{
        document.getElementById('status').innerText = 'Select a country first.';
        return;
    }}

    async function waitForCompareAndRedirect(maxAttempts, delayMs) {{
        for (let attempt = 0; attempt < maxAttempts; attempt++) {{
            await new Promise(r => setTimeout(r, delayMs));
            try {{
                const rdy = await fetch('/compare-ready', {{ cache: 'no-store' }});
                if (rdy.ok) {{
                    window.location.href = '/';
                    return true;
                }}
            }} catch (_) {{}}
        }}
        return false;
    }}

    document.getElementById('status').innerText = `Starting Review UI for ${{sel.value}}...`;
    setBusy(true, `Preparing data from ${{sel.value}}`);
    startLoadingPhases(sel.value);
    try {{
        let r = null;
        let lastStartError = null;
        for (let attempt = 0; attempt < 3; attempt++) {{
            try {{
                r = await fetch('/start-review', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    cache: 'no-store',
                    body: JSON.stringify({{ country: sel.value }})
                }});
                lastStartError = null;
                break;
            }} catch (e) {{
                lastStartError = e;
                if (attempt < 2) await new Promise(res => setTimeout(res, 250));
            }}
        }}

        if (!r) {{
            document.getElementById('status').innerText = 'Switching to compare view... checking server readiness.';
            document.getElementById('loadingText').innerText = 'Connection briefly dropped while switching modes. Verifying compare view...';
            const readyAfterDrop = await waitForCompareAndRedirect(24, 250);
            if (readyAfterDrop) return;
            throw lastStartError || new Error('Failed to reach /start-review');
        }}

        const j = await r.json();
        if (!j.ok) {{
            setBusy(false, '');
            document.getElementById('status').innerText = j.message || 'Failed to start review.';
            return;
        }}
        stopLoadingPhases();
        document.getElementById('loadingText').innerText = 'Launching compare view...';
        document.getElementById('status').innerText = j.message || 'Launching review...';
        // Poll /compare-ready until the review UI server is accepting requests,
        // then redirect. This avoids a race between data loading and the redirect.
        (async function pollReady() {{
            const ready = await waitForCompareAndRedirect(60, 500);
            if (!ready) window.location.href = '/';
        }})();
    }} catch (e) {{
        setBusy(false, '');
        document.getElementById('status').innerText = `Failed to start: ${{e}}`;
    }}
}}

async function logoutApp() {{
    const status = document.getElementById('status');
    if (status) status.innerText = 'Logging out and closing Review UI...';
    setBusy(true, 'Closing Review UI...');
    try {{
        await fetch('/logout', {{ method: 'POST' }});
    }} catch (_) {{
        // Even if the response is interrupted, continue with shutdown UX.
    }}
    setBusy(false, '');
    document.body.innerHTML = "<div style='font-family:sans-serif;margin:20px;'><h2>Review UI closed</h2><p>You can close this browser tab now.</p></div>";
}}

async function importColnectFiles() {{
    const sel = document.getElementById('countrySel');
    const country = sel.value;
    const status = document.getElementById('status');
    const statusBox = document.getElementById('colnectCopyStatus');
    const logEl = document.getElementById('colnectCopyLog');
    
    if (!country) {{
        if (status) status.innerText = 'Select a country first.';
        if (statusBox) {{
            statusBox.className = 'colnectCopyStatus fail';
            statusBox.textContent = 'Select a country first.';
        }}
        return;
    }}

    if (status) status.innerText = 'Importing Colnect files for ' + country + '...';
    setBusy(true, 'Importing Colnect CSV files for ' + country + '...');
    if (statusBox) {{
        statusBox.className = 'colnectCopyStatus';
        statusBox.textContent = 'Starting import for ' + country + '...';
    }}
    appendColnectCopyLog('--- import started for ' + country + ' ---');

    let shownLogCount = 0;

    async function pollImportLog() {{
        try {{
            const r = await fetch('/runtime-log', {{ cache: 'no-store' }});
            const j = await r.json();
            const lines = j.lines || [];
            const newLines = lines.slice(shownLogCount);
            shownLogCount = lines.length;
            for (const line of newLines) {{
                appendColnectCopyLog(line);
                if (statusBox) {{
                    statusBox.className = 'colnectCopyStatus';
                    statusBox.textContent = line;
                }}
            }}
        }} catch (_) {{}}
    }}

    const logPollTimer = setInterval(pollImportLog, 700);

    try {{
        const r = await fetch('/api/colnect-import', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();
        clearInterval(logPollTimer);
        await pollImportLog();
        setBusy(false, '');

        if (j.ok && j.stamps_imported > 0) {{
            const doneMsg = 'Imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)';
            if (status) status.innerText = doneMsg;
            if (statusBox) {{
                statusBox.className = 'colnectCopyStatus ok';
                statusBox.textContent = doneMsg;
            }}
            appendColnectCopyLog('✔ ' + doneMsg);
            await refreshState();
        }} else if (j.ok && j.stamps_imported === 0) {{
            const msg = j.message || 'No stamps were imported.';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nDetails:\n' + j.errors.join('\n')
                : '\n\nOpen the Review UI for this country and verify the configured colnect inbox path.';
            if (status) status.innerText = 'No stamps imported';
            if (statusBox) {{
                statusBox.className = 'colnectCopyStatus';
                statusBox.textContent = msg;
            }}
            appendColnectCopyLog(msg);
            if (j.errors && j.errors.length > 0) {{
                for (const err of j.errors) appendColnectCopyLog('! ' + err);
            }} else {{
                appendColnectCopyLog(details.replace(/^\n+/, ''));
            }}
        }} else {{
            const msg = j.message || 'Import failed';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nErrors:\n' + j.errors.join('\n')
                : '';
            if (status) status.innerText = 'Import unavailable: ' + msg;
            if (statusBox) {{
                statusBox.className = 'colnectCopyStatus fail';
                statusBox.textContent = msg;
            }}
            appendColnectCopyLog('✘ ' + msg);
            if (details) appendColnectCopyLog(details.replace(/^\n+/, ''));
        }}
    }} catch (e) {{
        clearInterval(logPollTimer);
        setBusy(false, '');
        if (status) status.innerText = 'Import error';
        if (statusBox) {{
            statusBox.className = 'colnectCopyStatus fail';
            statusBox.textContent = 'Import error: ' + e;
        }}
        appendColnectCopyLog('Error: ' + e);
    }}
}}

async function generateAlbumPdfNow() {{
    const sel = document.getElementById('countrySel');
    const country = String((sel && sel.value) || '').trim();
    const status = document.getElementById('status');

    if (bootstrapPdfRequestInFlight) {{
        if (status) status.innerText = 'PDF generation already in progress. Please wait.';
        return;
    }}

    if (!country) {{
        if (status) status.innerText = 'Select a country first.';
        return;
    }}

    const startYearVal = document.getElementById('pdfStartYear')?.value.trim();
    const endYearVal   = document.getElementById('pdfEndYear')?.value.trim();
    const startYear    = startYearVal ? parseInt(startYearVal, 10) : null;
    const endYear      = endYearVal   ? parseInt(endYearVal, 10)   : null;

    const yearDesc = (startYear || endYear)
        ? ` (years ${{startYear || 'start'}}–${{endYear || 'end'}})` : '';
    if (status) status.innerText = 'Generating album text and PDF for ' + country + yearDesc + '...';
    setBusy(true, 'Generating album and PDF for ' + country + yearDesc + '...');
    bootstrapPdfRequestInFlight = true;

    try {{
        const r = await fetch('/api/generate-pdf-now', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country: country, start_year: startYear, end_year: endYear }})
        }});
        const j = await r.json();
        setBusy(false, '');

        if (!j.ok) {{
            if (status) status.innerText = j.message || 'PDF generation failed.';
            alert('PDF generation failed.\n\n' + (j.message || 'Unknown error.'));
            return;
        }}

        const outputPath = j.generated_output_path ? ('\nOutput: ' + j.generated_output_path) : '';
        const pdfPath = j.generated_pdf_path ? ('\nPDF: ' + j.generated_pdf_path) : '';
        if (status) status.innerText = j.message || 'PDF generation completed.';
        alert((j.message || 'PDF generation completed.') + outputPath + pdfPath);
    }} catch (e) {{
        setBusy(false, '');
        const msg = 'Generate PDF request failed: ' + e;
        if (status) status.innerText = msg;
        alert(msg);
    }} finally {{
        bootstrapPdfRequestInFlight = false;
    }}
}}

document.getElementById('swStartYear').addEventListener('change', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swEndYear').addEventListener('change', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swCountry').addEventListener('input', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swPreset').addEventListener('change', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swExtraFlags').addEventListener('input', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});

document.getElementById('albumsRootPath').addEventListener('keydown', (ev) => {{
    if (ev.key === 'Enter') {{
        ev.preventDefault();
        setAlbumsRootPath();
    }}
}});

initYearSelectors();
ensureCopyableAlertDialog();
try {{
    const v = localStorage.getItem(runtimeLogVisibleStorageKey);
    const cb = document.getElementById('showRuntimeLog');
    if (cb && (v === '0' || v === '1')) cb.checked = v === '1';
}} catch (_) {{}}
applyRuntimeLogVisibility();
startRuntimeLogPolling();
startBackendHealthPolling();
refreshState();
pollStampWorldStatus();
document.querySelectorAll('.copyrightCurrentYear').forEach((el) => {{
    el.textContent = String(new Date().getFullYear());
}});
</script>
{footer_html}
<script>
document.querySelectorAll('.copyrightCurrentYear').forEach((el) => {{
    el.textContent = String(new Date().getFullYear());
}});
</script>
{config_editor_assets}
</body>
</html>"#,
        inbox_hint = inbox_hint,
        backend_not_responding_message = BACKEND_NOT_RESPONDING_MESSAGE,
        footer_html = render_app_footer_html(),
        config_editor_assets = render_config_editor_assets()
    )
}

pub(super) fn render_html(payload: &MatcherPayload) -> String {
    let json =
        serde_json::to_string(payload).unwrap_or_else(|_| "{\"left\":[],\"right\":[]}".to_string());
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset='utf-8'>
<title>Stamp Album Gen 2</title>
<link rel='icon' type='image/png' href='/favicon.png'>
<style>
body {{
  font-family: sans-serif;
  margin: 16px;
    background: linear-gradient(180deg, #eef4f8 0%, #f7f9fb 100%);
    opacity: 1;
    transition: opacity 170ms ease, filter 170ms ease, transform 170ms ease;
  --row-odd-bg: #ffffff;
  --row-even-bg: #e6eef4;
  --row-odd-key-bg: #f1f3f5;
  --row-even-key-bg: #d7e2eb;
    --accent-blue: #2f6fa3;
    --accent-green: #2f8f63;
    --accent-amber: #9b6a20;
    --accent-red: #a34545;
    --accent-slate: #4b5a68;
}}
body.pageLeaving {{ opacity: 0.2; filter: blur(1px); transform: translateY(2px); }}
body.pageEnter {{ animation: reviewPageIn 170ms ease-out; }}
@keyframes reviewPageIn {{ from {{ opacity: 0.45; transform: translateY(2px); }} to {{ opacity: 1; transform: translateY(0); }} }}
.wrap {{ background: #ffffff; border: 1px solid #dbe4ec; border-radius: 12px; padding: 14px; box-shadow: 0 8px 24px rgba(27, 51, 72, 0.08); }}
.top {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-bottom: 10px; }}
.startGuide {{ border: 1px solid #d8e3ed; border-left: 6px solid var(--accent-blue); border-radius: 10px; background: #f5fafe; padding: 12px; margin-bottom: 12px; }}
.startGuide h2 {{ margin: 0 0 4px 0; font-size: 18px; color: #173952; }}
.startGuide p {{ margin: 0; color: #35566d; font-size: 13px; }}
.guideStats {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.guideActions {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.controlGroups {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 10px; margin-bottom: 10px; }}
.groupCard {{ border: 1px solid #dbe4ec; border-radius: 10px; background: #fbfdff; padding: 10px; }}
.groupCard h4 {{ margin: 0 0 8px 0; font-size: 13px; letter-spacing: 0.2px; }}
.groupCard.match {{ border-left: 5px solid var(--accent-green); }}
.groupCard.candidate {{ border-left: 5px solid var(--accent-amber); }}
.groupCard.session {{ border-left: 5px solid var(--accent-slate); }}
.groupBtns {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.sessionStack {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 8px; width: 100%; }}
.sessionSection {{ border-radius: 8px; padding: 8px; }}
.sessionSectionBlue {{ border: 1px solid #cddff2; background: #f2f8ff; }}
.sessionSectionGreen {{ border: 1px solid #cfe8da; background: #f2fbf6; }}
.sessionSectionAmber {{ border: 1px solid #ead8bc; background: #fff8ef; }}
.sessionSectionSlate {{ border: 1px solid #d5dbe3; background: #f5f6f8; }}
.sessionSectionTitle {{ margin: 0 0 6px 0; font-size: 12px; font-weight: 700; color: #2a4458; letter-spacing: 0.2px; }}
.sessionRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.sessionStatusBox {{ margin-top: 8px; min-height: 16px; font-size: 12px; border-radius: 8px; border-left-width: 4px; border-left-style: solid; padding: 6px 8px; }}
.sessionStatusBlue {{ border: 1px solid #cddff2; border-left: 4px solid #2f6fa3; background: #f8fcff; color: #2f4f67; }}
.sessionStatusGreen {{ border: 1px solid #cfe8da; border-left: 4px solid #2f8f63; background: #f4fcf8; color: #245641; }}
.sessionStatusAmber {{ border: 1px solid #ead8bc; border-left: 4px solid #c08a2b; background: #fffaf2; color: #694314; }}
.sessionStatusSlate {{ border: 1px solid #d5dbe3; border-left: 4px solid #5c6b78; background: #f7f8fa; color: #334b60; }}
.actionRows {{ display: flex; flex-direction: column; gap: 8px; margin-bottom: 10px; }}
.actionBar {{ border: 1px solid #dbe4ec; border-radius: 10px; padding: 9px 10px; background: #fbfdff; display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.actionBar.match {{ border-left: 5px solid var(--accent-green); }}
.actionBar.candidate {{ border-left: 5px solid var(--accent-amber); }}
.actionBarTitle {{ font-size: 12px; font-weight: 700; color: #2a4458; margin-right: 6px; }}
.actionMessage {{ border: 1px solid #dbe4ec; border-radius: 8px; background: #f7fbff; color: #35566d; padding: 7px 10px; font-size: 12px; min-height: 16px; }}
.filtersCard {{ border: 1px solid #dbe4ec; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: #f8fbfe; padding: 10px; margin-bottom: 12px; }}
.filtersTitle {{ margin: 0 0 8px 0; font-size: 13px; color: #1f4663; }}
.filtersGrid {{ display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.leftActionBanner {{ border: 1px solid #e5dccd; border-left: 5px solid var(--accent-amber); border-radius: 10px; background: linear-gradient(180deg, #fffaf2 0%, #fffdf8 100%); padding: 10px; margin-bottom: 8px; }}
.leftActionTop {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.leftActionTop strong {{ color: #5d4116; }}
.leftActionText {{ margin: 0; color: #6c5530; font-size: 12px; line-height: 1.45; }}
.leftActionBtns {{ display: flex; gap: 8px; flex-wrap: wrap; margin-top: 8px; }}
.leftActionNote {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 6px; }}
.leftSearchRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }}
.leftSearchRow input {{ width: 220px; max-width: 52vw; padding: 5px 7px; border: 1px solid #c8d5e2; border-radius: 7px; }}
.leftNavRow {{ display: flex; gap: 6px; align-items: center; flex-wrap: wrap; margin-top: 8px; background: #fff4e2; border: 1px solid #ead5b4; border-radius: 8px; padding: 6px; }}
.rightActionBanner {{ border: 1px solid #d6e4f2; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: linear-gradient(180deg, #f4f9ff 0%, #f9fcff 100%); padding: 10px; margin-bottom: 8px; }}
.rightActionTop {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.rightActionTop strong {{ color: #214d72; }}
.rightBottomBlock {{ display: flex; flex-direction: column; gap: 0; }}
.rightSearchRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }}
.rightSearchRow input {{ width: 220px; max-width: 52vw; padding: 5px 7px; border: 1px solid #c8d5e2; border-radius: 7px; }}
.rightNavRow {{ display: flex; gap: 6px; align-items: center; flex-wrap: wrap; margin-top: 8px; background: #eaf4ff; border: 1px solid #c9ddf2; border-radius: 8px; padding: 6px; }}
@media (min-width: 981px) {{
    #comparePanels {{
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto auto 1fr;
        gap: 0 16px;
        align-items: start;
    }}
    #comparePanels .col {{
        display: grid;
        grid-template-rows: subgrid;
        grid-row: 1 / span 3;
    }}
    #comparePanels .col:first-child {{ grid-column: 1; }}
    #comparePanels .col:last-child  {{ grid-column: 2; }}
    /* Push right search/nav to the bottom of the shared banner row so it
       sits at the same eye level as the left search/nav (which is naturally
       at the bottom of the taller left action banner). */
    .rightActionBanner {{
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }}
}}
.leftEditorGrid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 10px; margin-bottom: 8px; }}
.leftEditorGrid label, .leftEditorStack label {{ display: flex; flex-direction: column; gap: 4px; font-size: 12px; font-weight: 600; color: #284357; }}
.leftEditorStack {{ display: flex; flex-direction: column; gap: 8px; margin-bottom: 8px; }}
.leftEditorStack textarea {{ min-height: 74px; resize: vertical; }}
.leftEditorModes {{ display: flex; gap: 12px; flex-wrap: wrap; margin: 8px 0; }}
.leftEditorHint {{ margin: 0 0 4px 0; color: #5d6f7f; font-size: 12px; }}
.leftEditorDialog {{ width: min(980px, 97vw); border: 1px solid #c8d9e5; border-radius: 14px; padding: 0; overflow: hidden; box-shadow: 0 22px 48px rgba(16, 39, 56, 0.28); }}
.leftEditorHead {{ background: linear-gradient(180deg, #f3f8fc 0%, #eaf2f8 100%); padding: 14px 16px 12px; border-bottom: 1px solid #d6e3ed; }}
.leftEditorHead h4 {{ margin: 0 0 6px 0; font-size: 18px; color: #173a4f; }}
.leftEditorBody {{ max-height: 66vh; overflow-y: auto; padding: 14px 16px; background: linear-gradient(180deg, #fbfdff 0%, #f2f7fb 100%); }}
.leftEditorSection {{ border: 1px solid #d5e1ea; border-radius: 12px; background: #ffffff; margin-bottom: 12px; overflow: hidden; }}
.leftEditorSection h5 {{ margin: 0; padding: 10px 12px; font-size: 13px; color: #173a4f; background: #eef5fa; border-bottom: 1px solid #d9e5ef; }}
.leftEditorSectionBody {{ padding: 10px 12px; }}
.leftEditorDimsGrid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; }}
.leftEditorCompact input, .leftEditorCompact textarea {{ background: #fcfeff; border: 1px solid #cddbe6; border-radius: 8px; padding: 7px 9px; }}
.leftEditorCompact input:focus, .leftEditorCompact textarea:focus {{ outline: 2px solid rgba(47, 111, 163, 0.16); border-color: #2f6fa3; }}
.leftEditorActions {{ display: flex; justify-content: flex-end; gap: 10px; padding: 12px 16px; border-top: 1px solid #d6e3ed; background: #f5f9fc; }}
.row {{ display: flex; gap: 16px; align-items: stretch; }}
.col {{ flex: 1; min-width: 280px; display: flex; flex-direction: column; }}
.colNav {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 6px; }}
.titleRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 8px; }}
.titleRow h3 {{ margin: 0; }}
.panelCard {{ display: flex; flex-direction: column; border: 1px solid #ddd; border-radius: 8px; background: #fff; overflow: hidden; min-height: 0; }}
.panelHead {{ padding: 10px; border-bottom: 1px solid #ececec; background: #fbfcfd; font-size: 12px; }}
.panelBody {{ padding: 8px; overflow: auto; max-height: 56vh; }}
.imgbox {{ min-height: 260px; display: flex; align-items: center; justify-content: center; border: 1px solid #ccc; background: #fff; }}
img {{ max-width: 260px; max-height: 260px; border: 1px solid #ccc; background: #fff; }}
.imgph {{ width: 260px; height: 260px; border: 1px dashed #bbb; background: #f7f7f7; color: #666; display: flex; align-items: center; justify-content: center; text-align: center; font-size: 12px; padding: 8px; box-sizing: border-box; }}
.kv {{ width: 100%; border-collapse: collapse; margin-top: 8px; font-size: 12px; }}
.kv td {{ border: 1px solid #e1e6ea; padding: 4px 6px; vertical-align: top; }}
.kv tr:nth-child(odd) td {{ background: var(--row-odd-bg); }}
.kv tr:nth-child(even) td {{ background: var(--row-even-bg); }}
.kv tr:nth-child(odd) td.k {{ background: var(--row-odd-key-bg); }}
.kv tr:nth-child(even) td.k {{ background: var(--row-even-key-bg); }}
.kv td.k {{ width: 36%; font-weight: 600; }}
.kv td.v {{ width: 64%; word-break: break-word; }}
button {{ padding: 6px 10px; border-radius: 8px; border: 1px solid #c9d5e2; background: #ffffff; color: #1f3448; cursor: pointer; }}
button:hover {{ background: #f1f6fa; }}
button.primary {{ background: #2f6fa3; border-color: #2f6fa3; color: #ffffff; }}
button.primary:hover {{ background: #285f8a; }}
button.success {{ background: #2f8f63; border-color: #2f8f63; color: #ffffff; }}
button.success:hover {{ background: #287953; }}
button.warn {{ background: #f0e1c6; border-color: #ddc79f; color: #694314; }}
button.danger {{ background: #a34545; border-color: #a34545; color: #ffffff; }}
button.danger:hover {{ background: #8f3939; }}
button.neutral {{ background: #4b5a68; border-color: #4b5a68; color: #ffffff; }}
button.navBtn {{ background: #eef4f8; border-color: #d4e2ee; }}
.badge {{ display: inline-block; padding: 2px 8px; border: 1px solid #d3dfe9; border-radius: 999px; background: #f8fbfe; font-size: 12px; color: #2d4760; }}
.modal-backdrop {{ position: fixed; inset: 0; background: rgba(0,0,0,0.45); display: none; align-items: center; justify-content: center; z-index: 1000; }}
.modal {{ background: #fff; border-radius: 10px; border: 1px solid #ddd; width: min(700px, 96vw); max-height: 88vh; overflow-y: auto; padding: 14px; box-sizing: border-box; }}
.modal h4 {{ margin: 0 0 10px 0; }}
.modal .rowx {{ display: flex; gap: 10px; margin-bottom: 8px; flex-wrap: wrap; }}
.modal .rowx label {{ display: inline-flex; align-items: center; gap: 6px; }}
.modal input[type='text'], .modal input[type='number'] {{ padding: 6px 8px; border: 1px solid #ccc; border-radius: 6px; min-width: 140px; }}
.opBusyBackdrop {{
    position: fixed;
    inset: 0;
    display: none;
    align-items: center;
    justify-content: center;
    background: rgba(15, 28, 42, 0.48);
    z-index: 1500;
}}
.opBusyCard {{
    width: min(560px, 92vw);
    border: 1px solid #c9d8e6;
    border-radius: 12px;
    background: #ffffff;
    box-shadow: 0 18px 42px rgba(17, 38, 56, 0.25);
    padding: 14px 16px;
}}
.opBusyRow {{
    display: flex;
    align-items: center;
    gap: 10px;
}}
.opBusySpinner {{
    width: 18px;
    height: 18px;
    border: 3px solid #d7e2eb;
    border-top-color: #2f6fa3;
    border-radius: 50%;
    animation: spin 0.9s linear infinite;
    flex: 0 0 auto;
}}
.opBusyText {{
    color: #1f3b52;
    font-size: 13px;
    line-height: 1.4;
}}
.opBusyDots {{
    display: inline-block;
    min-width: 18px;
    text-align: left;
}}
.reconnectBanner {{
    display: none;
    border: 1px solid #ddc79f;
    border-left: 5px solid #9b6a20;
    border-radius: 10px;
    background: #fff7e8;
    color: #694314;
    padding: 10px 12px;
    margin-bottom: 10px;
}}
.reconnectBanner.show {{
    display: block;
}}
.reconnectBanner.recovered {{
    border-color: #b9dec9;
    border-left-color: #2f8f63;
    background: #f4fcf8;
    color: #245641;
}}
.reconnectBanner .title {{
    font-weight: 700;
    margin-bottom: 4px;
}}
.reconnectBanner .meta {{
    font-size: 12px;
    color: #5e7283;
}}
.reconnectBanner .actions {{
    display: flex;
    gap: 8px;
    margin-top: 8px;
    flex-wrap: wrap;
}}
.colnectImportStatus {{ margin-top: 8px; }}
.colnectImportSpinRow {{ display: flex; align-items: center; gap: 8px; }}
.colnectImportSpinner {{ width: 14px; height: 14px; border: 2px solid #d7e2eb; border-top-color: #2f6fa3; border-radius: 50%; animation: colnectSpin 0.9s linear infinite; flex: 0 0 auto; display: inline-block; }}
.colnectImportStatusText {{ font-size: 12px; color: inherit; font-weight: 600; }}
.colnectImportLog {{ margin-top: 6px; max-height: 120px; overflow-y: auto; font-size: 11px; font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; background: #f2f7fb; border: 1px solid #d3dfe9; border-radius: 6px; padding: 4px 6px; }}
.colnectLogLine {{ padding: 2px 0; border-bottom: 1px solid #e8eef4; animation: colnectMsgIn 0.2s ease-out; }}
.colnectLogLine:last-child {{ border-bottom: none; }}
.reviewColnectDirPath {{ display: block; max-width: 100%; white-space: pre-wrap; overflow-wrap: anywhere; word-break: break-word; }}
.colnectImportDone {{ color: #2f8f63 !important; }}
.colnectImportFail {{ color: #a34545 !important; }}
.pdfStatusWrap {{
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    min-height: 22px;
}}
.pdfStatusText {{
    font-size: 12px;
    color: inherit;
}}
.pdfStatusDone {{ color: #2f8f63; font-weight: 600; }}
.pdfStatusFail {{ color: #a34545; font-weight: 600; }}
@keyframes colnectSpin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@keyframes colnectMsgIn {{ from {{ opacity: 0; transform: translateY(3px); }} to {{ opacity: 1; transform: translateY(0); }} }}
@keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@media (max-width: 980px) {{
    .row {{ flex-direction: column; }}
    .panelBody {{ max-height: 42vh; }}
}}
</style>
</head>
<body>
<div class='wrap'>
    <div id='reconnectBannerCompare' class='reconnectBanner' role='status' aria-live='polite'>
        <div id='reconnectBannerCompareTitle' class='title'>Connection lost, retrying...</div>
        <button onclick='refreshState()'>Refresh</button>
        <button onclick='openConfigEditor()'>Edit Config</button>
        <div class='actions'>
            <button type='button' onclick='retryBackendHealthCheckCompare()'>Retry Now</button>
            <button type='button' onclick='reloadCurrentPageCompare()'>Reload Page</button>
            <span id='reconnectBannerCompareMeta' class='meta'></span>
        </div>
    </div>
    <div class='top'>
        <button class='neutral' onclick='returnToStartReviewPage()'>Back To Country Selection</button>
        <span class='badge'>Switch country without closing Review UI</span>
    </div>
    <div id='startGuide' class='startGuide'>
        <h2>Start Country Review Session</h2>
        <p>Use the grouped controls to navigate candidates, assign matches, and apply updates safely without losing CLI/config capabilities.</p>
        <div class='guideStats'>
            <span id='statLeftCount' class='badge'>Left: 0</span>
            <span id='statRightCount' class='badge'>Candidates: 0</span>
            <span id='statAssignedCount' class='badge'>Assigned: 0</span>
        </div>
        <div class='guideActions'>
            <button class='primary' onclick='dismissStartGuide()'>Start Reviewing</button>
            <button onclick='scrollToFilters()'>Review Field Options</button>
            <button onclick='scrollToPanels()'>Jump To Compare Panels</button>
        </div>
  </div>
    <div id='runtimeLogComparePanel' style='border:1px solid #dbe4ec;border-radius:10px;background:#fbfdff;padding:10px;margin-bottom:10px;'>
        <div style='display:flex;gap:8px;justify-content:space-between;align-items:center;flex-wrap:wrap;margin-bottom:6px;'>
            <strong>Runtime Log</strong>
            <div style='display:flex;gap:8px;align-items:center;flex-wrap:wrap;'>
                <label style='font-size:12px;font-weight:600;display:flex;align-items:center;gap:6px;'>
                    <input id='showRuntimeLogCompare' type='checkbox' checked onchange='toggleRuntimeLogVisibilityCompare()' />
                    Show Log
                </label>
                <button onclick='copyRuntimeLogCompare()'>Copy Log</button>
            </div>
        </div>
        <div id='runtimeLogCompareBody'>
            <textarea id='runtimeLogCompare' readonly style='width:100%;min-height:120px;max-height:220px;resize:vertical;padding:8px;border:1px solid #c8d5e2;border-radius:8px;font-family:monospace;font-size:12px;'></textarea>
        </div>
    </div>
    <div class='controlGroups'>
        <div class='groupCard session'>
            <h4>Session</h4>
            <div class='sessionStack'>
                <div class='sessionSection sessionSectionBlue'>
                    <div class='sessionSectionTitle'>Configuration</div>
                    <div class='sessionRow'>
                        <button onclick='openConfigEditor()'>Edit Local Config</button>
                        <label for='defaultCatalogSelect'>Local Catalog</label>
                        <select id='defaultCatalogSelect' onchange='changeCountryDefaultCatalog()'>
                            <option value='sg'>Stanley Gibbons (SG)</option>
                            <option value='sn'>Scott Number (SN)</option>
                            <option value='mi'>Michel (MI)</option>
                            <option value='yt'>Yvert (YT)</option>
                            <option value='afa'>AFA</option>
                            <option value='nvp'>NVP</option>
                            <option value='un'>Unificato (UN)</option>
                            <option value='col'>Colnect (COL)</option>
                        </select>
                        <button onclick='saveCatalogDefaultToConfig()'>Update Local Config Default</button>
                    </div>
                    <div id='configStatus' class='sessionStatusBox sessionStatusBlue'>Ready</div>
                </div>
                <div class='sessionSection sessionSectionGreen'>
                    <div class='sessionSectionTitle'>Import Colnect</div>
                    <div class='sessionRow'>
                        <button id='importColnectPickBtn' onclick='pickAndCopyColnectFileReview()'>Browse &amp; Copy Colnect File&#8230;</button>
                        <button id='importColnectBtn' onclick='importColnectFilesReview()'>Import Colnect</button>
                        <span id='reviewColnectCopyCountBadge' class='badge'>Files: 0</span>
                    </div>
                    <div style='margin-top:6px;font-size:12px;color:#35566d;'>
                        Destination: <code id='reviewColnectDirPath' class='reviewColnectDirPath'>(unknown)</code>
                    </div>
                    <div id='colnectImportStatus' class='colnectImportStatus sessionStatusBox sessionStatusGreen'>
                        <div class='colnectImportSpinRow'>
                            <span class='colnectImportSpinner' style='display:none;'></span>
                            <span id='colnectImportStatusText' class='colnectImportStatusText'>Ready</span>
                        </div>
                        <div id='colnectImportLog' class='colnectImportLog' style='display:none;'></div>
                    </div>
                </div>
                <div class='sessionSection sessionSectionAmber'>
                    <div class='sessionSectionTitle'>PDF</div>
                    <div class='sessionRow'>
                        <button id='reviewPdfBtn' onclick='generatePdfNowReview()'>Generate PDF Now</button>
                        <button id='reviewPdfPreviewBtn' onclick='previewLastPdfReview()' style='display:none;'>Preview Last PDF</button>
                        <label for='reviewPdfStartYear' style='font-size:0.9em;margin:0 0 0 0.4em;'>Year</label>
                        <input id='reviewPdfStartYear' type='number' placeholder='Start' min='1840' max='2100' style='width:78px;padding:2px 4px;font-size:0.9em'/>
                        <span style='font-size:0.9em;'>–</span>
                        <input id='reviewPdfEndYear' type='number' placeholder='End' min='1840' max='2100' style='width:78px;padding:2px 4px;font-size:0.9em'/>
                        <label for='reviewPdfAlwaysShow' style='font-size:0.9em;display:inline-flex;align-items:center;gap:6px;'>
                            <input id='reviewPdfAlwaysShow' type='checkbox' onchange='togglePdfAlwaysOpenPreference()'/>
                            Always open PDF after generation
                        </label>
                    </div>
                    <div id='pdfStatusWrap' class='pdfStatusWrap sessionStatusBox sessionStatusAmber'>
                        <span id='pdfStatusSpinner' class='colnectImportSpinner' style='display:none;'></span>
                        <span id='pdfStatus' class='pdfStatusText'>Ready</span>
                    </div>
                </div>
                <div class='sessionSection sessionSectionSlate'>
                    <div class='sessionSectionTitle'>Navigation</div>
                    <div class='sessionRow'>
                        <button onclick='returnToStartReviewPage()'>Back To Country Selection</button>
                        <button class='neutral' onclick='closeReviewUi()'>Close Review UI</button>
                    </div>
                    <div id='status' class='sessionStatusBox sessionStatusSlate'>Ready</div>
                </div>
            </div>
        </div>
    </div>
        <div id='filtersCard' class='filtersCard'>
            <h4 class='filtersTitle'>Field Update Options</h4>
            <div class='filtersGrid'>
        <label><input type='checkbox' id='f_series'/> series</label>
        <label><input type='checkbox' id='f_description' checked/> description</label>
        <label><input type='checkbox' id='f_watermark'/> watermark</label>
        <label><input type='checkbox' id='f_design'/> design</label>
        <label><input type='checkbox' id='f_engraving'/> engraving</label>
        <label><input type='checkbox' id='f_perforation'/> perforation</label>
        <label><input type='checkbox' id='f_context'/> context</label>
        <label><input type='checkbox' id='f_links' checked/> colnect links</label>
        <label><input type='checkbox' id='f_image_url' checked/> colnect image url</label>
        <label><input type='checkbox' id='f_colnect_meta' checked/> colnect metadata fields</label>
        <label><input type='checkbox' id='f_replace_sg'/> replace catalog number</label>
        <label><input type='checkbox' id='f_right_unmatched_only' onchange='render()'/> right unmatched only</label>
        <label><input type='checkbox' id='f_show_imageless_left' onchange='toggleImagelessLeftRows()'/> show left rows without images</label>
                <label>order
                        <select id='sortMode' onchange='changeSortMode()'>
                                <option value='sg'>Catalog number (default)</option>
                                <option value='year_sg'>Year then catalog number</option>
                                <option value='series_sg'>Series then catalog number</option>
                                <option value='rowid'>DB row id</option>
                                <option value='colnect_id'>Colnect ID</option>
                        </select>
                </label>
      </div>
    </div>
    <div class='actionRows'>
        <div class='actionBar match'>
            <span class='actionBarTitle'>Match Actions</span>
            <button class='success' onclick='assignCurrent()'>Assign Candidate</button>
            <button onclick='clearAssignment()'>Clear Assignment</button>
            <button class='primary' onclick='applyMatches()'>Apply Matches To DB</button>
        </div>
        <div class='actionBar candidate'>
            <span class='actionBarTitle'>Candidate Management</span>
            <button class='warn' onclick='openAddDialog()'>Add Colnect As Stamp</button>
            <button class='danger' onclick='deleteCurrentCandidate()'>Delete Candidate</button>
            <button onclick='undoLastDelete()'>Undo Delete</button>
        </div>
        <div id='matchActionStatus' class='actionMessage'>Ready</div>
    </div>
  <div id='comparePanels' class='row'>
    <div class='col'>
            <div class='titleRow'>
                <h3>Left: StampWorld Base</h3>
                <span id='leftPos' class='badge'></span>
            </div>
                        <div id='leftActionBanner' class='leftActionBanner'></div>
      <div id='leftPanel'></div>
    </div>
    <div class='col'>
            <div class='titleRow'>
                <h3>Right: Colnect Candidate</h3>
                <span id='rightPos' class='badge'></span>
            </div>
                        <div id='rightActionBanner' class='rightActionBanner'></div>
      <div id='rightPanel'></div>
    </div>
  </div>
</div>

<div id='addModalBackdrop' class='modal-backdrop'>
    <div class='modal'>
        <h4>Add Colnect Candidate To stamps</h4>
        <div class='rowx'>
            <label><span id='addCatalogLabel'>Catalog</span> <input id='addSg' type='text' /></label>
            <label>Year <input id='addYear' type='number' /></label>
            <label>Series <input id='addSeries' type='text' /></label>
        </div>
        <div class='rowx'>
            <label><input id='addAssociateImage' type='checkbox' checked onchange='toggleAddImageOptions()'/> Associate image</label>
        </div>
        <div id='addImageOptions'>
            <div class='rowx'>
                <label><input type='radio' name='addImageMode' value='current_left' checked onchange='toggleAddImageOptions()'/> Use current left image</label>
                <label><input type='radio' name='addImageMode' value='upload' onchange='toggleAddImageOptions()'/> Upload image file</label>
                <label><input type='radio' name='addImageMode' value='none' onchange='toggleAddImageOptions()'/> No image</label>
            </div>
            <div class='rowx' id='uploadRow' style='display:none;'>
                <label>Upload <input id='addUploadFile' type='file' accept='image/png,image/jpeg,image/webp'/></label>
            </div>
        </div>
        <div class='rowx' style='justify-content:flex-end; margin-top: 12px;'>
            <button onclick='closeAddDialog()'>Cancel</button>
            <button onclick='addCurrentColnectAsStamp()'>Add</button>
        </div>
    </div>
</div>

<div id='leftEditorBackdrop' class='modal-backdrop'>
    <div class='modal leftEditorDialog leftEditorCompact'>
        <div class='leftEditorHead'>
            <h4 id='leftEditorTitle'>Edit Base Stamp</h4>
            <p id='leftEditorHint' class='leftEditorHint'>Use this for StampWorld corrections, watermark/reference images, or fixing a bad Colnect placement.</p>
        </div>
        <div class='leftEditorBody'>
            <section class='leftEditorSection'>
                <h5>Base Stamp Details</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorGrid'>
                        <label>Catalog # / Ref Code <input id='leftEditorSg' type='text' /></label>
                        <label>Catalog Type Search <input id='leftEditorCatalogCodeSearch' type='text' list='leftEditorCatalogCodeSearchList' placeholder='Type to filter catalogs...' oninput='filterLeftEditorCatalogTypeOptions()' onchange='applyLeftEditorCatalogSearchChoice()' onblur='tryApplyLeftEditorCatalogSearchChoice()' /></label>
                        <datalist id='leftEditorCatalogCodeSearchList'></datalist>
                        <label>Catalog Type
                            <select id='leftEditorCatalogCodeSelect' onchange='toggleLeftEditorCatalogCodeCustom()'>
                                <option value='sg'>Stanley Gibbons (SG)</option>
                                <option value='sn'>Scott Number (SN)</option>
                                <option value='mi'>Michel (MI)</option>
                                <option value='yt'>Yvert (YT)</option>
                                <option value='afa'>AFA</option>
                                <option value='nvp'>NVP</option>
                                <option value='un'>Unificato (UN)</option>
                                <option value='col'>Colnect (COL)</option>
                                <option value='custom'>Custom...</option>
                            </select>
                        </label>
                        <label id='leftEditorCatalogCodeCustomWrap' style='display:none;'>Custom Catalog Type Code <input id='leftEditorCatalogCodeCustom' type='text' placeholder='e.g. zz' /></label>
                        <label>Year <input id='leftEditorYear' type='number' /></label>
                        <label>Series <input id='leftEditorSeries' type='text' /></label>
                    </div>
                    <div class='leftEditorStack'>
                        <label>Description <textarea id='leftEditorDescription'></textarea></label>
                        <label>Watermark <textarea id='leftEditorWatermark'></textarea></label>
                        <label>Design <textarea id='leftEditorDesign'></textarea></label>
                        <label>Engraving <textarea id='leftEditorEngraving'></textarea></label>
                        <label>Perforation <textarea id='leftEditorPerforation'></textarea></label>
                        <label>Context / Notes <textarea id='leftEditorContext'></textarea></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Dimensions</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorDimsGrid'>
                        <label>Width <input id='leftEditorWidth' type='text' placeholder='e.g. 25' /></label>
                        <label>Height <input id='leftEditorHeight' type='text' placeholder='e.g. 30' /></label>
                        <label>Colnect Width <input id='leftEditorColnectWidth' type='text' /></label>
                        <label>Colnect Height <input id='leftEditorColnectHeight' type='text' /></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Colnect Metadata</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorGrid'>
                        <label>Colnect ID <input id='leftEditorColnectId' type='text' /></label>
                        <label>Colnect Name <input id='leftEditorColnectName' type='text' /></label>
                        <label>Colnect Country <input id='leftEditorColnectCountry' type='text' /></label>
                        <label>Catalog Codes <input id='leftEditorColnectCatalogCodes' type='text' /></label>
                        <label>Colnect Page URL <input id='leftEditorColnectPageUrl' type='text' /></label>
                        <label>Colnect Image URL <input id='leftEditorColnectImageUrl' type='text' /></label>
                        <label>Format <input id='leftEditorColnectFormat' type='text' /></label>
                        <label>Emission <input id='leftEditorColnectEmission' type='text' /></label>
                        <label>Printing <input id='leftEditorColnectPrinting' type='text' /></label>
                        <label>Paper <input id='leftEditorColnectPaper' type='text' /></label>
                        <label>Gum <input id='leftEditorColnectGum' type='text' /></label>
                        <label>Face Value <input id='leftEditorColnectFaceValue' type='text' /></label>
                        <label>Currency <input id='leftEditorColnectCurrency' type='text' /></label>
                        <label>Ord MI <input id='leftEditorColnectOrdMi' type='text' /></label>
                        <label>Ord SN <input id='leftEditorColnectOrdSn' type='text' /></label>
                        <label>Ord YT <input id='leftEditorColnectOrdYt' type='text' /></label>
                        <label>Ord Catalog (primary) <input id='leftEditorColnectOrdSg' type='text' /></label>
                        <label>Expiry Date <input id='leftEditorColnectExpiryDate' type='text' /></label>
                        <label>Print Run <input id='leftEditorColnectPrintRun' type='text' /></label>
                        <label>Colors <input id='leftEditorColnectColors' type='text' /></label>
                        <label>Parent Item <input id='leftEditorColnectParentItem' type='text' /></label>
                        <label>Variant <input id='leftEditorColnectVariant' type='text' /></label>
                        <label>Overprint Variant <input id='leftEditorColnectOverprintVariant' type='text' /></label>
                        <label>Score <input id='leftEditorColnectScore' type='text' /></label>
                        <label>Accuracy <input id='leftEditorColnectAccuracy' type='text' /></label>
                        <label>List <input id='leftEditorColnectList' type='text' /></label>
                        <label>Quantity <input id='leftEditorColnectQuantity' type='text' /></label>
                        <label>Condition <input id='leftEditorColnectCondition' type='text' /></label>
                    </div>
                    <div class='leftEditorStack'>
                        <label>Public Note <textarea id='leftEditorColnectPublicNote'></textarea></label>
                        <label>Private Note <textarea id='leftEditorColnectPrivateNote'></textarea></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Image Handling</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorHint'>Items without an image will not stay visible in the left review list.</div>
                    <div class='leftEditorModes'>
                        <label id='leftModeKeepWrap'><input id='leftModeKeep' type='radio' name='leftImageMode' value='keep' checked onchange='toggleLeftEditorImageOptions()'/> Keep current image</label>
                        <label id='leftModeCurrentWrap'><input id='leftModeCurrent' type='radio' name='leftImageMode' value='current_left' onchange='toggleLeftEditorImageOptions()'/> Use current left image</label>
                        <label><input id='leftModeUpload' type='radio' name='leftImageMode' value='upload' onchange='toggleLeftEditorImageOptions()'/> Upload image</label>
                    </div>
                    <div id='leftUploadRow' class='rowx' style='display:none;'>
                        <label>Upload <input id='leftEditorUploadFile' type='file' accept='image/png,image/jpeg,image/webp'/></label>
                    </div>
                </div>
            </section>
        </div>
        <div class='leftEditorActions'>
            <button onclick='closeLeftEditor()'>Cancel</button>
            <button class='primary' onclick='submitLeftEditor()'>Save</button>
        </div>
    </div>
</div>
<div id='opBusyBackdrop' class='opBusyBackdrop'>
    <div class='opBusyCard'>
        <div class='opBusyRow'>
            <span class='opBusySpinner'></span>
            <div class='opBusyText'>
                <strong id='opBusyTitle'>Working<span id='opBusyDots' class='opBusyDots'></span></strong><br/>
                <span id='opBusyText'>Please wait...</span>
            </div>
        </div>
    </div>
</div>
<script>
const payload = {json};
let left = payload.left || [];
let rightAll = payload.right || [];
const reviewColnectDir = String(payload.colnect_dir || '');
let reviewColnectFileCount = Number(payload.colnect_file_count || 0);
const showImagelessLeftRowsDefault = payload.show_imageless_left_rows === true;
const configDefaultCatalogCode = String(payload.default_catalog_code || '').toLowerCase();
const countryCatalogStorageKey = 'reviewui.defaultCatalog.' + String(payload.album_name || 'default');
let countryDefaultCatalogCode = '';
let leftIdx = 0;
let rightIdxByLeft = {{}};
let assignedByLeft = {{}}; // left_rowid -> colnect_id
let selectedCatalogCodeByLeft = {{}}; // left_rowid -> chosen code
let selectedCatalogTypeByLeft = {{}}; // left_rowid -> chosen catalog family
let deletedCandidatesStack = []; // stack entries: candidate + removedAssignments
let guideDismissed = false;
let resolvingRightImageById = {{}};
let resolvedRightImageById = {{}};
let leftEditorMode = 'edit';
let leftEditorSourceRowid = null;
let sortMode = 'sg';
let leftSearchMatches = [];
let leftSearchMatchPos = -1;
let leftSearchQuery = '';
let rightSearchQuery = '';
let opBusyDotsTimer = null;
let runtimeLogCompareTimer = null;
let backendHealthCompareTimer = null;
let backendHealthCompareFailures = 0;
let backendHealthCompareCheckInFlight = false;
let backendCompareMarkedDisconnected = false;
let backendCompareRecoveredReloadPending = false;
const runtimeLogCompareVisibleStorageKey = 'reviewui.compare.showRuntimeLog';
const reviewPdfAlwaysOpenStorageKey = 'reviewui.pdf.always_open';
const backendHealthComparePollMs = 15000;
const backendHealthCompareFailureThreshold = 2;
let reviewLastGeneratedPdfPath = '';
window.__returnToStartInFlight = false;
const reviewPdfLastPathStorageKey = 'reviewui.pdf.last_path.' + String(payload.album_name || 'default');
const reviewPdfLastStatusStorageKey = 'reviewui.pdf.last_status.' + String(payload.album_name || 'default');

function persistPdfUiState(statusText, tone) {{
    try {{
        localStorage.setItem(reviewPdfLastPathStorageKey, String(reviewLastGeneratedPdfPath || ''));
        localStorage.setItem(reviewPdfLastStatusStorageKey, JSON.stringify({{
            text: String(statusText || ''),
            tone: String(tone || '')
        }}));
    }} catch (_) {{}}
}}

function restorePdfUiState() {{
    const statusEl = document.getElementById('pdfStatus');
    if (!statusEl) return;
    try {{
        reviewLastGeneratedPdfPath = String(localStorage.getItem(reviewPdfLastPathStorageKey) || '');
    }} catch (_) {{
        reviewLastGeneratedPdfPath = '';
    }}
    updatePdfPreviewControls();
    try {{
        const raw = localStorage.getItem(reviewPdfLastStatusStorageKey);
        if (!raw) return;
        const parsed = JSON.parse(raw);
        const text = String(parsed?.text || '').trim();
        const tone = String(parsed?.tone || '').trim();
        if (!text) return;
        statusEl.classList.remove('pdfStatusDone', 'pdfStatusFail');
        if (tone === 'done') statusEl.classList.add('pdfStatusDone');
        if (tone === 'fail') statusEl.classList.add('pdfStatusFail');
        statusEl.innerText = text;
    }} catch (_) {{}}
}}

function updatePdfPreviewControls() {{
    const previewBtn = document.getElementById('reviewPdfPreviewBtn');
    if (!previewBtn) return;
    const hasPdf = !!String(reviewLastGeneratedPdfPath || '').trim();
    previewBtn.style.display = hasPdf ? '' : 'none';
    previewBtn.disabled = !hasPdf;
}}

function togglePdfAlwaysOpenPreference() {{
    const cb = document.getElementById('reviewPdfAlwaysShow');
    if (!cb) return;
    try {{
        localStorage.setItem(reviewPdfAlwaysOpenStorageKey, cb.checked ? '1' : '0');
    }} catch (_) {{}}
}}

async function previewLastPdfReview() {{
    const statusEl = document.getElementById('pdfStatus');
    const spinnerEl = document.getElementById('pdfStatusSpinner');
    const path = String(reviewLastGeneratedPdfPath || '').trim();
    if (!path) {{
        if (statusEl) {{
            statusEl.classList.remove('pdfStatusDone');
            statusEl.classList.add('pdfStatusFail');
            statusEl.innerText = 'No generated PDF found yet. Generate PDF first.';
        }}
        return;
    }}

    if (statusEl) {{
        statusEl.classList.remove('pdfStatusDone', 'pdfStatusFail');
        statusEl.innerText = 'Opening PDF preview...';
    }}
    if (spinnerEl) spinnerEl.style.display = 'inline-block';

    try {{
        const r = await fetch('/open-local-path', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ path }})
        }});
        const j = await r.json();
        if (j.ok) {{
            if (statusEl) {{
                statusEl.classList.remove('pdfStatusFail');
                statusEl.classList.add('pdfStatusDone');
                statusEl.innerText = j.message || 'Opened PDF preview.';
            }}
        }} else {{
            if (statusEl) {{
                statusEl.classList.remove('pdfStatusDone');
                statusEl.classList.add('pdfStatusFail');
                statusEl.innerText = j.message || 'Failed to open PDF preview.';
            }}
        }}
    }} catch (e) {{
        if (statusEl) {{
            statusEl.classList.remove('pdfStatusDone');
            statusEl.classList.add('pdfStatusFail');
            statusEl.innerText = 'Failed to open PDF preview: ' + e;
        }}
    }} finally {{
        if (spinnerEl) spinnerEl.style.display = 'none';
    }}
}}

function setMainStatus(message) {{
    const statusEl = document.getElementById('status');
    if (statusEl) statusEl.innerText = message;
}}

function setMatchStatus(message) {{
    const matchEl = document.getElementById('matchActionStatus');
    if (matchEl) matchEl.innerText = message;
    setMainStatus(message);
}}

function setConfigStatus(message) {{
    const configEl = document.getElementById('configStatus');
    if (configEl) configEl.innerText = message;
    setMainStatus(message);
}}

window.setConfigStatus = setConfigStatus;

function reloadCurrentPageCompare() {{
    window.location.reload();
}}

function setReconnectBannerCompare(visible, title, text, recovered) {{
    const banner = document.getElementById('reconnectBannerCompare');
    const titleEl = document.getElementById('reconnectBannerCompareTitle');
    const textEl = document.getElementById('reconnectBannerCompareText');
    const metaEl = document.getElementById('reconnectBannerCompareMeta');
    if (!banner || !titleEl || !textEl || !metaEl) return;
    titleEl.innerText = title || 'Connection lost, retrying...';
    textEl.innerText = text || '{backend_not_responding_message}';
    metaEl.innerText = visible ? ('Checks every ' + Math.round(backendHealthComparePollMs / 1000) + 's') : '';
    banner.classList.toggle('show', !!visible);
    banner.classList.toggle('recovered', !!recovered);
}}

async function fetchBackendHealthCompare() {{
    const controller = typeof AbortController !== 'undefined' ? new AbortController() : null;
    const timeoutId = controller ? setTimeout(() => controller.abort(), 4000) : null;
    try {{
        const res = await fetch('/health', {{ cache: 'no-store', signal: controller ? controller.signal : undefined }});
        if (!res.ok) throw new Error('HTTP ' + res.status);
        const j = await res.json();
        if (!j || j.ok !== true) throw new Error('invalid health response');
        return j;
    }} finally {{
        if (timeoutId) clearTimeout(timeoutId);
    }}
}}

async function checkBackendHealthCompare() {{
    if (window.__reviewPdfRequestInFlight === true) {{
        backendHealthCompareFailures = 0;
        return;
    }}
    if (backendHealthCompareCheckInFlight) return;
    backendHealthCompareCheckInFlight = true;
    try {{
        await fetchBackendHealthCompare();
        backendHealthCompareFailures = 0;
        if (backendCompareMarkedDisconnected && !backendCompareRecoveredReloadPending) {{
            backendCompareRecoveredReloadPending = true;
            setReconnectBannerCompare(true, 'Connection restored', 'The backend is responding again. Reloading this page now.', true);
            setTimeout(() => window.location.reload(), 900);
        }} else if (!backendCompareRecoveredReloadPending) {{
            setReconnectBannerCompare(false, '', '', false);
        }}
        backendCompareMarkedDisconnected = false;
    }} catch (_) {{
        backendHealthCompareFailures += 1;
        if (backendHealthCompareFailures >= backendHealthCompareFailureThreshold) {{
            backendCompareMarkedDisconnected = true;
            backendCompareRecoveredReloadPending = false;
            setReconnectBannerCompare(
                true,
                'Connection lost, retrying...',
                   '{backend_not_responding_message}',
                false
            );
        }}
    }} finally {{
        backendHealthCompareCheckInFlight = false;
    }}
}}

function startBackendHealthPollingCompare() {{
    if (backendHealthCompareTimer) clearInterval(backendHealthCompareTimer);
    checkBackendHealthCompare();
    backendHealthCompareTimer = setInterval(checkBackendHealthCompare, backendHealthComparePollMs);
    window.addEventListener('online', checkBackendHealthCompare);
    document.addEventListener('visibilitychange', () => {{
        if (!document.hidden) checkBackendHealthCompare();
    }});
}}

function retryBackendHealthCheckCompare() {{
    checkBackendHealthCompare();
}}

function isRuntimeLogVisibleCompare() {{
    const cb = document.getElementById('showRuntimeLogCompare');
    return cb ? !!cb.checked : true;
}}

function applyRuntimeLogVisibilityCompare() {{
    const body = document.getElementById('runtimeLogCompareBody');
    if (!body) return;
    body.style.display = isRuntimeLogVisibleCompare() ? '' : 'none';
}}

function toggleRuntimeLogVisibilityCompare() {{
    applyRuntimeLogVisibilityCompare();
    try {{
        localStorage.setItem(runtimeLogCompareVisibleStorageKey, isRuntimeLogVisibleCompare() ? '1' : '0');
    }} catch (_) {{}}
}}

function copyTextToClipboardCompare(text) {{
    if (navigator.clipboard && navigator.clipboard.writeText) {{
        return navigator.clipboard.writeText(text);
    }}
    return new Promise((resolve, reject) => {{
        try {{
            const ta = document.createElement('textarea');
            ta.value = text;
            ta.style.position = 'fixed';
            ta.style.left = '-9999px';
            document.body.appendChild(ta);
            ta.focus();
            ta.select();
            document.execCommand('copy');
            document.body.removeChild(ta);
            resolve();
        }} catch (e) {{
            reject(e);
        }}
    }});
}}

function ensureCopyableAlertDialogCompare() {{
    if (document.getElementById('copyAlertDialogCompare')) return;
    const dlg = document.createElement('dialog');
    dlg.id = 'copyAlertDialogCompare';
    dlg.style.maxWidth = '760px';
    dlg.style.width = '92vw';
    dlg.innerHTML =
        "<div style='display:flex;flex-direction:column;gap:10px;'>" +
        "<h3 id='copyAlertTitleCompare' style='margin:0;'>Message</h3>" +
        "<textarea id='copyAlertTextCompare' readonly style='width:100%;min-height:180px;resize:vertical;padding:8px;border:1px solid #c8d5e2;border-radius:8px;font-family:monospace;font-size:12px;'></textarea>" +
        "<div style='display:flex;gap:8px;justify-content:flex-end;'>" +
        "<button id='copyAlertCopyBtnCompare'>Copy</button>" +
        "<button id='copyAlertCloseBtnCompare'>Close</button>" +
        "</div></div>";
    document.body.appendChild(dlg);
    dlg.querySelector('#copyAlertCloseBtnCompare').addEventListener('click', () => dlg.close());
    dlg.querySelector('#copyAlertCopyBtnCompare').addEventListener('click', async () => {{
        const text = dlg.querySelector('#copyAlertTextCompare')?.value || '';
        try {{
            await copyTextToClipboardCompare(text);
        }} catch (_) {{}}
    }});
}}

function showCopyableAlertCompare(message, title) {{
    ensureCopyableAlertDialogCompare();
    const dlg = document.getElementById('copyAlertDialogCompare');
    const titleEl = document.getElementById('copyAlertTitleCompare');
    const textEl = document.getElementById('copyAlertTextCompare');
    if (!dlg || !textEl || !titleEl) return;
    titleEl.innerText = title || 'Message';
    textEl.value = String(message || '');
    if (!dlg.open) dlg.showModal();
    textEl.focus();
    textEl.select();
}}

window.alert = function(message) {{
    showCopyableAlertCompare(message, 'Message');
}};

async function refreshRuntimeLogCompare() {{
    try {{
        if (!isRuntimeLogVisibleCompare()) return;
        const r = await fetch('/runtime-log');
        const j = await r.json();
        const box = document.getElementById('runtimeLogCompare');
        if (!box) return;
        const lines = Array.isArray(j?.lines) ? j.lines : [];
        const atBottom = (box.scrollTop + box.clientHeight + 20) >= box.scrollHeight;
        box.value = lines.join('\n');
        if (atBottom) box.scrollTop = box.scrollHeight;
    }} catch (_) {{}}
}}

function startRuntimeLogPollingCompare() {{
    if (runtimeLogCompareTimer) clearInterval(runtimeLogCompareTimer);
    refreshRuntimeLogCompare();
    runtimeLogCompareTimer = setInterval(refreshRuntimeLogCompare, 1200);
}}

async function copyRuntimeLogCompare() {{
    const box = document.getElementById('runtimeLogCompare');
    const statusEl = document.getElementById('status');
    if (!box) return;
    try {{
        await copyTextToClipboardCompare(box.value || '');
        if (statusEl) statusEl.innerText = 'Runtime log copied to clipboard.';
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Failed to copy runtime log: ' + e;
    }}
}}

function setOperationBusy(active, message) {{
    const backdrop = document.getElementById('opBusyBackdrop');
    const text = document.getElementById('opBusyText');
    const dots = document.getElementById('opBusyDots');
    if (!backdrop || !text || !dots) return;

    if (!active) {{
        backdrop.style.display = 'none';
        if (opBusyDotsTimer) {{
            clearInterval(opBusyDotsTimer);
            opBusyDotsTimer = null;
        }}
        dots.innerText = '';
        return;
    }}

    text.innerText = message || 'Please wait...';
    backdrop.style.display = 'flex';
    if (!opBusyDotsTimer) {{
        let idx = 0;
        const frames = ['', '.', '..', '...'];
        opBusyDotsTimer = setInterval(() => {{
            idx = (idx + 1) % frames.length;
            dots.innerText = frames[idx];
        }}, 260);
    }}
}}

async function navigateWithFade(url) {{
    try {{
        if (document.body) document.body.classList.add('pageLeaving');
        await new Promise(resolve => setTimeout(resolve, 140));
    }} catch (_) {{}}
    window.location.href = url;
}}

async function waitForBootstrapReady(maxAttempts, delayMs) {{
    for (let i = 0; i < maxAttempts; i++) {{
        const controller = new AbortController();
        const timer = setTimeout(() => controller.abort(), Math.max(350, delayMs));
        try {{
            const res = await fetch('/bootstrap-state', {{
                cache: 'no-store',
                signal: controller.signal,
            }});
            if (res.ok) return true;
        }} catch (_) {{
            // Ignore transient network errors while the server switches modes.
        }} finally {{
            clearTimeout(timer);
        }}
        await new Promise(resolve => setTimeout(resolve, delayMs));
    }}
    return false;
}}

function removeCandidateFromUi(colnectId) {{
    if (!colnectId) return;
    rightAll = rightAll.filter(c => c.colnect_id !== colnectId);
    Object.keys(assignedByLeft).forEach(k => {{
        if (assignedByLeft[k] === colnectId) delete assignedByLeft[k];
    }});
    Object.keys(selectedCatalogCodeByLeft).forEach(k => {{
        if (!assignedByLeft[k]) delete selectedCatalogCodeByLeft[k];
    }});
    Object.keys(selectedCatalogTypeByLeft).forEach(k => {{
        if (!assignedByLeft[k]) delete selectedCatalogTypeByLeft[k];
    }});
    for (const key of Object.keys(rightIdxByLeft)) {{
        rightIdxByLeft[key] = 0;
    }}
    for (let i = deletedCandidatesStack.length - 1; i >= 0; i -= 1) {{
        if (deletedCandidatesStack[i]?.candidate?.colnect_id === colnectId) {{
            deletedCandidatesStack.splice(i, 1);
        }}
    }}
}}
const rowOddColor = (payload.odd_row_color || '').trim() || '#ffffff';
const rowEvenColor = (payload.even_row_color || '').trim() || '#e6eef4';
const checkboxDefaults = {{
  f_series: payload.series_default_checked !== false,
  f_description: payload.description_default_checked !== false,
  f_watermark: payload.watermark_default_checked !== false,
  f_design: payload.design_default_checked !== false,
  f_engraving: payload.engraving_default_checked !== false,
  f_perforation: payload.perforation_default_checked !== false,
  f_context: payload.context_default_checked !== false,
  f_links: payload.colnect_links_default_checked !== false,
  f_image_url: payload.colnect_image_url_default_checked !== false,
  f_colnect_meta: payload.colnect_metadata_default_checked !== false,
  f_replace_sg: payload.replace_sg_number_default_checked !== false,
  f_right_unmatched_only: payload.right_unmatched_only_default_checked !== false
}};
const pdfGenerationDefaults = {{
    enabledByDefault: payload.albumeasy_auto_generate_pdf_default !== false,
    cliConfigured: payload.albumeasy_cli_configured !== false
}};
document.documentElement.style.setProperty('--row-odd-bg', rowOddColor);
document.documentElement.style.setProperty('--row-even-bg', rowEvenColor);

function b(v) {{ return !!v && String(v).trim().length > 0; }}
function fmt(v) {{ return b(v) ? String(v) : '(no info)'; }}
function html(v) {{ return String(v)
  .replaceAll('&','&amp;')
  .replaceAll('<','&lt;')
  .replaceAll('>','&gt;')
  .replaceAll('"','&quot;'); }}
function row(k, v) {{ return `<tr><td class='k'>${{html(k)}}</td><td class='v'>${{html(fmt(v))}}</td></tr>`; }}
function allDbRows(fields) {{
    if (!Array.isArray(fields) || !fields.length) return row('DB Fields', '(no fields)');
    return fields.map(f => row(`DB:${{f.key || ''}}`, f.value || '')).join('');
}}
function toggleImagelessLeftRows() {{
    const checkbox = document.getElementById('f_show_imageless_left');
    const url = new URL(window.location.href);
    if (checkbox?.checked) {{
        url.searchParams.set('show_no_image', '1');
    }} else {{
        url.searchParams.delete('show_no_image');
    }}
    window.location.href = url.pathname + url.search;
}}

function parseConfiguredCatalogFamilyOptions(entries) {{
    const parsed = [];
    for (const raw of (Array.isArray(entries) ? entries : [])) {{
        const line = String(raw || '').trim();
        if (!line) continue;
        const parts = line.split('|').map(v => String(v || '').trim());
        if (!parts[0] || !parts[1]) continue;
        const code = parts[0].toLowerCase();
        const label = parts[1];
        const count = parts[2] ? Number(String(parts[2]).replaceAll(',', '')) : 0;
        parsed.push({{ code, label, count: Number.isFinite(count) ? count : 0 }});
    }}
    return parsed;
}}

function defaultCatalogFamilyOptions() {{
    return [
        {{ code: 'afa', label: 'AFA number', count: 116770 }},
        {{ code: 'afm', label: 'Afinsa-Mundifil', count: 17227 }},
        {{ code: 'ank', label: 'ANK', count: 8320 }},
        {{ code: 'barefoot', label: 'Barefoot', count: 17982 }},
        {{ code: 'belaruspost', label: 'Belarus post Inc.', count: 2530 }},
        {{ code: 'belgium', label: 'Belgium', count: 26787 }},
        {{ code: 'bertossa', label: 'Bertossa', count: 2686 }},
        {{ code: 'bolaffi', label: 'Bolaffi', count: 2354 }},
        {{ code: 'bw', label: 'Brusden-White', count: 1319 }},
        {{ code: 'bustamante', label: 'Bustamante', count: 2952 }},
        {{ code: 'campbellpaterson', label: 'Campbell Paterson', count: 6974 }},
        {{ code: 'cps', label: 'Chilean Philatelic Society', count: 3018 }},
        {{ code: 'css', label: 'China Stamp Society', count: 3271 }},
        {{ code: 'ciardi', label: 'Ciardi', count: 7451 }},
        {{ code: 'col', label: 'Colnect codes', count: 445975 }},
        {{ code: 'croatianpost', label: 'Croatian post Inc.', count: 1648 }},
        {{ code: 'dainippon', label: 'Dai Nippon', count: 1610 }},
        {{ code: 'durland', label: 'Durland', count: 7995 }},
        {{ code: 'edifil', label: 'Edifil', count: 15000 }},
        {{ code: 'facit', label: 'Facit', count: 7964 }},
        {{ code: 'farahbakhsh', label: 'Farahbakhsh', count: 4292 }},
        {{ code: 'gotigjalil', label: 'Gotig and Jalil', count: 5772 }},
        {{ code: 'gz', label: 'Gz (Cefiloza)', count: 9845 }},
        {{ code: 'hectormena', label: 'Hector Mena', count: 202 }},
        {{ code: 'hibernian', label: 'Hibernian', count: 3104 }},
        {{ code: 'iscmalaysia', label: 'ISC Malaysia', count: 5178 }},
        {{ code: 'isfila', label: 'Isfila', count: 1780 }},
        {{ code: 'ipf', label: 'Israel Philatelic Federation', count: 2256 }},
        {{ code: 'karamitsos', label: 'Karamitsos', count: 5526 }},
        {{ code: 'lapefinland', label: 'LaPe Finland', count: 5046 }},
        {{ code: 'leotemprano', label: 'Leo Temprano', count: 3336 }},
        {{ code: 'markapmr', label: 'Markapmr.ru', count: 1681 }},
        {{ code: 'maury', label: 'Maury', count: 1730 }},
        {{ code: 'mi', label: 'Michel', count: 1126331 }},
        {{ code: 'ngo', label: 'Ngo', count: 1753 }},
        {{ code: 'nilepost', label: 'Nile Post', count: 164 }},
        {{ code: 'norgeskatalogen', label: 'Norgeskatalogen', count: 3079 }},
        {{ code: 'nvp', label: 'NVPH', count: 18985 }},
        {{ code: 'philaindia', label: 'Phila-India', count: 95 }},
        {{ code: 'phh', label: 'Philatelia Hungarica Catalog', count: 24472 }},
        {{ code: 'pofis', label: 'POFIS', count: 14007 }},
        {{ code: 'fischer', label: 'Polish Stamps Catalog (Fischer)', count: 13603 }},
        {{ code: 'rhm', label: 'RHM', count: 6129 }},
        {{ code: 'romfilatelia', label: 'Romfilatelia', count: 7402 }},
        {{ code: 'sakura', label: 'Sakura', count: 14636 }},
        {{ code: 'sassone', label: 'Sassone', count: 18777 }},
        {{ code: 'sevenseas', label: 'Seven Seas Stamps', count: 15550 }},
        {{ code: 'sfk', label: 'SFK Serbia', count: 2820 }},
        {{ code: 'siddiqui', label: 'Siddiqui Catalogue', count: 1644 }},
        {{ code: 'slovenika', label: 'Slovenika', count: 2270 }},
        {{ code: 'soloviev', label: 'Soloviev', count: 7479 }},
        {{ code: 'safrican', label: 'South African Stamp Catalog', count: 3640 }},
        {{ code: 'sn', label: 'Scott Number', count: 814224 }},
        {{ code: 'sg', label: 'Stanley Gibbons', count: 705728 }},
        {{ code: 'thaistamp', label: 'Thai Stamp Catalogue', count: 3615 }},
        {{ code: 'ukrposhta', label: 'Ukrposhta', count: 3307 }},
        {{ code: 'un', label: 'Unificato', count: 169053 }},
        {{ code: 'vandam', label: 'Van Dam', count: 2613 }},
        {{ code: 'wad', label: 'WADP Numbering System - WNS', count: 91196 }},
        {{ code: 'wsp', label: 'World Stamps Project', count: 3152 }},
        {{ code: 'yang', label: 'Yang', count: 871 }},
        {{ code: 'yt', label: 'Yvert et Tellier', count: 925704 }},
        {{ code: 'zagorskiy', label: 'Zagorskiy', count: 14287 }},
        {{ code: 'zimbrul', label: 'Zimbrul Carpatin', count: 1567 }},
        {{ code: 'zonnebloem', label: 'Zonnebloem', count: 4948 }},
        {{ code: 'zumstein', label: 'Zumstein', count: 8559 }}
    ];
}}

const reviewCatalogFamilies = (() => {{
    const merged = new Map();
    for (const item of defaultCatalogFamilyOptions()) merged.set(item.code, item);
    for (const item of parseConfiguredCatalogFamilyOptions(payload.catalog_family_options)) merged.set(item.code, item);
    return Array.from(merged.values()).sort((a, b) => {{
        const ac = Number(a?.count || 0);
        const bc = Number(b?.count || 0);
        if (bc !== ac) return bc - ac;
        return String(a?.label || '').localeCompare(String(b?.label || ''));
    }});
}})();
const reviewCatalogFamilyByCode = (() => {{
    const m = new Map();
    for (const item of reviewCatalogFamilies) m.set(item.code, item);
    return m;
}})();
const reviewCatalogKnownCodes = new Set(reviewCatalogFamilies.map(v => v.code));

function catalogDisplayName(code) {{
    const c = String(code || '').trim().toLowerCase();
    const known = reviewCatalogFamilyByCode.get(c);
    if (known && known.label) return known.label;
    return c ? c.toUpperCase() : 'Catalog';
}}

function normalizeCatalogCodeForUi(code) {{
    const c = String(code || '').trim().toLowerCase();
    return c;
}}

function getEffectiveDefaultCatalogCode() {{
    return normalizeCatalogCodeForUi(countryDefaultCatalogCode)
        || normalizeCatalogCodeForUi(configDefaultCatalogCode)
        || 'sg';
}}

function changeCountryDefaultCatalog() {{
    const sel = document.getElementById('defaultCatalogSelect');
    if (!sel) return;
    countryDefaultCatalogCode = normalizeCatalogCodeForUi(sel.value) || 'sg';
    try {{
        localStorage.setItem(countryCatalogStorageKey, countryDefaultCatalogCode);
    }} catch (_) {{}}
    render();
}}

async function saveCatalogDefaultToConfig() {{
    const sel = document.getElementById('defaultCatalogSelect');
    const code = normalizeCatalogCodeForUi(sel?.value) || '';
    if (!code) {{
        alert('Choose a catalog first.');
        return;
    }}

    setConfigStatus('Updating config default catalog...');
    try {{
        const r = await fetch('/api/default-catalog-config', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ default_catalog_code: code }})
        }});
        const j = await r.json();
        if (j.ok) {{
            setConfigStatus(j.message || 'Config default updated');
            alert(j.message || 'Config updated.');
        }} else {{
            setConfigStatus('Config update failed');
            alert(j.message || 'Failed to update config default.');
        }}
    }} catch (e) {{
        setConfigStatus('Config update error');
        alert('Config update error: ' + e);
    }}
}}
function normalizeCatalogNumber(v) {{
    let s = String(v || '').trim();
    if (!s) return '';
    const colonIdx = s.indexOf(':');
    if (colonIdx >= 0) s = s.slice(colonIdx + 1).trim();
    // Match 2-3 letter country prefixes like "NO 1" / "CA. 454".
    const m = s.match(/^([A-Za-z]{{2,3}})\.?\s+(.+)$/);
    if (m && m[2]) return String(m[2]).trim();
    return s;
}}

function parseCatalogCodePairs(raw) {{
    const text = String(raw || '');
    if (!text.trim()) return [];
    const parts = text.split(/[|,]/);
    const out = [];
    for (const part of parts) {{
        const trimmed = String(part || '').trim();
        if (!trimmed) continue;
        const idx = trimmed.indexOf(':');
        if (idx <= 0) continue;
        const catalogType = String(trimmed.slice(0, idx)).trim().toLowerCase();
        const code = String(trimmed.slice(idx + 1)).trim();
        if (!catalogType || !code) continue;
        out.push({{ catalogType, code }});
    }}
    return out;
}}

function inferCatalogTypeForNumber(rawCatalogCodes, displayedNumber, fallbackCatalogCode) {{
    const normalizedDisplay = normalizeCatalogNumber(displayedNumber);
    const pairs = parseCatalogCodePairs(rawCatalogCodes);
    for (const pair of pairs) {{
        if (normalizeCatalogNumber(pair.code) === normalizedDisplay) return pair.catalogType;
    }}
    return normalizeCatalogCodeForUi(fallbackCatalogCode) || '';
}}

function effectiveCatalogCodeForItem(item) {{
    const explicit = normalizeCatalogCodeForUi(item?.catalog_code);
    if (explicit) return explicit;
    const inferred = inferCatalogTypeForNumber(
        item?.colnect_catalog_codes,
        item?.sg_number,
        getEffectiveDefaultCatalogCode()
    );
    return inferred || getEffectiveDefaultCatalogCode();
}}
function imageBox(src, label) {{
  if (b(src)) return `<div class='imgbox'><img src='${{src}}' alt='${{html(label)}}'/></div>`;
  return `<div class='imgbox'><div class='imgph'>${{html(label)}}<br/>No image available</div></div>`;
}}

function leftSortKey(item) {{
    const year = Number(item?.year || 0);
    const sg = String(item?.sg_number || '');
    return [year, sg.toLowerCase(), Number(item?.rowid || 0)];
}}

function sgSortText(item) {{
    return normalizeCatalogNumber(String(item?.sg_number || '')).toLowerCase();
}}

function compareBySortMode(a, b, mode) {{
    const m = String(mode || 'sg');
    if (m === 'rowid') {{
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}
    if (m === 'year_sg') {{
        const ay = Number(a?.year || 0);
        const by = Number(b?.year || 0);
        if (ay !== by) return ay - by;
        const asg = sgSortText(a);
        const bsg = sgSortText(b);
        if (asg !== bsg) return asg.localeCompare(bsg, undefined, {{ numeric: true, sensitivity: 'base' }});
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}
    if (m === 'series_sg') {{
        const aser = String(a?.series || '').toLowerCase();
        const bser = String(b?.series || '').toLowerCase();
        if (aser !== bser) return aser.localeCompare(bser, undefined, {{ numeric: true, sensitivity: 'base' }});
        const asg = sgSortText(a);
        const bsg = sgSortText(b);
        if (asg !== bsg) return asg.localeCompare(bsg, undefined, {{ numeric: true, sensitivity: 'base' }});
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}
    if (m === 'colnect_id') {{
        const acid = String(a?.colnect_id || '').toLowerCase();
        const bcid = String(b?.colnect_id || '').toLowerCase();
        if (acid !== bcid) return acid.localeCompare(bcid, undefined, {{ numeric: true, sensitivity: 'base' }});
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}

    const asg = sgSortText(a);
    const bsg = sgSortText(b);
    if (asg !== bsg) return asg.localeCompare(bsg, undefined, {{ numeric: true, sensitivity: 'base' }});
    const ay = Number(a?.year || 0);
    const by = Number(b?.year || 0);
    if (ay !== by) return ay - by;
    return Number(a?.rowid || 0) - Number(b?.rowid || 0);
}}

function sortLeftRows(preferredRowid) {{
    left.sort((a, b) => compareBySortMode(a, b, sortMode));
    if (!left.length) {{
        leftIdx = 0;
        updateLeftSearchMatches(true);
        return;
    }}
    const idx = preferredRowid == null ? -1 : left.findIndex(item => String(item.rowid) === String(preferredRowid));
    if (idx >= 0) {{
        leftIdx = idx;
    }} else if (leftIdx >= left.length) {{
        leftIdx = left.length - 1;
    }}
    updateLeftSearchMatches(true);
}}

function upsertLeftRow(updatedLeft) {{
    if (!updatedLeft || updatedLeft.rowid == null) return;
    const idx = left.findIndex(item => String(item.rowid) === String(updatedLeft.rowid));
    if (idx >= 0) left[idx] = updatedLeft;
    else left.push(updatedLeft);
    sortLeftRows(updatedLeft.rowid);
}}

function removeLeftRow(rowid) {{
    const before = left.length;
    left = left.filter(item => String(item.rowid) !== String(rowid));
    if (left.length !== before) sortLeftRows(null);
}}

function setLeftSearchStatus(text) {{
    const el = document.getElementById('leftSearchStatus');
    if (el) el.innerText = text;
}}

function leftSearchText(item) {{
    const parts = [
        item?.sg_number,
        item?.series,
        item?.description,
        item?.watermark,
        item?.context,
        item?.colnect_id,
        item?.colnect_name,
        item?.colnect_catalog_codes,
        item?.colnect_country,
        item?.year,
        item?.rowid,
    ];
    return parts
        .map(v => String(v || '').toLowerCase())
        .join(' | ');
}}

function rightSearchText(item) {{
    const parts = [
        item?.colnect_id,
        item?.sg_number,
        item?.series,
        item?.name,
        item?.country,
        item?.catalog_codes_text,
        item?.description,
        item?.watermark,
        item?.format,
        item?.emission,
        item?.printing,
        item?.paper,
        item?.gum,
        item?.context,
        item?.year,
    ];
    return parts
        .map(v => String(v || '').toLowerCase())
        .join(' | ');
}}

function setRightSearchStatus(text) {{
    const el = document.getElementById('rightSearchStatus');
    if (el) el.innerText = text;
}}

function updateRightSearchStatus() {{
    const l = left[leftIdx];
    if (!l) {{
        setRightSearchStatus('Matches: 0/0');
        return;
    }}
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) {{
        setRightSearchStatus('Matches: 0/0');
        return;
    }}
    const key = String(l.rowid);
    if (!(key in rightIdxByLeft) || rightIdxByLeft[key] >= avail.length) rightIdxByLeft[key] = 0;
    setRightSearchStatus(`Matches: ${{(rightIdxByLeft[key] || 0) + 1}}/${{avail.length}}`);
}}

function onRightSearchInput() {{
    const input = document.getElementById('rightSearchInput');
    const selStart = input ? input.selectionStart : null;
    const selEnd   = input ? input.selectionEnd   : null;
    if (input) rightSearchQuery = String(input.value || '');
    const l = left[leftIdx];
    if (l) rightIdxByLeft[String(l.rowid)] = 0;
    render();
    const newInput = document.getElementById('rightSearchInput');
    if (newInput) {{
        newInput.focus();
        if (selStart !== null) newInput.setSelectionRange(selStart, selEnd);
    }}
}}

function updateLeftSearchMatches(preserveCurrentSelection) {{
    const input = document.getElementById('leftSearchInput');
    if (input) leftSearchQuery = String(input.value || '');
    const query = String(leftSearchQuery || '').trim().toLowerCase();

    if (!left.length) {{
        leftSearchMatches = [];
        leftSearchMatchPos = -1;
        setLeftSearchStatus('Matches: 0/0');
        return;
    }}

    if (!query) {{
        leftSearchMatches = left.map((_, idx) => idx);
    }} else {{
        leftSearchMatches = left
            .map((item, idx) => (leftSearchText(item).includes(query) ? idx : -1))
            .filter(idx => idx >= 0);
    }}

    if (!leftSearchMatches.length) {{
        leftSearchMatchPos = -1;
        setLeftSearchStatus('Matches: 0/0');
        return;
    }}

    if (preserveCurrentSelection) {{
        const currentPos = leftSearchMatches.indexOf(leftIdx);
        if (currentPos >= 0) {{
            leftSearchMatchPos = currentPos;
            setLeftSearchStatus(`Matches: ${{leftSearchMatchPos + 1}}/${{leftSearchMatches.length}}`);
            return;
        }}
    }}

    leftSearchMatchPos = 0;
    leftIdx = leftSearchMatches[0];
    setLeftSearchStatus(`Matches: 1/${{leftSearchMatches.length}}`);
}}

function onLeftSearchInput() {{
    const input = document.getElementById('leftSearchInput');
    if (input) leftSearchQuery = String(input.value || '');
    const before = leftIdx;
    updateLeftSearchMatches(true);
    if (leftIdx !== before) render({{ preserveRightScroll: true }});
}}

function captureSearchFocusState() {{
    const active = document.activeElement;
    if (!active) return null;
    const id = active.id;
    if (id !== 'leftSearchInput' && id !== 'rightSearchInput') return null;
    return {{
        id,
        start: typeof active.selectionStart === 'number' ? active.selectionStart : null,
        end: typeof active.selectionEnd === 'number' ? active.selectionEnd : null,
    }};
}}

function restoreSearchFocusState(state) {{
    if (!state || !state.id) return;
    const input = document.getElementById(state.id);
    if (!input) return;
    input.focus();
    if (state.start !== null) {{
        try {{
            input.setSelectionRange(state.start, state.end);
        }} catch (_) {{}}
    }}
}}

function stepLeftSearch(delta) {{
    updateLeftSearchMatches(true);
    if (!leftSearchMatches.length) return;

    const total = leftSearchMatches.length;
    if (leftSearchMatchPos < 0) leftSearchMatchPos = 0;
    leftSearchMatchPos = (leftSearchMatchPos + delta + total) % total;
    leftIdx = leftSearchMatches[leftSearchMatchPos];
    setLeftSearchStatus(`Matches: ${{leftSearchMatchPos + 1}}/${{total}}`);
    render({{ preserveRightScroll: true }});
}}

async function ensureRightImageUrl(r) {{
    if (!r || !b(r.colnect_id) || !b(r.page_url)) return;
    if (b(r.image_abs) || b(r.image_url)) return;

    const key = String(r.colnect_id);
    if (key in resolvedRightImageById) {{
        r.image_url = resolvedRightImageById[key] || '';
        return;
    }}
    if (resolvingRightImageById[key]) return;

    resolvingRightImageById[key] = true;
    try {{
        const res = await fetch(`/resolve-colnect-image?page_url=${{encodeURIComponent(r.page_url)}}`);
        const j = await res.json();
        const url = (j && j.ok && b(j.image_url)) ? String(j.image_url) : '';
        resolvedRightImageById[key] = url;
        r.image_url = url;

        const current = currentRightCandidate();
        if (current && String(current.colnect_id) === key) {{
            render();
        }}
    }} catch (_) {{
        resolvedRightImageById[key] = '';
    }} finally {{
        delete resolvingRightImageById[key];
    }}
}}

function dismissStartGuide() {{
    guideDismissed = true;
    const guide = document.getElementById('startGuide');
    if (guide) guide.style.display = 'none';
}}

function scrollToFilters() {{
    document.getElementById('filtersCard')?.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
}}

function scrollToPanels() {{
    document.getElementById('comparePanels')?.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
}}

function updateGuide() {{
    const leftCount = left.length;
    const rightCount = rightAll.length;
    const assignedCount = Object.keys(assignedByLeft).length;

    const leftBadge = document.getElementById('statLeftCount');
    const rightBadge = document.getElementById('statRightCount');
    const assignedBadge = document.getElementById('statAssignedCount');
    if (leftBadge) leftBadge.innerText = `Left: ${{leftCount}}`;
    if (rightBadge) rightBadge.innerText = `Candidates: ${{rightCount}}`;
    if (assignedBadge) assignedBadge.innerText = `Assigned: ${{assignedCount}}`;

    const guide = document.getElementById('startGuide');
    if (!guide) return;
    if (guideDismissed) {{
        guide.style.display = 'none';
        return;
    }}
    guide.style.display = '';
}}

function usedColnectIds(exceptLeftRowId) {{
  const used = new Set();
  for (const [l, cid] of Object.entries(assignedByLeft)) {{
    if (String(exceptLeftRowId) === String(l)) continue;
    used.add(cid);
  }}
  return used;
}}

function availableCandidatesForCurrentLeft() {{
  if (!left.length) return [];
  const currentLeft = left[leftIdx];
  const used = usedColnectIds(currentLeft.rowid);
  const assignedForCurrent = assignedByLeft[currentLeft.rowid];
    const unmatchedOnly = !!document.getElementById('f_right_unmatched_only')?.checked;
    const matchedInDb = new Set(left.map(s => String(s.colnect_id || '').trim()).filter(v => !!v));
    const filtered = rightAll.filter(c => {{
        const keepUsed = (!used.has(c.colnect_id) || c.colnect_id === assignedForCurrent);
        if (!keepUsed) return false;
        if (!unmatchedOnly) return true;
        if (c.colnect_id === assignedForCurrent) return true;
        return !matchedInDb.has(String(c.colnect_id || '').trim());
    }});
    filtered.sort((a, b) => compareBySortMode(a, b, sortMode));

    const query = String(rightSearchQuery || '').trim().toLowerCase();
    if (!query) return filtered;
    return filtered.filter(item => rightSearchText(item).includes(query));
}}

function normalizeIndex(cur, len) {{
    if (len <= 0) return 0;
    if (!Number.isFinite(cur)) return 0;
    if (cur < 0) return 0;
    if (cur >= len) return len - 1;
    return cur;
}}

function jumpLeft(delta) {{
    if (!left.length) return;
    leftIdx = normalizeIndex(leftIdx + Number(delta || 0), left.length);
    render({{ preserveRightScroll: true }});
}}

function firstLeft() {{
    if (!left.length) return;
    leftIdx = 0;
    render({{ preserveRightScroll: true }});
}}

function lastLeft() {{
    if (!left.length) return;
    leftIdx = left.length - 1;
    render({{ preserveRightScroll: true }});
}}

function jumpRight(delta) {{
    const l = left[leftIdx];
    if (!l) return;
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) return;
    const key = String(l.rowid);
    const cur = rightIdxByLeft[key] || 0;
    rightIdxByLeft[key] = normalizeIndex(cur + Number(delta || 0), avail.length);
    render();
}}

function firstRight() {{
    const l = left[leftIdx];
    if (!l) return;
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) return;
    rightIdxByLeft[String(l.rowid)] = 0;
    render();
}}

function lastRight() {{
    const l = left[leftIdx];
    if (!l) return;
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) return;
    rightIdxByLeft[String(l.rowid)] = avail.length - 1;
    render();
}}

function changeSortMode() {{
    const sel = document.getElementById('sortMode');
    sortMode = String(sel?.value || 'sg');
    const currentRowid = left[leftIdx]?.rowid;
    sortLeftRows(currentRowid);
    for (const key of Object.keys(rightIdxByLeft)) rightIdxByLeft[key] = 0;
    render();
}}

function currentRightCandidate() {{
  const avail = availableCandidatesForCurrentLeft();
  if (!avail.length) return null;
  const key = String(left[leftIdx].rowid);
  if (!(key in rightIdxByLeft) || rightIdxByLeft[key] >= avail.length) rightIdxByLeft[key] = 0;
  return avail[rightIdxByLeft[key]];
}}

function selectedCatalogCodeForCurrent(r) {{
    const l = left[leftIdx];
    if (!l || !r) return '';
    const existing = selectedCatalogCodeByLeft[l.rowid] || '';
    if (existing) return existing;
    const pairs = r.catalog_code_pairs || [];
    if (!pairs.length) return '';
    const preferredCode = getEffectiveDefaultCatalogCode();
    const preferred = pairs.find(p => String(p.catalog_type || '').toLowerCase() === preferredCode);
    return preferred ? preferred.code : (pairs[0].code || '');
}}

function selectedCatalogTypeForCurrent(r) {{
    const l = left[leftIdx];
    if (!l || !r) return '';
    const existingType = selectedCatalogTypeByLeft[l.rowid] || '';
    if (existingType) return String(existingType).toLowerCase();
    const selectedCode = selectedCatalogCodeForCurrent(r);
    const pairs = r.catalog_code_pairs || [];
    if (!pairs.length) return '';
    const matched = pairs.find(p => String(p.code || '') === String(selectedCode || ''));
    if (matched && matched.catalog_type) return String(matched.catalog_type).toLowerCase();
    const preferredCode = getEffectiveDefaultCatalogCode();
    const preferred = pairs.find(p => String(p.catalog_type || '').toLowerCase() === preferredCode);
    if (preferred && preferred.catalog_type) return String(preferred.catalog_type).toLowerCase();
    return String(pairs[0].catalog_type || '').toLowerCase();
}}

function rightCatalogFamiliesForCandidate(r) {{
    const families = [];
    const seen = new Set();
    for (const pair of (r?.catalog_code_pairs || [])) {{
        const code = normalizeCatalogCodeForUi(pair?.catalog_type);
        if (!code || seen.has(code)) continue;
        seen.add(code);
        families.push(code);
    }}
    return families;
}}

function populateRightCatalogSearchSuggestions(items) {{
    const listEl = document.getElementById('catalogCodeSearchList');
    if (!listEl) return;
    listEl.innerHTML = (items || [])
        .slice(0, 200)
        .map(item => `<option value='${{html(catalogFamilySearchText(item))}}'></option>`)
        .join('');
}}

function applyRightCatalogSearchChoice() {{
    const inputEl = document.getElementById('catalogCodeSearchInput');
    const selectEl = document.getElementById('catalogCodeSelect');
    if (!inputEl || !selectEl) return;
    const code = parseCatalogCodeFromSearchValue(inputEl.value);
    if (!code) return;

    const specificOpt = Array.from(selectEl.options).find(opt => String(opt.dataset?.cat || '').toLowerCase() === code && !String(opt.value || '').startsWith('__family__'));
    if (specificOpt) {{
        selectEl.value = specificOpt.value;
        rememberCatalogCode();
        return;
    }}
    const familyValue = `__family__${{code}}`;
    if (Array.from(selectEl.options).some(opt => opt.value === familyValue)) {{
        selectEl.value = familyValue;
        rememberCatalogCode();
    }}
}}

function tryApplyRightCatalogSearchChoice() {{
    // Called on blur: silently try to extract and apply catalog code if possible
    const inputEl = document.getElementById('catalogCodeSearchInput');
    if (!inputEl || !inputEl.value.trim()) return;
    applyRightCatalogSearchChoice(); // Just call the regular function, it silently ignores if no code found
}}

function buildRightCatalogSelectOptions(r, selectedCode, selectedType) {{
    const query = String(document.getElementById('catalogCodeSearchInput')?.value || '').trim().toLowerCase();
    const pairs = Array.isArray(r?.catalog_code_pairs) ? r.catalog_code_pairs : [];
    const familySet = new Set(rightCatalogFamiliesForCandidate(r));
    const matchesQuery = (text) => !query || String(text || '').toLowerCase().includes(query);

    const stampSpecific = pairs.filter(p => {{
        const catCode = normalizeCatalogCodeForUi(p?.catalog_type);
        const text = `${{catalogDisplayName(catCode)}} ${{String(catCode || '').toUpperCase()}} ${{p?.code || ''}}`;
        return matchesQuery(text);
    }});

    const otherFamilies = reviewCatalogFamilies.filter(item => {{
        if (familySet.has(item.code)) return false;
        return matchesQuery(`${{item.label}} ${{item.code}}`);
    }});

    populateRightCatalogSearchSuggestions([
        ...reviewCatalogFamilies.filter(item => familySet.has(item.code)),
        ...otherFamilies
    ]);

    const optionSpecific = stampSpecific.map(p => {{
        const catCode = normalizeCatalogCodeForUi(p?.catalog_type);
        const label = `${{catalogDisplayName(catCode)}} (${{String(catCode || '').toUpperCase()}}): ${{p.code || ''}}`;
        const selected = String(selectedCode || '') === String(p.code || '') ? ' selected' : '';
        return `<option value='${{html(p.code || '')}}' data-cat='${{html(catCode)}}'${{selected}}>${{html(label)}}</option>`;
    }}).join('');

    const optionOther = otherFamilies.map(item => {{
        const value = `__family__${{item.code}}`;
        const selected = !selectedCode && String(selectedType || '') === String(item.code) ? ' selected' : '';
        return `<option value='${{html(value)}}' data-cat='${{html(item.code)}}'${{selected}}>${{html(catalogFamilyOptionText(item))}}</option>`;
    }}).join('');

    return [
        "<option value=''>-- none --</option>",
        optionSpecific ? `<optgroup label='For this stamp'>${{optionSpecific}}</optgroup>` : '',
        optionOther ? `<optgroup label='Other catalogs'>${{optionOther}}</optgroup>` : ''
    ].join('');
}}

function ordinalValueForCatalog(item, catalogCode) {{
    const code = normalizeCatalogCodeForUi(catalogCode);
    if (code === 'sg') return item?.ord_sg || '';
    if (code === 'sn') return item?.ord_sn || '';
    if (code === 'mi') return item?.ord_mi || '';
    if (code === 'yt') return item?.ord_yt || '';
    return '';
}}

function render(options) {{
    const searchFocusState = captureSearchFocusState();
    const preserveRightScroll = options && options.preserveRightScroll === true;
    const rightPanelBefore = document.getElementById('rightPanel');
    const rightPanelBodyBefore = rightPanelBefore ? rightPanelBefore.querySelector('.panelBody') : null;
    const savedRightScrollTop = (preserveRightScroll && rightPanelBodyBefore) ? rightPanelBodyBefore.scrollTop : 0;

    function restoreRightPanelScrollIfNeeded() {{
        if (!preserveRightScroll) return;
        const rightPanelAfter = document.getElementById('rightPanel');
        const rightPanelBodyAfter = rightPanelAfter ? rightPanelAfter.querySelector('.panelBody') : null;
        if (rightPanelBodyAfter) rightPanelBodyAfter.scrollTop = savedRightScrollTop;
    }}

    updateGuide();
  if (!left.length) {{
        document.getElementById('leftPos').innerText = 'Left 0/0';
        document.getElementById('rightPos').innerText = 'Candidate 0/0';
    document.getElementById('leftActionBanner').innerHTML = `
        <div class='leftActionTop'>
            <strong>Manual Base Stamp Tools</strong>
            <span class='badge'>No left rows loaded</span>
        </div>
        <p class='leftActionText'>Add a base/reference item here when you need a watermark image or a missing left-side record before matching Colnect data.${{showImagelessLeftRowsDefault ? ' Image-less rows are currently visible.' : ''}}</p>
        <div class='leftActionBtns'>
            <button class='warn' onclick='openLeftEditor("add")'>Add Base / Watermark Item</button>
        </div>`;
    document.getElementById('leftPanel').innerHTML = '<div class="imgph">No StampWorld rows with real images found</div>';
    document.getElementById('rightPanel').innerHTML = '<div class="imgph">No candidates</div>';
    restoreRightPanelScrollIfNeeded();
    restoreSearchFocusState(searchFocusState);
    return;
  }}

  const seriesEl = document.getElementById('f_series');
  if (seriesEl && !seriesEl.dataset.userTouched) {{
    seriesEl.checked = checkboxDefaults.f_series;
  }}

  const l = left[leftIdx];
    const leftCatalogCode = effectiveCatalogCodeForItem(l);
    const leftCatalogLabel = catalogDisplayName(leftCatalogCode);
    const leftCatalogTypeLabel = leftCatalogCode
        ? `${{leftCatalogLabel}} (${{String(leftCatalogCode).toUpperCase()}})`
        : leftCatalogLabel;
  const leftImg = b(l.image_abs) ? `/image?path=${{encodeURIComponent(l.image_abs)}}` : l.stampworld_image_url;
  document.getElementById('leftPos').innerText = `Left ${{leftIdx + 1}}/${{left.length}} (rowid=${{l.rowid}})`;
  document.getElementById('leftActionBanner').innerHTML = `
        <div class='leftActionTop'>
            <strong>Manual Base Stamp Tools</strong>
            <div class='leftActionNote'>
                <span class='badge'>Current rowid=${{html(l.rowid)}}</span>
                <span class='badge'>${{b(l.colnect_id) ? `Linked Colnect: ${{html(l.colnect_id)}}` : 'No Colnect link'}}</span>
            </div>
        </div>
        <p class='leftActionText'>Use these when the StampWorld base row is wrong, when a watermark/reference image should live on the left, or when a Colnect import landed on the wrong base row.${{showImagelessLeftRowsDefault ? ' Image-less rows are visible in this session.' : ' New left items need an image to remain visible here.'}}</p>
        <div class='leftActionBtns'>
            <button onclick='openLeftEditor("edit")'>Edit Current</button>
            <button class='warn' onclick='openLeftEditor("add")'>Add Base / Watermark Item</button>
            <button onclick='clearCurrentLeftColnectLink()' ${{b(l.colnect_id) ? '' : 'disabled'}}>Clear Colnect Link</button>
            <button class='danger' onclick='deleteCurrentLeft()'>Delete Current</button>
        </div>
        <div class='leftSearchRow'>
            <label for='leftSearchInput'>Search Left</label>
            <input id='leftSearchInput' type='text' placeholder='Catalog #, series, name...' oninput='onLeftSearchInput()' value='${{html(leftSearchQuery)}}'/>
            <button onclick='stepLeftSearch(-1)'>Back</button>
            <button onclick='stepLeftSearch(1)'>Forward</button>
            <span id='leftSearchStatus' class='badge'>Matches: 0/0</span>
        </div>
        <div class='leftNavRow'>
            <button class='navBtn' onclick='firstLeft()'>First</button>
            <button class='navBtn' onclick='jumpLeft(-50)'>-50</button>
            <button class='navBtn' onclick='jumpLeft(-10)'>-10</button>
            <button class='navBtn' onclick='prevLeft()'>Prev Left</button>
            <button class='navBtn' onclick='nextLeft()'>Next Left</button>
            <button class='navBtn' onclick='jumpLeft(10)'>+10</button>
            <button class='navBtn' onclick='jumpLeft(50)'>+50</button>
            <button class='navBtn' onclick='lastLeft()'>Last</button>
        </div>`;
  updateLeftSearchMatches(true);
  document.getElementById('rightActionBanner').innerHTML = `
        <div class='rightActionTop'>
            <strong>Candidate Search And Navigation</strong>
            <span class='badge'>Filtered by current left row</span>
        </div>
        <div class='rightBottomBlock'>
        <div class='rightSearchRow'>
            <label for='rightSearchInput'>Search Right</label>
            <input id='rightSearchInput' type='text' placeholder='ID, catalog #, name, series...' oninput='onRightSearchInput()' value='${{html(rightSearchQuery)}}'/>
            <button onclick='jumpRight(-1)'>Back</button>
            <button onclick='jumpRight(1)'>Forward</button>
            <span id='rightSearchStatus' class='badge'>Matches: 0/0</span>
        </div>
        <div class='rightNavRow'>
            <button class='navBtn' onclick='firstRight()'>First</button>
            <button class='navBtn' onclick='jumpRight(-50)'>-50</button>
            <button class='navBtn' onclick='jumpRight(-10)'>-10</button>
            <button class='navBtn' onclick='prevRight()'>Prev Candidate</button>
            <button class='navBtn' onclick='nextRight()'>Next Candidate</button>
            <button class='navBtn' onclick='jumpRight(10)'>+10</button>
            <button class='navBtn' onclick='jumpRight(50)'>+50</button>
            <button class='navBtn' onclick='lastRight()'>Last</button>
        </div>
        </div>`;
  updateRightSearchStatus();
  document.getElementById('leftPanel').innerHTML = `
        <div class='panelCard'>
            <div class='panelHead'>
                <strong>Base Stamp</strong> ${{html(leftCatalogTypeLabel)}} #=${{html(fmt(l.sg_number))}} | Year=${{html(fmt(l.year))}} | Series=${{html(fmt(l.series))}}
            </div>
            <div class='panelBody'>
                ${{imageBox(leftImg, 'StampWorld image')}}
                <table class='kv'>
                    ${{row('Catalog Type', leftCatalogTypeLabel)}}
                    ${{row('Catalog #', l.sg_number)}}
                    ${{row('Year', l.year)}}
                    ${{row('Series', l.series)}}
                    ${{row('Image Path', l.image_path)}}
                    ${{row('StampWorld URL', l.stampworld_image_url)}}
                    ${{row('Description', l.description)}}
                    ${{row('Watermark', l.watermark)}}
                    ${{row('Design', l.design)}}
                    ${{row('Engraving', l.engraving)}}
                    ${{row('Perforation', l.perforation)}}
                    ${{row('Context', l.context)}}
                    ${{row('colnect_page_url', l.colnect_page_url)}}
                    ${{row('colnect_image_url', l.colnect_image_url)}}
                    ${{row('colnect_id', l.colnect_id)}}
                    ${{row('colnect_name', l.colnect_name)}}
                    ${{row('colnect_catalog_codes', l.colnect_catalog_codes)}}
                    ${{row('colnect_country', l.colnect_country)}}
                    ${{row('colnect_width', l.colnect_width)}}
                    ${{row('colnect_height', l.colnect_height)}}
                    ${{row('colnect_format', l.colnect_format)}}
                    ${{row('colnect_emission', l.colnect_emission)}}
                    ${{row('colnect_printing', l.colnect_printing)}}
                    ${{row('colnect_paper', l.colnect_paper)}}
                    ${{row('colnect_gum', l.colnect_gum)}}
                    ${{row('colnect_ord_mi', l.colnect_ord_mi)}}
                    ${{row('colnect_ord_sn', l.colnect_ord_sn)}}
                    ${{row('colnect_ord_yt', l.colnect_ord_yt)}}
                    ${{row('colnect_face_value', l.colnect_face_value)}}
                    ${{row('colnect_currency', l.colnect_currency)}}
                    ${{row('colnect_ord_sg', l.colnect_ord_sg)}}
                    ${{row('colnect_expiry_date', l.colnect_expiry_date)}}
                    ${{row('colnect_print_run', l.colnect_print_run)}}
                    ${{row('colnect_colors', l.colnect_colors)}}
                    ${{row('colnect_parent_item', l.colnect_parent_item)}}
                    ${{row('colnect_variant', l.colnect_variant)}}
                    ${{row('colnect_overprint_variant', l.colnect_overprint_variant)}}
                    ${{row('colnect_score', l.colnect_score)}}
                    ${{row('colnect_accuracy', l.colnect_accuracy)}}
                    ${{row('colnect_list', l.colnect_list)}}
                    ${{row('colnect_quantity', l.colnect_quantity)}}
                    ${{row('colnect_condition', l.colnect_condition)}}
                    ${{row('colnect_public_note', l.colnect_public_note)}}
                    ${{row('colnect_private_note', l.colnect_private_note)}}
                    ${{allDbRows(l.all_db_fields)}}
                    ${{row('Assigned Colnect ID', assignedByLeft[l.rowid] || '(none)')}}
                </table>
            </div>
        </div>
  `;

  const avail = availableCandidatesForCurrentLeft();
  const r = currentRightCandidate();
  if (!r) {{
    document.getElementById('rightPos').innerText = 'No available candidate';
    document.getElementById('rightPanel').innerHTML = '<div class="imgph">All candidates already assigned</div>';
        updateRightSearchStatus();
    restoreRightPanelScrollIfNeeded();
    restoreSearchFocusState(searchFocusState);
    return;
  }}

  const key = String(l.rowid);
  document.getElementById('rightPos').innerText = `Candidate ${{(rightIdxByLeft[key] || 0) + 1}}/${{avail.length}}`;
        updateRightSearchStatus();
    ensureRightImageUrl(r);
  const rightImg = b(r.image_abs) ? `/image?path=${{encodeURIComponent(r.image_abs)}}` : r.image_url;
        const effectiveCatalogCode = getEffectiveDefaultCatalogCode();
        const effectiveCatalogCodeUpper = String(effectiveCatalogCode || '').toUpperCase();
        const effectiveCatalogOrdLabel = `Ord Current Catalog (${{effectiveCatalogCodeUpper || 'N/A'}})`;
        const effectiveCatalogOrdValue = ordinalValueForCatalog(r, effectiveCatalogCode);
    const rightCatalogType = inferCatalogTypeForNumber(
        r.catalog_codes_text,
        r.sg_number,
        effectiveCatalogCode
    );
    const rightCatalogTypeLabel = rightCatalogType
        ? `${{catalogDisplayName(rightCatalogType)}} (${{String(rightCatalogType).toUpperCase()}})`
        : 'Catalog';
    const rightCatalogQuery = String(document.getElementById('catalogCodeSearchInput')?.value || '');
    const selectedCode = selectedCatalogCodeForCurrent(r);
    const selectedType = selectedCatalogTypeForCurrent(r);
    const rightCatalogOptions = buildRightCatalogSelectOptions(r, selectedCode, selectedType);
  document.getElementById('rightPanel').innerHTML = `
        <div class='panelCard'>
            <div class='panelHead'>
               
                <div style='margin-top:8px;'>
                    <label>Catalog code to assign (for catalog number replace):</label>
                    <input id='catalogCodeSearchInput' type='text' list='catalogCodeSearchList' value='${{html(rightCatalogQuery)}}' placeholder='Type to filter catalogs...' oninput='render({{ preserveRightScroll: true }})' onchange='applyRightCatalogSearchChoice()' onblur='tryApplyRightCatalogSearchChoice()' style='display:block; margin:6px 0; width:100%; max-width:460px;' />
                    <datalist id='catalogCodeSearchList'></datalist>
                    <select id='catalogCodeSelect' onchange='rememberCatalogCode()'>
                        ${{rightCatalogOptions}}
                    </select>
                </div>
            </div>
            <div class='panelBody'>
                ${{imageBox(rightImg, 'Colnect candidate image')}}
                <table class='kv'>
                    ${{row('Colnect ID', r.colnect_id)}}
                    ${{row(rightCatalogTypeLabel + ' #', r.sg_number)}}
            ${{row('Catalog Codes Text', r.catalog_codes_text)}}
                    ${{row('Year', r.year)}}
            ${{row('Country', r.country)}}
                    ${{row('Series', r.series)}}
                    ${{row('Page URL', r.page_url)}}
                    ${{row('Image URL', r.image_url)}}
            ${{row('Name', r.name)}}
                    ${{row('Description', r.description)}}
                    ${{row('Watermark', r.watermark)}}
            ${{row('Format', r.format)}}
            ${{row('Emission', r.emission)}}
            ${{row('Printing', r.printing)}}
            ${{row('Paper', r.paper)}}
            ${{row('Gum', r.gum)}}
            ${{row('Width', r.width)}}
            ${{row('Height', r.height)}}
            ${{row('Face Value', r.face_value)}}
            ${{row('Currency', r.currency)}}
            ${{row('Colors', r.colors)}}
      ${{row('Design', r.design)}}
      ${{row('Engraving', r.engraving)}}
      ${{row('Perforation', r.perforation)}}
            ${{row(effectiveCatalogOrdLabel, effectiveCatalogOrdValue)}}
            ${{row('ORD MI', r.ord_mi)}}
            ${{row('ORD SN', r.ord_sn)}}
            ${{row('ORD YT', r.ord_yt)}}
            ${{row('Print Run', r.print_run)}}
            ${{row('Expiry Date', r.expiry_date)}}
            ${{row('Parent Item', r.parent_item)}}
            ${{row('Variant', r.variant)}}
            ${{row('Overprint Variant', r.overprint_variant)}}
            ${{row('Score', r.score)}}
            ${{row('Accuracy', r.accuracy)}}
            ${{row('List', r.list)}}
            ${{row('Quantity', r.quantity)}}
            ${{row('Condition', r.condition)}}
            ${{row('Public Note', r.public_note)}}
            ${{row('Private Note', r.private_note)}}
                    ${{row('Context', r.context)}}
                </table>
            </div>
        </div>
  `;

    const sel = document.getElementById('catalogCodeSelect');
    if (sel) {{
        if (selectedCode) {{
            sel.value = selectedCode;
        }} else if (selectedType) {{
            sel.value = `__family__${{selectedType}}`;
        }}
    }}

    restoreRightPanelScrollIfNeeded();
    restoreSearchFocusState(searchFocusState);
}}

function rememberCatalogCode() {{
    const l = left[leftIdx];
    if (!l) return;
    const sel = document.getElementById('catalogCodeSelect');
    if (!sel) return;
    const rawValue = String(sel.value || '');
    if (rawValue.startsWith('__family__')) {{
        selectedCatalogCodeByLeft[l.rowid] = '';
        selectedCatalogTypeByLeft[l.rowid] = rawValue.slice('__family__'.length);
        return;
    }}
    selectedCatalogCodeByLeft[l.rowid] = rawValue;
    const selectedOpt = sel.options[sel.selectedIndex];
    selectedCatalogTypeByLeft[l.rowid] = selectedOpt?.dataset?.cat || '';
}}

function prevLeft() {{
  if (!left.length) return;
  leftIdx = (leftIdx - 1 + left.length) % left.length;
  render({{ preserveRightScroll: true }});
}}
function nextLeft() {{
  if (!left.length) return;
  leftIdx = (leftIdx + 1) % left.length;
  render({{ preserveRightScroll: true }});
}}
function prevRight() {{
  const l = left[leftIdx];
  const avail = availableCandidatesForCurrentLeft();
  if (!avail.length) return;
  const key = String(l.rowid);
  const cur = rightIdxByLeft[key] || 0;
  rightIdxByLeft[key] = (cur - 1 + avail.length) % avail.length;
  render();
}}
function nextRight() {{
  const l = left[leftIdx];
  const avail = availableCandidatesForCurrentLeft();
  if (!avail.length) return;
  const key = String(l.rowid);
  const cur = rightIdxByLeft[key] || 0;
  rightIdxByLeft[key] = (cur + 1) % avail.length;
  render();
}}
function assignCurrent() {{
  const l = left[leftIdx];
  const r = currentRightCandidate();
    if (!l || !r) return;
    rememberCatalogCode();
  assignedByLeft[l.rowid] = r.colnect_id;
    setMatchStatus(`Assigned rowid=${{l.rowid}} -> colnect_id=${{r.colnect_id}}`);
  render();
}}
function clearAssignment() {{
  const l = left[leftIdx];
  if (!l) return;
  delete assignedByLeft[l.rowid];
    delete selectedCatalogCodeByLeft[l.rowid];
    delete selectedCatalogTypeByLeft[l.rowid];
    setMatchStatus(`Cleared assignment for rowid=${{l.rowid}}`);
  render();
}}

function applySuccessfulMatchToLocalLeft(match, candidate, fields) {{
    if (!match || !candidate || !fields) return;
    const leftRow = left.find(item => String(item.rowid) === String(match.left_rowid));
    if (!leftRow) return;

    if (fields.replace_sg_number && match.selected_catalog_code && String(match.selected_catalog_code).trim()) {{
        leftRow.sg_number = String(match.selected_catalog_code).trim();
        leftRow.catalog_code = String(match.selected_catalog_type || '').trim();
    }}
    if (fields.update_series && candidate.series) leftRow.series = candidate.series;
    if (fields.update_description && candidate.description) leftRow.description = candidate.description;
    if (fields.update_watermark && candidate.watermark) leftRow.watermark = candidate.watermark;
    if (fields.update_design && candidate.design) leftRow.design = candidate.design;
    if (fields.update_engraving && candidate.engraving) leftRow.engraving = candidate.engraving;
    if (fields.update_perforation && candidate.perforation) leftRow.perforation = candidate.perforation;
    if (fields.update_context && candidate.context) leftRow.context = candidate.context;

    if (fields.update_colnect_links) {{
        if (candidate.page_url) leftRow.colnect_page_url = candidate.page_url;
    }}

    if (fields.update_colnect_image_url) {{
        if (candidate.image_url) leftRow.colnect_image_url = candidate.image_url;
        const remote = String(leftRow.remote_image_url || '').trim();
        if (candidate.image_url && (!remote || remote.includes('colnect.com/stamps/stamp/'))) {{
            leftRow.remote_image_url = candidate.image_url;
        }}
    }}

    if (fields.update_colnect_metadata) {{
        leftRow.colnect_id = candidate.colnect_id || leftRow.colnect_id;
        leftRow.colnect_name = candidate.name || leftRow.colnect_name;
        leftRow.colnect_catalog_codes = candidate.catalog_codes_text || leftRow.colnect_catalog_codes;
        leftRow.colnect_country = candidate.country || leftRow.colnect_country;
        leftRow.colnect_width = match.width || candidate.width || leftRow.colnect_width;
        leftRow.colnect_height = match.height || candidate.height || leftRow.colnect_height;
        leftRow.colnect_format = candidate.format || leftRow.colnect_format;
        leftRow.colnect_emission = candidate.emission || leftRow.colnect_emission;
        leftRow.colnect_printing = candidate.printing || leftRow.colnect_printing;
        leftRow.colnect_paper = candidate.paper || leftRow.colnect_paper;
        leftRow.colnect_gum = candidate.gum || leftRow.colnect_gum;
        leftRow.colnect_ord_mi = candidate.ord_mi || leftRow.colnect_ord_mi;
        leftRow.colnect_ord_sn = candidate.ord_sn || leftRow.colnect_ord_sn;
        leftRow.colnect_ord_yt = candidate.ord_yt || leftRow.colnect_ord_yt;
        leftRow.colnect_ord_sg = candidate.ord_sg || leftRow.colnect_ord_sg;
        leftRow.colnect_expiry_date = candidate.expiry_date || leftRow.colnect_expiry_date;
        leftRow.colnect_print_run = candidate.print_run || leftRow.colnect_print_run;
        leftRow.colnect_face_value = candidate.face_value || leftRow.colnect_face_value;
        leftRow.colnect_currency = candidate.currency || leftRow.colnect_currency;
        leftRow.colnect_colors = candidate.colors || leftRow.colnect_colors;
        leftRow.colnect_parent_item = candidate.parent_item || leftRow.colnect_parent_item;
        leftRow.colnect_variant = candidate.variant || leftRow.colnect_variant;
        leftRow.colnect_overprint_variant = candidate.overprint_variant || leftRow.colnect_overprint_variant;
        leftRow.colnect_score = candidate.score || leftRow.colnect_score;
        leftRow.colnect_accuracy = candidate.accuracy || leftRow.colnect_accuracy;
        leftRow.colnect_list = candidate.list || leftRow.colnect_list;
        leftRow.colnect_quantity = candidate.quantity || leftRow.colnect_quantity;
        leftRow.colnect_condition = candidate.condition || leftRow.colnect_condition;
        leftRow.colnect_public_note = candidate.public_note || leftRow.colnect_public_note;
        leftRow.colnect_private_note = candidate.private_note || leftRow.colnect_private_note;
    }}
}}

async function applyMatches() {{
    const candidateMap = {{}};
    rightAll.forEach(c => {{ candidateMap[c.colnect_id] = c; }});
    const matches = Object.entries(assignedByLeft).map(([leftRowid, colnectId]) => {{
        const candidate = candidateMap[colnectId];
        return {{
    left_rowid: Number(leftRowid),
        colnect_id: colnectId,
                selected_catalog_code: selectedCatalogCodeByLeft[leftRowid] || null,
            selected_catalog_type: selectedCatalogTypeByLeft[leftRowid] || null,
                width: candidate?.width || null,
                height: candidate?.height || null
        }};
    }});

    if (!matches.length) {{
        setMatchStatus('No assigned matches selected. Use Assign Candidate first.');
        return;
    }}

    const fields = {{
        update_series: !!document.getElementById('f_series')?.checked,
        update_description: !!document.getElementById('f_description')?.checked,
        update_watermark: !!document.getElementById('f_watermark')?.checked,
        update_design: !!document.getElementById('f_design')?.checked,
        update_engraving: !!document.getElementById('f_engraving')?.checked,
        update_perforation: !!document.getElementById('f_perforation')?.checked,
        update_context: !!document.getElementById('f_context')?.checked,
        update_colnect_links: !!document.getElementById('f_links')?.checked,
        update_colnect_image_url: !!document.getElementById('f_image_url')?.checked,
        update_colnect_metadata: !!document.getElementById('f_colnect_meta')?.checked,
        replace_sg_number: !!document.getElementById('f_replace_sg')?.checked
    }};

  const res = await fetch('/apply-matches', {{
    method: 'POST',
    headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ matches, fields }})
  }});
  const j = await res.json();
        const skippedDetails = Array.isArray(j.skipped_details) ? j.skipped_details : [];
    for (const colnectId of (j.processed_colnect_ids || [])) {{
        const appliedMatch = matches.find(m => m.colnect_id === colnectId);
        const appliedCandidate = candidateMap[colnectId];
        applySuccessfulMatchToLocalLeft(appliedMatch, appliedCandidate, fields);
        removeCandidateFromUi(colnectId);
    }}
    let statusMsg = `Applied: updated=${{j.updated_rows}}, skipped=${{j.skipped_rows}}, errors=${{(j.errors||[]).length}}`;
    if (skippedDetails.length) {{
        statusMsg += ` | first skip: ${{skippedDetails[0]}}`;
    }}
    setMatchStatus(statusMsg);
    const notices = [];
    if (skippedDetails.length) notices.push('Skipped details:\n' + skippedDetails.join('\n'));
    if ((j.errors || []).length) notices.push('Errors:\n' + j.errors.join('\n'));
    if (notices.length) {{
        alert(notices.join('\n\n'));
  }}
    render();
}}

function getSelectedAddImageMode() {{
    const radios = document.querySelectorAll("input[name='addImageMode']");
    for (const r of radios) {{
        if (r.checked) return r.value;
    }}
    return 'current_left';
}}

function toggleAddImageOptions() {{
    const assoc = !!document.getElementById('addAssociateImage')?.checked;
    const options = document.getElementById('addImageOptions');
    if (options) options.style.display = assoc ? '' : 'none';
    const uploadRow = document.getElementById('uploadRow');
    const mode = getSelectedAddImageMode();
    if (uploadRow) uploadRow.style.display = assoc && mode === 'upload' ? '' : 'none';
}}

function openAddDialog() {{
    const r = currentRightCandidate();
    if (!r) {{
        setMatchStatus('No current candidate to add');
        return;
    }}
    const preferredCode = selectedCatalogCodeForCurrent(r) || r.sg_number || '';
    const preferredType = selectedCatalogTypeForCurrent(r) || getEffectiveDefaultCatalogCode() || '';
    const labelText = preferredType
        ? `${{catalogDisplayName(preferredType)}} (${{String(preferredType).toUpperCase()}})`
        : 'Catalog';
    const labelEl = document.getElementById('addCatalogLabel');
    if (labelEl) labelEl.innerText = labelText;
    document.getElementById('addSg').value = normalizeCatalogNumber(preferredCode);
    document.getElementById('addYear').value = Number.isFinite(r.year) ? r.year : '';
    document.getElementById('addSeries').value = r.series || '';
    document.getElementById('addAssociateImage').checked = true;
    const radioCurrent = document.querySelector("input[name='addImageMode'][value='current_left']");
    if (radioCurrent) radioCurrent.checked = true;
    const upload = document.getElementById('addUploadFile');
    if (upload) upload.value = '';
    toggleAddImageOptions();
    document.getElementById('addModalBackdrop').style.display = 'flex';
}}

function closeAddDialog() {{
    const modal = document.getElementById('addModalBackdrop');
    if (modal) modal.style.display = 'none';
}}

function fileToDataUrl(file) {{
    return new Promise((resolve, reject) => {{
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result || ''));
        reader.onerror = () => reject(new Error('failed to read upload file'));
        reader.readAsDataURL(file);
    }});
}}

function getSelectedLeftImageMode() {{
    const radios = document.querySelectorAll("input[name='leftImageMode']");
    for (const radio of radios) {{
        if (radio.checked) return radio.value;
    }}
    return leftEditorMode === 'edit' ? 'keep' : 'upload';
}}

function toggleLeftEditorImageOptions() {{
    const mode = getSelectedLeftImageMode();
    const uploadRow = document.getElementById('leftUploadRow');
    const keepWrap = document.getElementById('leftModeKeepWrap');
    const currentWrap = document.getElementById('leftModeCurrentWrap');
    if (keepWrap) keepWrap.style.display = leftEditorMode === 'edit' ? '' : 'none';
    if (currentWrap) currentWrap.style.display = leftEditorMode === 'add' ? '' : 'none';
    if (uploadRow) uploadRow.style.display = mode === 'upload' ? '' : 'none';
}}

function toggleLeftEditorCatalogCodeCustom() {{
    const selectEl = document.getElementById('leftEditorCatalogCodeSelect');
    const customWrap = document.getElementById('leftEditorCatalogCodeCustomWrap');
    const isCustom = String(selectEl?.value || '') === 'custom';
    if (customWrap) customWrap.style.display = isCustom ? '' : 'none';
}}

function parseCatalogTypesFromRaw(raw) {{
    const out = [];
    const seen = new Set();
    for (const part of String(raw || '').split(/[|,]/)) {{
        const trimmed = String(part || '').trim();
        if (!trimmed) continue;
        const idx = trimmed.indexOf(':');
        if (idx < 1) continue;
        const code = normalizeCatalogCodeForUi(trimmed.slice(0, idx));
        if (!code || seen.has(code)) continue;
        seen.add(code);
        out.push(code);
    }}
    return out;
}}

function leftEditorSuggestedCatalogCodes(base) {{
    const suggested = [];
    const seen = new Set();
    const push = (code) => {{
        const normalized = normalizeCatalogCodeForUi(code);
        if (!normalized || seen.has(normalized)) return;
        seen.add(normalized);
        suggested.push(normalized);
    }};
    push(base?.catalog_code);
    for (const c of parseCatalogTypesFromRaw(base?.colnect_catalog_codes)) push(c);
    push(getEffectiveDefaultCatalogCode());
    return suggested;
}}

function catalogFamilyOptionText(item) {{
    const countText = Number.isFinite(Number(item?.count || 0)) && Number(item.count) > 0
        ? ` (${{Number(item.count).toLocaleString()}})`
        : '';
    return `${{item.label}} (${{String(item.code || '').toUpperCase()}})${{countText}}`;
}}

function catalogFamilySearchText(item) {{
    return `${{item.label}} [${{String(item.code || '').toUpperCase()}}]`;
}}

function parseCatalogCodeFromSearchValue(v) {{
    const raw = String(v || '').trim();
    if (!raw) return '';
    
    // Try format: "Label [CODE]" (from datalist selection)
    let m = raw.match(/\[([^\]]+)\]\s*$/);
    if (m) return String(m[1] || '').trim().toLowerCase();
    
    // Try format: "Label (CODE)" (user might type this)
    m = raw.match(/\(([A-Za-z0-9]+)\)(?:\s|$)/);
    if (m) return String(m[1] || '').trim().toLowerCase();
    
    // Try to find CODE anywhere in the string (2-4 letter catalog codes)
    // Look for capital letter codes like WNS, WAD, SG, MI, SN, YT, etc.
    m = raw.match(/(^|\s|-)([A-Z0-9]{{2,4}})(\s|$|\)|#)/);
    if (m && m[2]) return String(m[2]).trim().toLowerCase();
    
    return '';
}}

function populateLeftEditorCatalogSearchSuggestions(items) {{
    const listEl = document.getElementById('leftEditorCatalogCodeSearchList');
    if (!listEl) return;
    listEl.innerHTML = (items || [])
        .slice(0, 200)
        .map(item => `<option value='${{html(catalogFamilySearchText(item))}}'></option>`)
        .join('');
}}

function applyLeftEditorCatalogSearchChoice() {{
    const searchEl = document.getElementById('leftEditorCatalogCodeSearch');
    if (!searchEl) return;
    const code = parseCatalogCodeFromSearchValue(searchEl.value);
    if (!code) return;
    rebuildLeftEditorCatalogTypeOptions(code);
}}

function tryApplyLeftEditorCatalogSearchChoice() {{
    // Called on blur: silently try to extract and apply catalog code if possible
    const searchEl = document.getElementById('leftEditorCatalogCodeSearch');
    if (!searchEl || !searchEl.value.trim()) return;
    const code = parseCatalogCodeFromSearchValue(searchEl.value);
    if (!code) return; // Silently ignore if parsing fails
    rebuildLeftEditorCatalogTypeOptions(code);
}}

function rebuildLeftEditorCatalogTypeOptions(selectedCode) {{
    const selectEl = document.getElementById('leftEditorCatalogCodeSelect');
    const customEl = document.getElementById('leftEditorCatalogCodeCustom');
    const searchEl = document.getElementById('leftEditorCatalogCodeSearch');
    if (!selectEl || !customEl) return;

    const query = String(searchEl?.value || '').trim().toLowerCase();
    const suggested = new Set(leftEditorSuggestedCatalogCodes(window.__leftEditorCatalogBase || null));
    const visible = reviewCatalogFamilies.filter(item => {{
        if (!query) return true;
        return String(item.code || '').toLowerCase().includes(query)
            || String(item.label || '').toLowerCase().includes(query);
    }});
    const suggestedItems = visible.filter(item => suggested.has(item.code));
    const otherItems = visible.filter(item => !suggested.has(item.code));

    populateLeftEditorCatalogSearchSuggestions(visible);

    const mkOption = (item) => `<option value='${{html(item.code)}}'>${{html(catalogFamilyOptionText(item))}}</option>`;
    let optionsHtml = '';
    if (suggestedItems.length) {{
        optionsHtml += `<optgroup label='For this stamp'>${{suggestedItems.map(mkOption).join('')}}</optgroup>`;
    }}
    if (otherItems.length) {{
        optionsHtml += `<optgroup label='Other catalogs'>${{otherItems.map(mkOption).join('')}}</optgroup>`;
    }}
    optionsHtml += "<option value='custom'>Other / Custom...</option>";
    selectEl.innerHTML = optionsHtml;

    const normalized = normalizeCatalogCodeForUi(selectedCode);
    if (normalized && reviewCatalogKnownCodes.has(normalized)) {{
        selectEl.value = normalized;
        customEl.value = '';
    }} else if (String(selectedCode || '').trim()) {{
        selectEl.value = 'custom';
        customEl.value = String(selectedCode || '').trim().toLowerCase();
    }} else {{
        const fallback = getEffectiveDefaultCatalogCode();
        if (reviewCatalogKnownCodes.has(fallback)) {{
            selectEl.value = fallback;
        }} else {{
            selectEl.value = 'custom';
            customEl.value = fallback;
        }}
    }}
    toggleLeftEditorCatalogCodeCustom();
}}

function filterLeftEditorCatalogTypeOptions() {{
    const current = getLeftEditorCatalogCodeValue();
    rebuildLeftEditorCatalogTypeOptions(current);
}}

function setLeftEditorCatalogCodeValue(code) {{
    rebuildLeftEditorCatalogTypeOptions(code);
}}

function getLeftEditorCatalogCodeValue() {{
    const selectEl = document.getElementById('leftEditorCatalogCodeSelect');
    const customEl = document.getElementById('leftEditorCatalogCodeCustom');
    if (String(selectEl?.value || '') === 'custom') {{
        return String(customEl?.value || '').trim().toLowerCase();
    }}
    return String(selectEl?.value || '').trim().toLowerCase();
}}

function openLeftEditor(mode) {{
    leftEditorMode = mode === 'add' ? 'add' : 'edit';
    const currentLeft = left[leftIdx] || null;
    leftEditorSourceRowid = currentLeft ? Number(currentLeft.rowid) : null;

    const title = document.getElementById('leftEditorTitle');
    const hint = document.getElementById('leftEditorHint');
    if (title) title.innerText = leftEditorMode === 'add' ? 'Add Base / Watermark Item' : 'Edit Base Stamp';
    if (hint) hint.innerText = leftEditorMode === 'add'
        ? 'Create a left-side reference row for a missing base stamp or watermark image. Use a short reference code such as WM-1 when this is not a true stamp number.'
        : 'Correct the current StampWorld base row without leaving the review page.';

    const base = leftEditorMode === 'edit'
        ? currentLeft
        : {{
            sg_number: '',
            catalog_code: getEffectiveDefaultCatalogCode(),
            year: currentLeft?.year || 0,
            series: currentLeft?.series || '',
            description: '',
            watermark: '',
            design: '',
            engraving: '',
            perforation: '',
            context: '',
            width: '',
            height: '',
            colnect_page_url: '',
            colnect_image_url: '',
            colnect_id: '',
            colnect_name: '',
            colnect_catalog_codes: '',
            colnect_country: '',
            colnect_width: '',
            colnect_height: '',
            colnect_format: '',
            colnect_emission: '',
            colnect_printing: '',
            colnect_paper: '',
            colnect_gum: '',
            colnect_ord_mi: '',
            colnect_ord_sn: '',
            colnect_ord_yt: '',
            colnect_face_value: '',
            colnect_currency: '',
            colnect_ord_sg: '',
            colnect_expiry_date: '',
            colnect_print_run: '',
            colnect_colors: '',
            colnect_parent_item: '',
            colnect_variant: '',
            colnect_overprint_variant: '',
            colnect_score: '',
            colnect_accuracy: '',
            colnect_list: '',
            colnect_quantity: '',
            colnect_condition: '',
            colnect_public_note: '',
            colnect_private_note: ''
        }};

    window.__leftEditorCatalogBase = base;
    const catalogSearchInput = document.getElementById('leftEditorCatalogCodeSearch');
    if (catalogSearchInput) catalogSearchInput.value = '';

    document.getElementById('leftEditorSg').value = base?.sg_number || '';
    setLeftEditorCatalogCodeValue(base?.catalog_code);
    document.getElementById('leftEditorYear').value = Number(base?.year || 0) || '';
    document.getElementById('leftEditorSeries').value = base?.series || '';
    document.getElementById('leftEditorDescription').value = base?.description || '';
    document.getElementById('leftEditorWatermark').value = base?.watermark || '';
    document.getElementById('leftEditorDesign').value = base?.design || '';
    document.getElementById('leftEditorEngraving').value = base?.engraving || '';
    document.getElementById('leftEditorPerforation').value = base?.perforation || '';
    document.getElementById('leftEditorContext').value = base?.context || '';
    document.getElementById('leftEditorWidth').value = base?.width || base?.colnect_width || '';
    document.getElementById('leftEditorHeight').value = base?.height || base?.colnect_height || '';
    document.getElementById('leftEditorColnectPageUrl').value = base?.colnect_page_url || '';
    document.getElementById('leftEditorColnectImageUrl').value = base?.colnect_image_url || '';
    document.getElementById('leftEditorColnectId').value = base?.colnect_id || '';
    document.getElementById('leftEditorColnectName').value = base?.colnect_name || '';
    document.getElementById('leftEditorColnectCatalogCodes').value = base?.colnect_catalog_codes || '';
    document.getElementById('leftEditorColnectCountry').value = base?.colnect_country || '';
    document.getElementById('leftEditorColnectWidth').value = base?.colnect_width || '';
    document.getElementById('leftEditorColnectHeight').value = base?.colnect_height || '';
    document.getElementById('leftEditorColnectFormat').value = base?.colnect_format || '';
    document.getElementById('leftEditorColnectEmission').value = base?.colnect_emission || '';
    document.getElementById('leftEditorColnectPrinting').value = base?.colnect_printing || '';
    document.getElementById('leftEditorColnectPaper').value = base?.colnect_paper || '';
    document.getElementById('leftEditorColnectGum').value = base?.colnect_gum || '';
    document.getElementById('leftEditorColnectOrdMi').value = base?.colnect_ord_mi || '';
    document.getElementById('leftEditorColnectOrdSn').value = base?.colnect_ord_sn || '';
    document.getElementById('leftEditorColnectOrdYt').value = base?.colnect_ord_yt || '';
    document.getElementById('leftEditorColnectFaceValue').value = base?.colnect_face_value || '';
    document.getElementById('leftEditorColnectCurrency').value = base?.colnect_currency || '';
    document.getElementById('leftEditorColnectOrdSg').value = base?.colnect_ord_sg || '';
    document.getElementById('leftEditorColnectExpiryDate').value = base?.colnect_expiry_date || '';
    document.getElementById('leftEditorColnectPrintRun').value = base?.colnect_print_run || '';
    document.getElementById('leftEditorColnectColors').value = base?.colnect_colors || '';
    document.getElementById('leftEditorColnectParentItem').value = base?.colnect_parent_item || '';
    document.getElementById('leftEditorColnectVariant').value = base?.colnect_variant || '';
    document.getElementById('leftEditorColnectOverprintVariant').value = base?.colnect_overprint_variant || '';
    document.getElementById('leftEditorColnectScore').value = base?.colnect_score || '';
    document.getElementById('leftEditorColnectAccuracy').value = base?.colnect_accuracy || '';
    document.getElementById('leftEditorColnectList').value = base?.colnect_list || '';
    document.getElementById('leftEditorColnectQuantity').value = base?.colnect_quantity || '';
    document.getElementById('leftEditorColnectCondition').value = base?.colnect_condition || '';
    document.getElementById('leftEditorColnectPublicNote').value = base?.colnect_public_note || '';
    document.getElementById('leftEditorColnectPrivateNote').value = base?.colnect_private_note || '';

    const keepRadio = document.getElementById('leftModeKeep');
    const currentRadio = document.getElementById('leftModeCurrent');
    const uploadRadio = document.getElementById('leftModeUpload');
    if (leftEditorMode === 'edit') {{
        if (keepRadio) keepRadio.checked = true;
    }} else if (currentLeft) {{
        if (currentRadio) currentRadio.checked = true;
    }} else if (uploadRadio) {{
        uploadRadio.checked = true;
    }}
    const uploadInput = document.getElementById('leftEditorUploadFile');
    if (uploadInput) uploadInput.value = '';
    toggleLeftEditorImageOptions();
    document.getElementById('leftEditorBackdrop').style.display = 'flex';
}}

function closeLeftEditor() {{
    const backdrop = document.getElementById('leftEditorBackdrop');
    if (backdrop) backdrop.style.display = 'none';
}}

async function submitLeftEditor() {{
    const sgNumber = document.getElementById('leftEditorSg')?.value || '';
    if (!sgNumber.trim()) {{
        alert('Enter a catalog number / reference code before saving.');
        return;
    }}

    const yearValue = Number(document.getElementById('leftEditorYear')?.value || 0);
    const mode = getSelectedLeftImageMode();
    let uploadDataUrl = null;
    let uploadFilename = null;
    if (mode === 'upload') {{
        const file = document.getElementById('leftEditorUploadFile')?.files?.[0];
        if (!file) {{
            alert('Choose an image file to upload.');
            return;
        }}
        try {{
            uploadDataUrl = await fileToDataUrl(file);
            uploadFilename = file.name || null;
        }} catch (error) {{
            alert(String(error));
            return;
        }}
    }}

    const body = {{
        sg_number: sgNumber,
        catalog_code: getLeftEditorCatalogCodeValue(),
        year: Number.isFinite(yearValue) ? yearValue : 0,
        series: document.getElementById('leftEditorSeries')?.value || '',
        description: document.getElementById('leftEditorDescription')?.value || '',
        watermark: document.getElementById('leftEditorWatermark')?.value || '',
        design: document.getElementById('leftEditorDesign')?.value || '',
        engraving: document.getElementById('leftEditorEngraving')?.value || '',
        perforation: document.getElementById('leftEditorPerforation')?.value || '',
        context: document.getElementById('leftEditorContext')?.value || '',
        width: document.getElementById('leftEditorWidth')?.value || '',
        height: document.getElementById('leftEditorHeight')?.value || '',
        colnect_page_url: document.getElementById('leftEditorColnectPageUrl')?.value || '',
        colnect_image_url: document.getElementById('leftEditorColnectImageUrl')?.value || '',
        colnect_id: document.getElementById('leftEditorColnectId')?.value || '',
        colnect_name: document.getElementById('leftEditorColnectName')?.value || '',
        colnect_catalog_codes: document.getElementById('leftEditorColnectCatalogCodes')?.value || '',
        colnect_country: document.getElementById('leftEditorColnectCountry')?.value || '',
        colnect_width: document.getElementById('leftEditorColnectWidth')?.value || '',
        colnect_height: document.getElementById('leftEditorColnectHeight')?.value || '',
        colnect_format: document.getElementById('leftEditorColnectFormat')?.value || '',
        colnect_emission: document.getElementById('leftEditorColnectEmission')?.value || '',
        colnect_printing: document.getElementById('leftEditorColnectPrinting')?.value || '',
        colnect_paper: document.getElementById('leftEditorColnectPaper')?.value || '',
        colnect_gum: document.getElementById('leftEditorColnectGum')?.value || '',
        colnect_ord_mi: document.getElementById('leftEditorColnectOrdMi')?.value || '',
        colnect_ord_sn: document.getElementById('leftEditorColnectOrdSn')?.value || '',
        colnect_ord_yt: document.getElementById('leftEditorColnectOrdYt')?.value || '',
        colnect_face_value: document.getElementById('leftEditorColnectFaceValue')?.value || '',
        colnect_currency: document.getElementById('leftEditorColnectCurrency')?.value || '',
        colnect_ord_sg: document.getElementById('leftEditorColnectOrdSg')?.value || '',
        colnect_expiry_date: document.getElementById('leftEditorColnectExpiryDate')?.value || '',
        colnect_print_run: document.getElementById('leftEditorColnectPrintRun')?.value || '',
        colnect_colors: document.getElementById('leftEditorColnectColors')?.value || '',
        colnect_parent_item: document.getElementById('leftEditorColnectParentItem')?.value || '',
        colnect_variant: document.getElementById('leftEditorColnectVariant')?.value || '',
        colnect_overprint_variant: document.getElementById('leftEditorColnectOverprintVariant')?.value || '',
        colnect_score: document.getElementById('leftEditorColnectScore')?.value || '',
        colnect_accuracy: document.getElementById('leftEditorColnectAccuracy')?.value || '',
        colnect_list: document.getElementById('leftEditorColnectList')?.value || '',
        colnect_quantity: document.getElementById('leftEditorColnectQuantity')?.value || '',
        colnect_condition: document.getElementById('leftEditorColnectCondition')?.value || '',
        colnect_public_note: document.getElementById('leftEditorColnectPublicNote')?.value || '',
        colnect_private_note: document.getElementById('leftEditorColnectPrivateNote')?.value || '',
        image_mode: mode,
        upload_data_url: uploadDataUrl,
        upload_filename: uploadFilename
    }};

    let endpoint = '/save-left-stamp';
    if (leftEditorMode === 'edit') {{
        const currentLeft = left[leftIdx];
        if (!currentLeft) {{
            alert('No current left stamp to edit.');
            return;
        }}
        body.rowid = Number(currentLeft.rowid);
    }} else {{
        endpoint = '/add-left-stamp';
        body.source_rowid = leftEditorSourceRowid;
    }}

    const response = await fetch(endpoint, {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify(body)
    }});
    const payload = await response.json();
    if (!payload.ok) {{
        alert(payload.message || 'Left-side save failed');
        return;
    }}

    const updatedLeft = payload.updated_left || payload.added_left || null;
    if (updatedLeft) upsertLeftRow(updatedLeft);
    closeLeftEditor();
    document.getElementById('status').innerText = payload.message || 'Left-side item saved';
    render();
}}

async function deleteCurrentLeft() {{
    const currentLeft = left[leftIdx];
    if (!currentLeft) {{
        document.getElementById('status').innerText = 'No current left stamp to delete';
        return;
    }}

    const warning = currentLeft.colnect_id
        ? `Deleting this row will also return linked Colnect candidate ${{currentLeft.colnect_id}} to the right side.\n\nContinue?`
        : 'Delete this left-side base/reference row?';
    if (!confirm(warning)) return;

    const response = await fetch('/delete-left-stamp', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ rowid: Number(currentLeft.rowid) }})
    }});
    const payload = await response.json();
    if (!payload.ok) {{
        alert(payload.message || 'Delete failed');
        return;
    }}

    if (payload.restored_colnect_id) {{
        location.reload();
        return;
    }}

    if (payload.deleted_rowid != null) removeLeftRow(payload.deleted_rowid);
    document.getElementById('status').innerText = payload.message || 'Left-side row deleted';
    render();
}}

async function clearCurrentLeftColnectLink() {{
    const currentLeft = left[leftIdx];
    if (!currentLeft) {{
        document.getElementById('status').innerText = 'No current left stamp selected';
        return;
    }}
    if (!currentLeft.colnect_id) {{
        document.getElementById('status').innerText = 'Current left stamp has no linked Colnect data';
        return;
    }}
    if (!confirm(`Clear linked Colnect data from rowid=${{currentLeft.rowid}} and return ${{currentLeft.colnect_id}} to the candidate list?`)) return;

    const response = await fetch('/clear-left-colnect-link', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ rowid: Number(currentLeft.rowid) }})
    }});
    const payload = await response.json();
    if (!payload.ok) {{
        alert(payload.message || 'Clear link failed');
        return;
    }}

    if (payload.updated_left) upsertLeftRow(payload.updated_left);
    document.getElementById('status').innerText = payload.message || 'Linked Colnect data cleared';
    location.reload();
}}

async function addCurrentColnectAsStamp() {{
    const r = currentRightCandidate();
    if (!r) {{
        setMatchStatus('No current candidate to add');
        return;
    }}

    const assoc = !!document.getElementById('addAssociateImage')?.checked;
    const mode = getSelectedAddImageMode();
    const l = left[leftIdx];

    let uploadDataUrl = null;
    let uploadFilename = null;
    if (assoc && mode === 'upload') {{
        const input = document.getElementById('addUploadFile');
        const file = input?.files?.[0];
        if (!file) {{
            alert('Choose an image file to upload, or switch image mode.');
            return;
        }}
        try {{
            uploadDataUrl = await fileToDataUrl(file);
            uploadFilename = file.name || null;
        }} catch (e) {{
            alert(String(e));
            return;
        }}
    }}

    const body = {{
        colnect_id: r.colnect_id,
        source_left_rowid: l ? Number(l.rowid) : null,
        target_sg_number: document.getElementById('addSg')?.value || r.sg_number || '',
        target_year: Number(document.getElementById('addYear')?.value || r.year || 0),
        target_series: document.getElementById('addSeries')?.value || r.series || '',
        associate_image: assoc,
        image_mode: assoc ? mode : 'none',
        upload_data_url: uploadDataUrl,
        upload_filename: uploadFilename
    }};

    const res = await fetch('/add-colnect-stamp', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify(body)
    }});
    const j = await res.json();
    if (!j.ok) {{
        alert(j.message || 'Add failed');
        return;
    }}

    if (j.added_left) {{
        left.push(j.added_left);
    }}
    if (j.processed_colnect_id) {{
        removeCandidateFromUi(j.processed_colnect_id);
    }}
    closeAddDialog();
    setMatchStatus(j.message || 'Added to stamps');
    render();
}}

async function deleteCurrentCandidate() {{
    const r = currentRightCandidate();
    if (!r) {{
        setMatchStatus('No current candidate to delete');
        return;
    }}

    if (!confirm(`Soft delete colnect candidate ${{r.colnect_id}} from colnect_stamps?`)) {{
        return;
    }}

    const res = await fetch('/delete-colnect-candidate', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ colnect_id: r.colnect_id }})
    }});
    const j = await res.json();
    if (!j.ok) {{
        alert(j.message || 'Delete failed');
        return;
    }}

    const removedAssignments = [];
    Object.keys(assignedByLeft).forEach(k => {{
        if (assignedByLeft[k] === r.colnect_id) {{
            removedAssignments.push([k, assignedByLeft[k], selectedCatalogCodeByLeft[k] || '', selectedCatalogTypeByLeft[k] || '']);
        }}
    }});
    deletedCandidatesStack.push({{ candidate: r, removedAssignments }});

    removeCandidateFromUi(r.colnect_id);
    setMatchStatus(j.message || 'Soft-deleted candidate');
    render();
}}

async function undoLastDelete() {{
    const last = deletedCandidatesStack.pop();
    if (!last) {{
        setMatchStatus('Nothing to undo');
        return;
    }}

    const res = await fetch('/undo-colnect-delete', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ colnect_id: last.candidate.colnect_id }})
    }});
    const j = await res.json();
    if (!j.ok) {{
        deletedCandidatesStack.push(last);
        alert(j.message || 'Undo failed');
        return;
    }}

    if (!rightAll.some(c => c.colnect_id === last.candidate.colnect_id)) {{
        rightAll.push(last.candidate);
    }}
    for (const [k, cid, code, catalogType] of last.removedAssignments || []) {{
        assignedByLeft[k] = cid;
        if (code) selectedCatalogCodeByLeft[k] = code;
        if (catalogType) selectedCatalogTypeByLeft[k] = catalogType;
    }}
    setMatchStatus(j.message || 'Undo complete');
    render();
}}

async function closeReviewUi() {{
    const statusEl = document.getElementById('status');
    if (window.__closeReviewInFlight === true) {{
        if (statusEl) statusEl.innerText = 'Close request already in progress...';
        return;
    }}

    // Show a 3-button modal: Generate & Close | Close Only | Cancel
    const CLOSE_PDF_PREF_KEY = 'stampAlbumReviewUiClosePdfEnabled';
    const choice = await new Promise(resolve => {{
        const configDefault = pdfGenerationDefaults.enabledByDefault && pdfGenerationDefaults.cliConfigured;
        // Persist the user's last preference across sessions; fall back to config default on first use.
        const savedPref = localStorage.getItem(CLOSE_PDF_PREF_KEY);
        const pdfChecked = pdfGenerationDefaults.cliConfigured &&
            (savedPref !== null ? savedPref === 'true' : configDefault);
        const pdfOptionHtml =
            '<label style="display:flex;align-items:flex-start;gap:0.65em;margin:0 0 1.1em;text-align:left;font-size:0.95em;line-height:1.4">' +
            `<input id="_dlg_pdf" type="checkbox" ${{pdfChecked ? 'checked' : ''}} ${{pdfGenerationDefaults.cliConfigured ? '' : 'disabled'}} style="margin-top:0.2em" />` +
            '<span>Generate PDF with AlbumEasy CLI <code>-c</code> and open it after generation</span>' +
            '</label>' +
            (pdfGenerationDefaults.cliConfigured
                ? ''
                : '<div style="margin:-0.45em 0 1.1em;text-align:left;color:#8a3b12;font-size:0.9em">AlbumEasy CLI is not configured. Set runtime.albumeasy_cli_path in config.toml to enable this.</div>');
        const dlg = document.createElement('dialog');
        dlg.style.cssText = [
            'padding:1.8em 2.2em',
            'border-radius:10px',
            'font-family:sans-serif',
            'border:1px solid #bbb',
            'min-width:320px',
            'text-align:center',
            'box-shadow:0 4px 24px rgba(0,0,0,.25)',
        ].join(';');
        dlg.innerHTML =
            '<p style="margin:0 0 1.2em;font-size:1.05em;font-weight:500">Close Review UI?</p>' +
            pdfOptionHtml +
            '<div style="display:flex;gap:0.6em;justify-content:center;flex-wrap:wrap">' +
            '<button id="_dlg_gen"  style="padding:0.5em 1.1em;background:#1a73e8;color:#fff;border:none;border-radius:5px;cursor:pointer;font-size:.95em">Generate &amp; Close</button>' +
            '<button id="_dlg_cls"  style="padding:0.5em 1.1em;background:#555;color:#fff;border:none;border-radius:5px;cursor:pointer;font-size:.95em">Close / End</button>' +
            '<button id="_dlg_cxl" style="padding:0.5em 1.1em;background:#e9e9e9;color:#333;border:none;border-radius:5px;cursor:pointer;font-size:.95em">Cancel</button>' +
            '</div>';
        document.body.appendChild(dlg);
        dlg.showModal();
        const done = v => {{
            const cbChecked = !!dlg.querySelector('#_dlg_pdf')?.checked;
            const generatePdf = v === 'generate' && cbChecked;
            // Persist the checkbox state (even on close/cancel so next open respects it).
            if (v !== 'cancel') {{
                try {{ localStorage.setItem(CLOSE_PDF_PREF_KEY, String(cbChecked)); }} catch (_) {{}}
            }}
            dlg.close();
            dlg.remove();
            resolve({{ choice: v, generatePdf }});
        }};
        dlg.querySelector('#_dlg_gen').onclick  = () => done('generate');
        dlg.querySelector('#_dlg_cls').onclick  = () => done('close');
        dlg.querySelector('#_dlg_cxl').onclick  = () => done('cancel');
        dlg.addEventListener('cancel', () => done('cancel')); // Esc key
    }});

    if (choice.choice === 'cancel') return;

    const generateAlbum = choice.choice === 'generate';
    const closeBusyMsg = generateAlbum
        ? (choice.generatePdf
            ? 'Generating album text and PDF, then closing Review UI...'
            : 'Generating album text, then closing Review UI...')
        : 'Closing Review UI...';
    window.__closeReviewInFlight = true;
    if (statusEl) statusEl.innerText = closeBusyMsg;
    setOperationBusy(true, closeBusyMsg);

    let j;
    try {{
        const res = await fetch('/shutdown', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{
                generate_album: generateAlbum,
                generate_pdf: choice.generatePdf,
                start_year: (() => {{ const v = document.getElementById('reviewPdfStartYear')?.value.trim(); return v ? parseInt(v, 10) : null; }})(),
                end_year:   (() => {{ const v = document.getElementById('reviewPdfEndYear')?.value.trim();   return v ? parseInt(v, 10) : null; }})(),
            }})
        }});

        j = await res.json();
    }} catch (e) {{
        setOperationBusy(false, '');
        window.__closeReviewInFlight = false;
        if (statusEl) statusEl.innerText = 'Close failed: ' + e;
        alert('Failed to close Review UI: ' + e);
        return;
    }}

    setOperationBusy(false, '');
    window.__closeReviewInFlight = false;

    // Build a clear alert: include PDF status (generated, skipped, or failed).
    const parts = [j.message || 'Review UI shutdown request sent.'];
    if (j.generated_output_path) parts.push(`\nOutput: ${{j.generated_output_path}}`);
    if (choice.generatePdf) {{
        if (j.generated_pdf_path) {{
            parts.push(`\nPDF: ${{j.generated_pdf_path}}`);
        }} else {{
            parts.push('\nPDF generation was requested but no PDF was produced. Check that AlbumEasy CLI is configured and the output file is valid.');
        }}
    }}
    alert(parts.join(''));

    const makeFileUrl = path => {{
        const p = String(path || '').trim();
        if (!p) return '';
        const normalized = p.startsWith('/') ? p : ('/' + p);
        return 'file://' + encodeURI(normalized);
    }};

    const outputPath = String(j.generated_output_path || '').trim();
    const pdfPath = String(j.generated_pdf_path || '').trim();
    const outputUrl = makeFileUrl(outputPath);
    const pdfUrl = makeFileUrl(pdfPath);

    const openLinksHtml = [
        outputUrl
            ? `<p><a href="${{html(outputUrl)}}" target="_blank" rel="noopener">Open generated album text</a><br/><small>${{html(outputPath)}}</small></p>`
            : '',
        pdfUrl
            ? `<p><a href="${{html(pdfUrl)}}" target="_blank" rel="noopener">Open generated PDF</a><br/><small>${{html(pdfPath)}}</small></p>`
            : ''
    ].join('');

    document.body.innerHTML =
        "<div style='font-family:sans-serif;margin:20px;max-width:780px;'>" +
        "<h2>Review UI closed</h2>" +
        `<p>${{html(j.message || 'Review UI shutdown request sent.')}}</p>` +
        openLinksHtml +
        "<p>You can close this browser tab now.</p>" +
        "</div>";

    // Try to close if this tab was script-opened; if blocked, keep the page with links.
    window.close();
}}

async function returnToStartReviewPage() {{
    const statusEl = document.getElementById('status');
    const originalText = statusEl?.innerText || 'Ready';
    if (window.__returnToStartInFlight) {{
        if (statusEl) statusEl.innerText = 'Return to Start is already in progress...';
        return;
    }}
    window.__returnToStartInFlight = true;

    if (statusEl) statusEl.innerText = 'Step 1/2: Switching server mode...';
    setOperationBusy(true, 'Step 1/2: Switching server mode...');

    try {{
        const res = await fetch('/return-to-start', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await res.json();
        if (!j.ok) {{
            if (statusEl) statusEl.innerText = 'Return to Start failed';
            setOperationBusy(false, '');
            alert(j.message || 'Failed to return to Start Review page.');
            return;
        }}

        if (statusEl) statusEl.innerText = 'Step 2/2: Loading country selector...';
        setOperationBusy(true, 'Step 2/2: Loading country selector...');

        const ready = await waitForBootstrapReady(20, 250);
        if (ready) {{
            setOperationBusy(true, 'Step 2/2: Loading country selector... Redirecting...');
            await navigateWithFade('/');
            return;
        }}

        setOperationBusy(true, 'Step 2/2: Loading country selector... Redirecting with fallback...');
        await navigateWithFade('/');
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Return to Start request failed';
        setOperationBusy(false, '');
        alert('Return to Start request failed: ' + e);
    }} finally {{
        window.__returnToStartInFlight = false;
        setTimeout(() => {{
            if (statusEl) statusEl.innerText = originalText;
        }}, 2200);
    }}
}}

if (document.body) {{
    document.body.classList.add('pageEnter');
    setTimeout(() => document.body.classList.remove('pageEnter'), 260);
}}

async function generatePdfNowReview() {{
    const statusWrap = document.getElementById('pdfStatusWrap');
    const spinnerEl = document.getElementById('pdfStatusSpinner');
    const statusEl = document.getElementById('pdfStatus');
    const btnEl = document.getElementById('reviewPdfBtn');
    const previewBtnEl = document.getElementById('reviewPdfPreviewBtn');
    const alwaysOpenPdfEl = document.getElementById('reviewPdfAlwaysShow');
    const startYearInput = document.getElementById('reviewPdfStartYear');
    const endYearInput = document.getElementById('reviewPdfEndYear');
    if (window.__reviewPdfRestoreTimer) {{
        clearTimeout(window.__reviewPdfRestoreTimer);
        window.__reviewPdfRestoreTimer = null;
    }}

    const setPdfUiState = (active, message) => {{
        if (statusEl && typeof message === 'string') statusEl.innerText = message;
        if (statusWrap) statusWrap.style.display = 'flex';
        if (spinnerEl) spinnerEl.style.display = active ? 'inline-block' : 'none';
        if (btnEl) btnEl.disabled = active;
        if (previewBtnEl) previewBtnEl.disabled = active || !String(reviewLastGeneratedPdfPath || '').trim();
        if (alwaysOpenPdfEl) alwaysOpenPdfEl.disabled = active;
        if (startYearInput) startYearInput.disabled = active;
        if (endYearInput) endYearInput.disabled = active;
    }};

    if (window.__reviewPdfRequestInFlight === true) {{
        if (statusEl) statusEl.innerText = 'PDF generation already in progress. Please wait.';
        if (statusWrap) statusWrap.style.display = 'inline-flex';
        return;
    }}

    const startYearVal = document.getElementById('reviewPdfStartYear')?.value.trim();
    const endYearVal   = document.getElementById('reviewPdfEndYear')?.value.trim();
    const startYear    = startYearVal ? parseInt(startYearVal, 10) : null;
    const endYear      = endYearVal   ? parseInt(endYearVal, 10)   : null;
    const yearDesc     = (startYear || endYear) ? ` (years ${{startYear || 'start'}}–${{endYear || 'end'}})` : '';
    if (statusEl) statusEl.classList.remove('pdfStatusDone', 'pdfStatusFail');
    const phases = [
        'Preparing PDF request',
        'Generating album text',
        'Running AlbumEasy CLI',
        'Finalizing PDF output'
    ];
    let phaseIdx = 0;
    setPdfUiState(true, phases[phaseIdx] + yearDesc + '...');
    const phaseTimer = setInterval(() => {{
        phaseIdx = (phaseIdx + 1) % phases.length;
        setPdfUiState(true, phases[phaseIdx] + yearDesc + '...');
    }}, 1200);

    window.__reviewPdfRequestInFlight = true;

    try {{
        const res = await fetch('/generate-pdf-now', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{
                start_year: startYear,
                end_year: endYear,
                open_pdf: !!alwaysOpenPdfEl?.checked
            }})
        }});
        const j = await res.json();
        if (!j.ok) {{
            if (statusEl) statusEl.classList.add('pdfStatusFail');
            setPdfUiState(false, j.message || 'PDF generation failed.');
            persistPdfUiState(j.message || 'PDF generation failed.', 'fail');
            return;
        }}

        reviewLastGeneratedPdfPath = String(j.generated_pdf_path || '');
        updatePdfPreviewControls();

        if (statusEl) statusEl.classList.add('pdfStatusDone');
        if (reviewLastGeneratedPdfPath.trim()) {{
            const openedTxt = alwaysOpenPdfEl?.checked
                ? 'Opened automatically. Use Preview Last PDF to open it again.'
                : 'Use Preview Last PDF to open it.';
            const finalMsg = (j.message || 'PDF generation completed.') + ' ' + openedTxt;
            setPdfUiState(false, finalMsg);
            persistPdfUiState(finalMsg, 'done');
        }} else {{
            const finalMsg = j.message || 'PDF generation completed, but no PDF was produced.';
            setPdfUiState(false, finalMsg);
            persistPdfUiState(finalMsg, 'fail');
        }}
    }} catch (e) {{
        if (statusEl) statusEl.classList.add('pdfStatusFail');
        const finalMsg = 'PDF generation request failed: ' + e;
        setPdfUiState(false, finalMsg);
        persistPdfUiState(finalMsg, 'fail');
    }} finally {{
        window.__reviewPdfRequestInFlight = false;
        clearInterval(phaseTimer);
        window.__reviewPdfRestoreTimer = null;
    }}
}}

async function importColnectFilesReview() {{
    const btn = document.getElementById('importColnectBtn');
    const statusBox = document.getElementById('colnectImportStatus');
    const statusText = document.getElementById('colnectImportStatusText');
    const logEl = document.getElementById('colnectImportLog');

    if (btn) btn.disabled = true;
    if (statusBox) statusBox.style.display = '';
    if (statusText) {{ statusText.className = 'colnectImportStatusText'; statusText.textContent = 'Starting import\u2026'; }}
    if (logEl) {{
        logEl.style.display = '';
        const sep = document.createElement('div');
        sep.className = 'colnectLogLine';
        sep.textContent = '--- ' + new Date().toLocaleTimeString() + ' import started ---';
        logEl.appendChild(sep);
        while (logEl.children.length > 20) logEl.removeChild(logEl.firstChild);
    }}

    const spinner = statusBox ? statusBox.querySelector('.colnectImportSpinner') : null;
    if (spinner) spinner.style.display = 'inline-block';

    // Snapshot current runtime log position so only new lines appear
    let shownLogCount = 0;
    try {{
        const snap = await fetch('/runtime-log', {{ cache: 'no-store' }});
        const snapJ = await snap.json();
        shownLogCount = (snapJ.lines || []).length;
    }} catch (_) {{}}

    const importKeywords = ['parsing', 'parsed', 'imported', 'assessed', 'already', 'ingest', 'csv', 'failed', 'stamp'];

    function appendLogLine(text) {{
        if (!logEl) return;
        const el = document.createElement('div');
        el.className = 'colnectLogLine';
        el.textContent = text;
        logEl.appendChild(el);
        while (logEl.children.length > 12) logEl.removeChild(logEl.firstChild);
        logEl.scrollTop = logEl.scrollHeight;
    }}

    async function pollImportLog() {{
        try {{
            const r = await fetch('/runtime-log', {{ cache: 'no-store' }});
            const j = await r.json();
            const lines = j.lines || [];
            const newLines = lines.slice(shownLogCount);
            shownLogCount = lines.length;
            for (const line of newLines) {{
                if (importKeywords.some(k => line.toLowerCase().includes(k))) {{
                    appendLogLine(line);
                }}
            }}
        }} catch (_) {{}}
    }}

    const logPollTimer = setInterval(pollImportLog, 800);

    try {{
        const r = await fetch('/api/colnect-import', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();
        clearInterval(logPollTimer);
        await pollImportLog(); // final sweep to catch last log lines
        if (spinner) spinner.style.display = 'none';

        const imported = j.stamps_imported || 0;
        const skipped = j.stamps_skipped || 0;

        if (j.ok && imported > 0) {{
            if (statusText) {{
                statusText.className = 'colnectImportStatusText colnectImportDone';
                statusText.textContent = '\u2714 Imported ' + imported + ' new stamp' + (imported === 1 ? '' : 's') +
                    (skipped > 0 ? ', ' + skipped + ' already in database' : '');
            }}
            appendLogLine('✔ ' + (j.message || 'Import completed'));
            if (btn) btn.disabled = false;
            // Refresh right-panel candidates without a full page reload so that
            // newly imported Colnect stamps appear immediately in the list.
            try {{
                const cr = await fetch('/api/fresh-candidates', {{ cache: 'no-store' }});
                const freshRight = await cr.json();
                if (Array.isArray(freshRight) && freshRight.length > 0) {{
                    rightAll = freshRight;
                    render();
                    appendLogLine('\u21bb Right panel refreshed (' + freshRight.length + ' candidates)');
                }}
            }} catch (_) {{
                appendLogLine('(Could not auto-refresh right panel \u2014 please reload if list looks stale)');
            }}
        }} else if (j.ok && imported === 0) {{
            if (statusText) {{
                statusText.className = 'colnectImportStatusText';
                statusText.textContent = skipped > 0
                    ? '\u2714 ' + skipped + ' assessed \u2014 all already in database'
                    : 'No new stamps found';
            }}
            if (btn) btn.disabled = false;
            appendLogLine(j.message || 'No new stamps found');
            if (j.errors && j.errors.length > 0) {{
                for (const err of j.errors) appendLogLine('! ' + err);
            }}
        }} else {{
            if (statusText) {{
                statusText.className = 'colnectImportStatusText colnectImportFail';
                statusText.textContent = '\u2718 ' + (j.message || 'Import failed');
            }}
            if (btn) btn.disabled = false;
            appendLogLine('✘ ' + (j.message || 'Import failed'));
            if (j.errors && j.errors.length > 0) {{
                for (const err of j.errors) appendLogLine('! ' + err);
            }}
        }}
    }} catch (e) {{
        clearInterval(logPollTimer);
        if (spinner) spinner.style.display = 'none';
        if (statusText) {{
            statusText.className = 'colnectImportStatusText colnectImportFail';
            statusText.textContent = '\u2718 Import error';
        }}
        appendLogLine('Error: ' + String(e));
        if (btn) btn.disabled = false;
    }}
}}

async function pickAndCopyColnectFileReview() {{
    const btn = document.getElementById('importColnectPickBtn');
    const statusBox = document.getElementById('colnectImportStatus');
    const statusText = document.getElementById('colnectImportStatusText');
    const logEl = document.getElementById('colnectImportLog');
    const countBadge = document.getElementById('reviewColnectCopyCountBadge');
    const destEl = document.getElementById('reviewColnectDirPath');

    if (btn) btn.disabled = true;
    if (statusBox) statusBox.style.display = '';
    if (statusText) {{
        statusText.className = 'colnectImportStatusText';
        statusText.textContent = 'Opening file picker\u2026';
    }}

    const spinner = statusBox ? statusBox.querySelector('.colnectImportSpinner') : null;
    if (spinner) spinner.style.display = 'none';

    if (logEl && !logEl.innerHTML) {{
        logEl.style.display = '';
        const line = document.createElement('div');
        line.className = 'colnectLogLine';
        line.textContent = 'Ready to pick a CSV and copy to the country colnect folder.';
        logEl.appendChild(line);
    }}

    try {{
        const r = await fetch('/api/review-pick-colnect-file', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();

        if (destEl && j.colnect_dir) destEl.textContent = j.colnect_dir;

        if (j.ok && j.picked) {{
            reviewColnectFileCount += 1;
            if (countBadge) countBadge.textContent = 'Files: ' + reviewColnectFileCount;
            if (statusText) {{
                statusText.className = 'colnectImportStatusText colnectImportDone';
                statusText.textContent = '\u2714 Copied: ' + (j.file_name || 'CSV');
            }}
            if (logEl) {{
                const line = document.createElement('div');
                line.className = 'colnectLogLine';
                line.textContent = '\u2714 ' + (j.message || 'Copied to destination folder');
                logEl.appendChild(line);
                while (logEl.children.length > 12) logEl.removeChild(logEl.firstChild);
                logEl.scrollTop = logEl.scrollHeight;
            }}
        }} else if (j.ok && !j.picked) {{
            if (statusText) {{
                statusText.className = 'colnectImportStatusText';
                statusText.textContent = 'Cancelled';
            }}
        }} else {{
            if (statusText) {{
                statusText.className = 'colnectImportStatusText colnectImportFail';
                statusText.textContent = '\u2718 ' + (j.message || 'Copy failed');
            }}
            if (logEl) {{
                const line = document.createElement('div');
                line.className = 'colnectLogLine';
                line.textContent = '\u2718 ' + (j.message || 'Copy failed');
                logEl.appendChild(line);
                while (logEl.children.length > 12) logEl.removeChild(logEl.firstChild);
                logEl.scrollTop = logEl.scrollHeight;
            }}
        }}
    }} catch (e) {{
        if (statusText) {{
            statusText.className = 'colnectImportStatusText colnectImportFail';
            statusText.textContent = '\u2718 Copy error';
        }}
        if (logEl) {{
            const line = document.createElement('div');
            line.className = 'colnectLogLine';
            line.textContent = 'Error: ' + String(e);
            logEl.appendChild(line);
            while (logEl.children.length > 12) logEl.removeChild(logEl.firstChild);
            logEl.scrollTop = logEl.scrollHeight;
        }}
    }} finally {{
        if (btn) btn.disabled = false;
    }}
}}

const seriesToggle = document.getElementById('f_series');
if (seriesToggle) {{
  seriesToggle.checked = checkboxDefaults.f_series;
  seriesToggle.addEventListener('change', () => {{
    seriesToggle.dataset.userTouched = '1';
  }});
}}

// Apply all checkbox defaults from config
for (const [checkboxId, checked] of Object.entries(checkboxDefaults)) {{
  const el = document.getElementById(checkboxId);
  if (el) {{
    el.checked = checked;
    el.addEventListener('change', () => {{
      el.dataset.userTouched = '1';
    }});
  }}
}}

try {{
    const storedCatalog = String(localStorage.getItem(countryCatalogStorageKey) || '').toLowerCase();
    countryDefaultCatalogCode = normalizeCatalogCodeForUi(storedCatalog)
        || normalizeCatalogCodeForUi(configDefaultCatalogCode)
        || 'sg';
}} catch (_) {{
    countryDefaultCatalogCode = normalizeCatalogCodeForUi(configDefaultCatalogCode) || 'sg';
}}

const defaultCatalogSelect = document.getElementById('defaultCatalogSelect');
if (defaultCatalogSelect) {{
    defaultCatalogSelect.value = normalizeCatalogCodeForUi(countryDefaultCatalogCode) || 'sg';
    if (!defaultCatalogSelect.value) defaultCatalogSelect.value = 'sg';
}}

const reviewColnectDirPathEl = document.getElementById('reviewColnectDirPath');
if (reviewColnectDirPathEl) reviewColnectDirPathEl.textContent = reviewColnectDir || '(unknown)';
const reviewColnectCopyCountBadgeEl = document.getElementById('reviewColnectCopyCountBadge');
if (reviewColnectCopyCountBadgeEl) reviewColnectCopyCountBadgeEl.textContent = 'Files: ' + reviewColnectFileCount;

const reviewPdfAlwaysShowEl = document.getElementById('reviewPdfAlwaysShow');
if (reviewPdfAlwaysShowEl) {{
    try {{
        reviewPdfAlwaysShowEl.checked = localStorage.getItem(reviewPdfAlwaysOpenStorageKey) === '1';
    }} catch (_) {{
        reviewPdfAlwaysShowEl.checked = false;
    }}
}}
updatePdfPreviewControls();
restorePdfUiState();

const sortModeSelect = document.getElementById('sortMode');
if (sortModeSelect) sortModeSelect.value = sortMode;
const showImagelessLeftToggle = document.getElementById('f_show_imageless_left');
if (showImagelessLeftToggle) showImagelessLeftToggle.checked = showImagelessLeftRowsDefault;
sortLeftRows(left[leftIdx]?.rowid);

ensureCopyableAlertDialogCompare();
try {{
    const v = localStorage.getItem(runtimeLogCompareVisibleStorageKey);
    const cb = document.getElementById('showRuntimeLogCompare');
    if (cb && (v === '0' || v === '1')) cb.checked = v === '1';
}} catch (_) {{}}
applyRuntimeLogVisibilityCompare();
startRuntimeLogPollingCompare();
startBackendHealthPollingCompare();
render();
document.querySelectorAll('.copyrightCurrentYear').forEach((el) => {{
    el.textContent = String(new Date().getFullYear());
}});
</script>
{footer_html}
<script>
document.querySelectorAll('.copyrightCurrentYear').forEach((el) => {{
    el.textContent = String(new Date().getFullYear());
}});
</script>
{config_editor_assets}
</body>
</html>"#,
        json = json,
        backend_not_responding_message = BACKEND_NOT_RESPONDING_MESSAGE,
        footer_html = render_app_footer_html(),
        config_editor_assets = render_config_editor_assets()
    )
}

#[cfg(test)]
mod tests {
    use super::{render_bootstrap_html, render_html};
    use crate::review_ui::MatcherPayload;

    fn minimal_payload() -> MatcherPayload {
        MatcherPayload {
            left: vec![],
            right: vec![],
            album_name: "Testland".to_string(),
            colnect_dir: "Albums/Testland/colnect_inbox".to_string(),
            colnect_file_count: 0,
            show_imageless_left_rows: false,
            default_catalog_code: "sg".to_string(),
            odd_row_color: "#ffffff".to_string(),
            even_row_color: "#f2f2f2".to_string(),
            series_default_checked: true,
            description_default_checked: true,
            watermark_default_checked: true,
            design_default_checked: true,
            engraving_default_checked: true,
            perforation_default_checked: true,
            context_default_checked: true,
            colnect_links_default_checked: true,
            colnect_image_url_default_checked: true,
            colnect_metadata_default_checked: true,
            replace_sg_number_default_checked: false,
            right_unmatched_only_default_checked: false,
            albumeasy_auto_generate_pdf_default: false,
            albumeasy_cli_configured: false,
            catalog_family_options: vec![],
        }
    }

    #[test]
    fn bootstrap_html_uses_default_inbox_hint_when_missing() {
        let html = render_bootstrap_html(None);
        assert!(html.contains("Colnect inbox hint: samples/colnect_inbox"));
    }

    #[test]
    fn bootstrap_html_uses_custom_inbox_hint_when_present() {
        let html = render_bootstrap_html(Some("Albums/Testland/custom_inbox"));
        assert!(html.contains("Colnect inbox hint: Albums/Testland/custom_inbox"));
    }

    #[test]
    fn bootstrap_html_contains_health_and_runtime_log_hooks() {
        let html = render_bootstrap_html(None);
        assert!(html.contains("/health"));
        assert!(html.contains("/runtime-log"));
        assert!(html.contains("id='runtimeLog'"));
    }

    #[test]
    fn compare_html_embeds_payload_and_core_shell() {
        let payload = minimal_payload();
        let html = render_html(&payload);
        assert!(html.contains("const payload ="));
        assert!(html.contains("\"album_name\":\"Testland\""));
        assert!(html.contains("id='leftPanel'"));
        assert!(html.contains("id='rightPanel'"));
    }

    #[test]
    fn compare_html_contains_backend_recovery_banner() {
        let payload = minimal_payload();
        let html = render_html(&payload);
        assert!(html.contains("id='reconnectBannerCompare'"));
        assert!(html.contains("retryBackendHealthCheckCompare"));
        assert!(html.contains("Connection lost, retrying..."));
    }
}
