use super::{BACKEND_NOT_RESPONDING_MESSAGE, FOOTER_YEAR_UPDATE_JS};

pub(super) fn render_bootstrap_script_html() -> String {
    format!(
        r#"<script>
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
let stampworldPollTick = 0;
let stampworldRunRequested = false;
let nativeDialogInFlight = false;
const runtimeLogVisibleStorageKey = 'reviewui.bootstrap.showRuntimeLog';
const introPanelStorageKey = 'reviewui.bootstrap.introPanel';
const albumsRootPanelStorageKey = 'reviewui.bootstrap.albumsRootPanel';
const stampworldPanelStorageKey = 'reviewui.bootstrap.stampworldPanel';
const colnectCopyPanelStorageKey = 'reviewui.bootstrap.colnectCopyPanel';
const selectedCountryStorageKey = 'reviewui.bootstrap.selectedCountry';
const stampworldStartYearStorageKey = 'reviewui.stampworld.startYear';
const stampworldEndYearStorageKey = 'reviewui.stampworld.endYear';
const stampworldMinYear = 1840;
const backendHealthPollMs = 15000;
function initCollapsiblePanel(panelId, storageKey) {{
        try {{
                const panel = document.getElementById(panelId);
                if (!panel) return;
                const savedState = localStorage.getItem(storageKey);
                if (savedState === 'closed') panel.removeAttribute('open');
                if (savedState === 'open') panel.setAttribute('open', '');
                panel.addEventListener('toggle', function() {{
                        try {{ localStorage.setItem(storageKey, panel.open ? 'open' : 'closed'); }} catch (_) {{}}
                }});
        }} catch (_) {{}}
}}

initCollapsiblePanel('introPanel', introPanelStorageKey);
initCollapsiblePanel('albumsRootPanel', albumsRootPanelStorageKey);
initCollapsiblePanel('stampworldPanel', stampworldPanelStorageKey);
initCollapsiblePanel('colnectCopyPanel', colnectCopyPanelStorageKey);
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
    try {{ if (sel && sel.value) localStorage.setItem(selectedCountryStorageKey, sel.value); }} catch(_) {{}}
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
    const swCountryRow = document.getElementById('swCountryRow');
    const swCountryFixedRow = document.getElementById('swCountryFixedRow');
    const swAlbumeasyRow = document.getElementById('swAlbumeasyRow');
    if (swCountry) swCountry.value = typed;
    if (swCountryRow) swCountryRow.style.display = '';
    if (swCountryFixedRow) swCountryFixedRow.style.display = 'none';
    if (swAlbumeasyRow) swAlbumeasyRow.style.display = '';
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
    const minYear = stampworldMinYear;
    const maxYear = new Date().getFullYear();
    const defaultStartYear = Math.max(minYear, maxYear - 2);
    const defaultEndYear = maxYear;

    let stickyStartYear = defaultStartYear;
    let stickyEndYear = defaultEndYear;
    try {{
        const storedStart = parseInt(localStorage.getItem(stampworldStartYearStorageKey) || '', 10);
        const storedEnd = parseInt(localStorage.getItem(stampworldEndYearStorageKey) || '', 10);
        if (Number.isFinite(storedStart)) stickyStartYear = Math.min(Math.max(storedStart, minYear), maxYear);
        if (Number.isFinite(storedEnd)) stickyEndYear = Math.min(Math.max(storedEnd, minYear), maxYear);
    }} catch (_) {{}}

    populateYearSelect('swStartYear', minYear, maxYear, stickyStartYear);
    populateYearSelect('swEndYear', minYear, maxYear, stickyEndYear);
}}

function persistStampWorldYearRange() {{
    const maxYear = new Date().getFullYear();
    const startYear = parseInt(document.getElementById('swStartYear')?.value || '', 10);
    const endYear = parseInt(document.getElementById('swEndYear')?.value || '', 10);
    try {{
        if (Number.isFinite(startYear)) localStorage.setItem(stampworldStartYearStorageKey, String(Math.min(Math.max(startYear, stampworldMinYear), maxYear)));
        if (Number.isFinite(endYear)) localStorage.setItem(stampworldEndYearStorageKey, String(Math.min(Math.max(endYear, stampworldMinYear), maxYear)));
    }} catch (_) {{}}
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
    const currentYear = new Date().getFullYear();
    const defaultStartYear = Math.max(stampworldMinYear, currentYear - 2);
    const y1 = document.getElementById('swStartYear').value || String(defaultStartYear);
    const y2 = document.getElementById('swEndYear').value || String(currentYear);
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
        stampworldPollTick += 1;
        const status = document.getElementById('status');
        if (status && stampworldPollTick % 5 === 1) {{
            status.innerText = `StampWorld status poll #${{stampworldPollTick}}...`;
        }}
        const r = await fetch('/api/stampworld-status', {{ cache: 'no-store' }});
        const j = await r.json();
        if (!j.ok) {{
            const msg = j.message || 'StampWorld status poll returned not-ok payload.';
            updateStampWorldRunStatus(msg, stampworldRunRequested, false);
            if (status) status.innerText = msg;
            return;
        }}

        const noJobStartedYet = !j.running && !j.finished && !j.country && !j.db_path;
        if (noJobStartedYet && !stampworldRunRequested) {{
            updateStampWorldRunStatus('Ready to run', false, false);
            if (status) status.innerText = 'Ready.';
            setBusy(false, '');
            return;
        }}

        if (j.running) {{
            updateStampWorldRunStatus(j.message || 'StampWorld pull running...', true, false);
            
            // Fetch and display real-time log
            try {{
                const logR = await fetch('/api/stampworld-log-tail', {{ cache: 'no-store' }});
                const logJ = await logR.json();
                if (logJ.ok && logJ.lines && logJ.lines.length > 0) {{
                    const logPanel = document.getElementById('swPullLogPanel');
                    const logArea = document.getElementById('swPullLog');
                    if (logPanel && logArea) {{
                        logPanel.style.display = '';
                        logArea.value = logJ.lines.join('\\n');
                        logArea.scrollTop = logArea.scrollHeight;
                    }}
                }}
            }} catch (e) {{
                const status = document.getElementById('status');
                const msg = 'StampWorld log poll failed: ' + e;
                if (status) status.innerText = msg;
            }}
            
            return;
        }}

        if (!j.finished) {{
            // Transient idle state: keep polling instead of stopping immediately.
            const idleMsg = j.message || 'StampWorld status is idle; waiting for run state...';
            updateStampWorldRunStatus(idleMsg, stampworldRunRequested, false);
            if (status) status.innerText = idleMsg;
            if (stampworldRunRequested && stampworldPollTick < 20) return;
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
            stampworldRunRequested = false;
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
        }} else {{
            const timeoutMsg = 'StampWorld status did not transition from idle. Request may not have started.';
            stampworldRunRequested = false;
            updateStampWorldRunStatus(timeoutMsg, false, false);
            if (status) status.innerText = timeoutMsg;
        }}
    }} catch (e) {{
        const status = document.getElementById('status');
        const msg = 'StampWorld status poll failed: ' + e;
        updateStampWorldRunStatus(msg, stampworldRunRequested, false);
        if (status) status.innerText = msg;
    }}
}}

async function runStampWorldPull() {{
    const country = (document.getElementById('swCountry')?.value || '').trim();
    const currentYear = new Date().getFullYear();
    const defaultStartYear = Math.max(stampworldMinYear, currentYear - 2);
    const startYearRaw = parseInt(document.getElementById('swStartYear')?.value || String(defaultStartYear), 10);
    const endYearRaw = parseInt(document.getElementById('swEndYear')?.value || String(currentYear), 10);
    const startYear = Math.min(Math.max(Number.isFinite(startYearRaw) ? startYearRaw : defaultStartYear, stampworldMinYear), currentYear);
    const endYear = Math.min(Math.max(Number.isFinite(endYearRaw) ? endYearRaw : currentYear, stampworldMinYear), currentYear);
    const preset = document.getElementById('swPreset')?.value || 'basic';
    const extraFlags = (document.getElementById('swExtraFlags')?.value || '').trim();
    const status = document.getElementById('status');
    stampworldPollTick = 0;

    if (!country) {{
        if (status) status.innerText = 'StampWorld country is required.';
        return;
    }}

    updateStampWorldRunStatus('Starting StampWorld pull...', true, false);
    if (status) status.innerText = 'Starting StampWorld pull for ' + country + '...';
    stampworldRunRequested = true;

    try {{
        const r = await fetch('/api/stampworld-run', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country: country, start_year: startYear, end_year: endYear, preset: preset, extra_flags: extraFlags }})
        }});
        const j = await r.json();
        if (!j.ok) {{
            stampworldRunRequested = false;
            setBusy(false, '');
            updateStampWorldRunStatus(j.message || 'Failed to start', false, false);
            if (status) status.innerText = j.message || 'Failed to start StampWorld pull.';
            return;
        }}

        updateStampWorldRunStatus('StampWorld pull accepted. Waiting for first status update...', true, false);
        if (status) status.innerText = j.message || 'StampWorld pull started.';
        if (stampworldRunPollTimer) clearInterval(stampworldRunPollTimer);
        stampworldRunPollTimer = setInterval(pollStampWorldStatus, 1400);
        await pollStampWorldStatus();
    }} catch (e) {{
        stampworldRunRequested = false;
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

function ensureAlbumeasyPathDialog() {{
    if (document.getElementById('albumeasyPathDialog')) return;

    const dlg = document.createElement('dialog');
    dlg.id = 'albumeasyPathDialog';
    dlg.style.maxWidth = '780px';
    dlg.style.width = '94vw';
    dlg.innerHTML =
        "<div style='display:flex;flex-direction:column;gap:10px;'>" +
        "<h3 style='margin:0;'>AlbumEasy CLI Setup</h3>" +
        "<p style='margin:0;'>AlbumEasy CLI is not configured for PDF generation.</p>" +
        "<label for='albumeasyPathInput' style='font-weight:600;'>Enter full AlbumEasy CLI path now (or leave blank to skip):</label>" +
        "<div style='display:flex;gap:8px;align-items:center;flex-wrap:wrap;'>" +
        "<input id='albumeasyPathInput' type='text' placeholder='/path/to/albumeasy' style='flex:1;min-width:320px;padding:8px;border:1px solid #c8d5e2;border-radius:8px;' />" +
        "<button id='albumeasyBrowseBtn' type='button'>Browse...</button>" +
        "</div>" +
        "<div id='albumeasyPathHint' style='font-size:12px;color:#555;word-break:break-word;'></div>" +
        "<div><a id='albumeasyDownloadLink' href='https://www.thestampweb.com/albumeasy/download' target='_blank' rel='noopener noreferrer'>Download: https://www.thestampweb.com/albumeasy/download</a></div>" +
        "<div id='albumeasyPathStatus' style='font-size:12px;min-height:18px;'></div>" +
        "<div style='display:flex;gap:8px;justify-content:flex-end;'>" +
        "<button id='albumeasySkipBtn' type='button'>Skip</button>" +
        "<button id='albumeasySaveBtn' type='button'>Save Path</button>" +
        "</div>" +
        "</div>";

    document.body.appendChild(dlg);
}}

async function browseAlbumeasyPathFromDialog() {{
    const input = document.getElementById('albumeasyPathInput');
    const status = document.getElementById('albumeasyPathStatus');
    beginNativeDialog('Opening AlbumEasy CLI file picker...');
    try {{
        const r = await fetch('/api/albumeasy-pick-path', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ current_path: String(input?.value || '').trim() || null }})
        }});
        const j = await r.json();
        if (j.ok && j.picked && j.cli_path) {{
            if (input) input.value = String(j.cli_path);
            if (status) status.innerText = j.message || 'AlbumEasy path selected and saved.';
            return;
        }}
        if (status) status.innerText = j.message || (j.picked ? 'Failed to save picked path.' : 'Cancelled.');
    }} catch (e) {{
        if (status) status.innerText = 'Failed to open AlbumEasy picker: ' + e;
    }} finally {{
        endNativeDialog();
    }}
}}

async function showAlbumeasyPathDialog(statusPayload) {{
    ensureAlbumeasyPathDialog();
    const dlg = document.getElementById('albumeasyPathDialog');
    const input = document.getElementById('albumeasyPathInput');
    const hint = document.getElementById('albumeasyPathHint');
    const status = document.getElementById('albumeasyPathStatus');
    const browseBtn = document.getElementById('albumeasyBrowseBtn');
    const saveBtn = document.getElementById('albumeasySaveBtn');
    const skipBtn = document.getElementById('albumeasySkipBtn');
    const download = document.getElementById('albumeasyDownloadLink');

    if (!dlg || !input || !hint || !status || !browseBtn || !saveBtn || !skipBtn || !download) return null;

    input.value = String(statusPayload?.cli_path || '');
    hint.innerText = statusPayload?.cli_path ? ('Current path: ' + String(statusPayload.cli_path)) : '';
    status.innerText = '';
    const downloadUrl = String(statusPayload?.download_url || 'https://www.thestampweb.com/albumeasy/download');
    download.href = downloadUrl;
    download.innerText = 'Download: ' + downloadUrl;

    return await new Promise((resolve) => {{
        let settled = false;
        const settle = (value) => {{
            if (settled) return;
            settled = true;
            cleanup();
            if (dlg.open) dlg.close();
            resolve(value);
        }};

        const onBrowse = async () => {{
            await browseAlbumeasyPathFromDialog();
        }};
        const onSave = () => {{
            settle(String(input.value || '').trim());
        }};
        const onSkip = () => {{
            settle(null);
        }};
        const onCancel = () => {{
            settle(null);
        }};

        const cleanup = () => {{
            browseBtn.removeEventListener('click', onBrowse);
            saveBtn.removeEventListener('click', onSave);
            skipBtn.removeEventListener('click', onSkip);
            dlg.removeEventListener('cancel', onCancel);
            dlg.removeEventListener('close', onCancel);
        }};

        browseBtn.addEventListener('click', onBrowse);
        saveBtn.addEventListener('click', onSave);
        skipBtn.addEventListener('click', onSkip);
        dlg.addEventListener('cancel', onCancel, {{ once: true }});
        dlg.addEventListener('close', onCancel, {{ once: true }});

        if (!dlg.open) dlg.showModal();
        input.focus();
        input.select();
    }});
}}

async function saveAlbumeasyPath(path) {{
    const saveResp = await fetch('/api/albumeasy-set-path', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ cli_path: String(path).trim() }})
    }});
    return await saveResp.json();
}}

async function checkAlbumeasyStatusAfterPull() {{
    try {{
        const r = await fetch('/api/albumeasy-status');
        const j = await r.json();
        if (!j.ok) return;
        if (j.configured && j.path_exists) return;

        const answer = await showAlbumeasyPathDialog(j);
        if (answer === null || !String(answer).trim()) return;

        const saveJson = await saveAlbumeasyPath(answer);
        const status = document.getElementById('status');
        if (status) status.innerText = saveJson.message || 'AlbumEasy path updated.';
    }} catch (_) {{}}
}}

async function configureAlbumeasyPath() {{
    try {{
        const r = await fetch('/api/albumeasy-status');
        const j = await r.json();
        if (!j.ok) return;

        const answer = await showAlbumeasyPathDialog(j);
        if (answer === null || !String(answer).trim()) return;

        const saveJson = await saveAlbumeasyPath(answer);
        const status = document.getElementById('status');
        if (status) status.innerText = saveJson.message || 'AlbumEasy path updated.';
    }} catch (_) {{}}
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
        const swCountryRow = document.getElementById('swCountryRow');
        const swCountryFixedRow = document.getElementById('swCountryFixedRow');
        const dirPathEl = document.getElementById('colnectDirPath');
        const countBadge = document.getElementById('colnectCopyCountBadge');
        const copyStatus = document.getElementById('colnectCopyStatus');
        if (swCountryRow) swCountryRow.style.display = '';
        if (swCountryFixedRow) swCountryFixedRow.style.display = 'none';
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

    const swCountryRow = document.getElementById('swCountryRow');
    const swCountryFixedRow = document.getElementById('swCountryFixedRow');
    const swCountryFixed = document.getElementById('swCountryFixed');
    const swAlbumeasyRow = document.getElementById('swAlbumeasyRow');
    document.getElementById('swCountry').value = c.country;
    if (swCountryFixed) swCountryFixed.innerText = c.country;
    if (swCountryRow) swCountryRow.style.display = 'none';
    if (swCountryFixedRow) swCountryFixedRow.style.display = '';
    if (swAlbumeasyRow) swAlbumeasyRow.style.display = 'none';
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

function revealColnectImportSection() {{
    const panel = document.getElementById('colnectCopyPanel');
    if (!panel) return;
    if (!panel.open) {{
        panel.setAttribute('open', '');
        try {{ localStorage.setItem(colnectCopyPanelStorageKey, 'open'); }} catch (_) {{}}
    }}
    panel.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
    const summary = panel.querySelector('summary');
    if (summary && typeof summary.focus === 'function') {{
        summary.focus();
    }}
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
        let savedCountry = ''; try {{ savedCountry = localStorage.getItem(selectedCountryStorageKey) || ''; }} catch(_) {{}}
        sel.innerHTML = countries.map(c => `<option value="${{c.country}}">${{c.country}}</option>`).join('');
        const preferred = old || savedCountry;
        if (preferred && countries.some(c => c.country === preferred)) sel.value = preferred;
        if (!sel.value && countries.length) sel.value = countries[0].country;
        const searchInput = document.getElementById('countrySearch');
        if (searchInput && sel.value) searchInput.value = sel.value;
        document.getElementById('countryCount').innerText = `Countries current count: ${{countries.length}}`;
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
    revealColnectImportSection();
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
            body: JSON.stringify({{ country }})
        }});
        const j = await r.json();
        clearInterval(logPollTimer);
        await pollImportLog();
        setBusy(false, '');

        if (j.ok && j.stamps_imported > 0) {{
            const doneMsg = j.message || ('Imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)');
            if (status) status.innerText = doneMsg;
            if (statusBox) {{
                statusBox.className = 'colnectCopyStatus ok';
                statusBox.textContent = doneMsg;
            }}
            appendColnectCopyLog('✔ ' + doneMsg);
            await refreshState();
        }} else if (j.ok && j.stamps_imported === 0) {{
            const msg = j.message || 'No stamps were imported.';
            const finalMsg = 'No stamps imported. ' + msg;
            if (status) status.innerText = finalMsg;
            if (statusBox) {{
                statusBox.className = 'colnectCopyStatus';
                statusBox.textContent = finalMsg;
            }}
            appendColnectCopyLog(finalMsg);
            if (j.errors && j.errors.length > 0) {{
                for (const err of j.errors) appendColnectCopyLog('! ' + err);
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
    persistStampWorldYearRange();
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swEndYear').addEventListener('change', () => {{
    persistStampWorldYearRange();
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
{footer_year_update_js}
</script>"#,
        backend_not_responding_message = BACKEND_NOT_RESPONDING_MESSAGE,
        footer_year_update_js = FOOTER_YEAR_UPDATE_JS,
    )
}

pub(super) fn bootstrap_head_css() -> String {
    format!(
        r#":root {{
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
button.btn-refresh {{ background: #6a8faa; color: #fff; border-color: #557888; }}
button.btn-config {{ background: #7a68c0; color: #fff; border-color: #6554a8; }}
button.btn-pdf {{ background: #c87c28; color: #fff; border-color: #a8641a; }}
button.btn-import {{ background: #2da070; color: #fff; border-color: #228458; }}
button:hover {{ background: #edf6ff; transform: translateY(-1px); box-shadow: 0 3px 10px rgba(21, 66, 95, 0.12); }}
code {{ background: #edf5fb; border: 1px solid #d4e2ee; border-radius: 6px; padding: 2px 6px; }}
.status {{ margin-top: 10px; min-height: 18px; font-size: 13px; color: #35566d; }}
.loadingBox {{ display: none; margin-top: 12px; padding: 12px; border: 1px solid #d7e2eb; border-left: 5px solid #2f6fa3; border-radius: 10px; background: #f5fafe; }}
.spinner {{ width: 16px; height: 16px; border: 3px solid #d7e2eb; border-top-color: #2f6fa3; border-radius: 50%; display: inline-block; animation: spin 0.9s linear infinite; }}
.loadingRow {{ display: flex; align-items: center; gap: 10px; }}
.dots {{ display: inline-block; min-width: 18px; text-align: left; }}
body.busy {{ cursor: progress; }}
body.busy button, body.busy select, body.busy input {{ opacity: 0.65; }}
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
.collapsibleSection {{ overflow: hidden; }}
.collapsibleSection.panel,
.collapsibleSection.albumsRootPanel,
.collapsibleSection.colnectCopyPanel {{ padding: 0; }}
.collapsibleSection summary {{ display: flex; align-items: center; gap: 8px; padding: 10px 14px; cursor: pointer; font-weight: 600; font-size: 14px; color: var(--accent-main); list-style: none; user-select: none; transition: background 140ms; }}
.collapsibleSection summary::-webkit-details-marker {{ display: none; }}
.collapsibleSection summary:hover {{ background: rgba(31,111,168,0.07); }}
.collapsibleSection summary .introArrow {{ font-size: 10px; transition: transform 200ms; flex-shrink: 0; }}
details.collapsibleSection:not([open]) summary .introArrow {{ transform: rotate(-90deg); }}
.collapsibleSection summary .introDimTag {{ font-size: 11px; font-weight: 400; color: var(--muted-text); margin-left: auto; }}
.collapsibleSectionBody {{ padding: 12px 14px 12px 38px; }}
.collapsibleSectionBody p {{ margin: 4px 0 8px 0; }}
.hintRow {{ display: flex; gap: 10px; align-items: flex-start; margin: 7px 0; padding: 8px 10px; background: rgba(255,255,255,0.75); border-radius: 7px; border: 1px solid #ddeef9; font-size: 13px; color: var(--muted-text); }}
.hintIcon {{ font-size: 15px; flex-shrink: 0; margin-top: 1px; }}
.colnectCopyPanel {{ border: 1px solid #d9e6f1; border-left: 5px solid #7b4ea3; border-radius: 10px; padding: 12px; background: #fdf8ff; margin-top: 12px; }}
.colnectCopyLog {{ margin-top: 8px; max-height: 130px; overflow-y: auto; font-size: 11px; font-family: ui-monospace, monospace; background: #f4f0fa; border: 1px solid #d5c8ea; border-radius: 6px; padding: 5px 7px; }}
.colnectCopyStatus {{ margin-top: 8px; font-size: 13px; padding: 6px 10px; border-radius: 6px; background: #ede8f8; border: 1px solid #c9baee; min-height: 26px; }}
.colnectCopyStatus.ok {{ background: #f2fbf6; border-color: #b9dec9; color: #245641; }}
.colnectCopyStatus.fail {{ background: #fff3f3; border-color: #e8b4b4; color: #7a2020; }}
#colnectDirPath {{ display: block; max-width: 100%; white-space: pre-wrap; overflow-wrap: anywhere; word-break: break-word; }}
#colnectCopyLog .line {{ border-bottom: 1px solid #e4dff1; padding: 2px 0; white-space: pre-wrap; word-break: break-word; }}
#colnectCopyLog .line:last-child {{ border-bottom: none; }}"#
    )
}

pub(super) fn render_bootstrap_body_html(
    inbox_hint: &str,
    reconnect_banner_html: &str,
    runtime_log_panel_html: &str,
) -> String {
    format!(
        r#"<div class='card'>
    {reconnect_banner_html}
    <h1>Stamp Album Generator 2</h1>
    <h2>Start Review UI</h2>
    <p style='font-size:12px;color:#5b6f81;margin-top:4px;'>Colnect inbox hint: {inbox_hint}</p>
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
        <button class='primary' onclick='startReview()'>Open Review</button>
        <button class='btn-refresh' onclick='refreshState()'>Refresh</button>
        <button class='btn-config' onclick='openConfigEditor()'>Edit Config</button>
        <button class='btn-pdf' onclick='generateAlbumPdfNow()'>Generate PDF Now</button>
        <button class='btn-import' onclick='importColnectFiles()'>Import Colnect</button>
        <button class='danger' onclick='logoutApp()'>Logout</button>
    </div>
    <div class='row'>
        <span id='countryCount' class='badge'>Countries: 0</span>
    </div>
    <details id='albumsRootPanel' class='albumsRootPanel collapsibleSection' open>
        <summary><span class='introArrow'>&#9660;</span> Albums Root <span id='albumsRootResolved' class='badge'>Albums root: auto</span> <span class='introDimTag'>click to expand / collapse</span></summary>
        <div class='collapsibleSectionBody'>
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
    </details>
    <div id='countryPanel' class='panel'></div>
    <details id='stampworldPanel' class='panel warn collapsibleSection' style='display:none;' open>
        <summary><span class='introArrow'>&#9660;</span> Run StampWorld pull for this country (including additional pull). <span class='introDimTag'>click to expand / collapse</span></summary>
        <div class='collapsibleSectionBody'>
            <strong id='swPanelLead'>StampWorld pull is available for this country.</strong>
            <p id='swPanelHint' style='margin-top:6px;'>Use this to pull additional StampWorld data for existing countries or create a new country.</p>
            <div class='row' id='swCountryRow'>
                <label>StampWorld Country</label>
                <input id='swCountry' type='text' />
            </div>
            <div class='row' id='swCountryFixedRow' style='display:none;'>
                <label>StampWorld Country</label>
                <span id='swCountryFixed' class='badge'></span>
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
            </div>
            <div class='row'>
                <label>Extra Flags</label>
                <input id='swExtraFlags' type='text' placeholder='--max-pages 20 --stop-after-no-hits 5' style='min-width:420px;' />
            </div>
            <div class='row'>
                <button class='accent' onclick='runStampWorldPull()'>Run StampWorld Pull</button>
            </div>
            <div class='row'>
                <strong>Suggested command:</strong>
                <code id='swCmd'></code>
            </div>
            <div id='swStatus' class='status'></div>
        </div>
    </details>
    <details id='colnectCopyPanel' class='colnectCopyPanel collapsibleSection' open>
        <summary><span class='introArrow'>&#9660;</span> Import Colnect Files <span class='introDimTag'>click to expand / collapse</span></summary>
        <div class='collapsibleSectionBody'>
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
    </details>
    <div id='status' class='status'></div>
    {runtime_log_panel_html}
    <div id='loadingBox' class='loadingBox'>
        <div class='loadingRow'>
            <span class='spinner'></span>
            <div>
                <strong>Preparing Review UI<span id='loadingDotsTitle' class='dots'></span></strong><br/>
                <span id='loadingText'>Loading country data and opening the compare view. This may take a few seconds.</span>
            </div>
        </div>
    </div>
</div>"#,
        inbox_hint = inbox_hint,
        reconnect_banner_html = reconnect_banner_html,
        runtime_log_panel_html = runtime_log_panel_html,
    )
}
