let countries = [];
const stampworldCountryUrl = new StampWorldCountryUrl(() =>
    getEffectiveStampworldCountry(document.getElementById('countrySel')?.value));
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
function initCollapsiblePanel(panelId, storageKey) {
    try {
        const panel = document.getElementById(panelId);
        if (!panel) return;
        const savedState = localStorage.getItem(storageKey);
        if (savedState === 'closed') panel.removeAttribute('open');
        if (savedState === 'open') panel.setAttribute('open', '');
        panel.addEventListener('toggle', function() {
            try { localStorage.setItem(storageKey, panel.open ? 'open' : 'closed'); } catch (_) {}
        });
    } catch (_) {}
}

initCollapsiblePanel('introPanel', introPanelStorageKey);
initCollapsiblePanel('albumsRootPanel', albumsRootPanelStorageKey);
initCollapsiblePanel('stampworldPanel', stampworldPanelStorageKey);
initCollapsiblePanel('colnectCopyPanel', colnectCopyPanelStorageKey);
const backendHealthFailureThreshold = 2;

function reloadCurrentPage() {
    window.location.reload();
}

function setReconnectBanner(visible, title, text, recovered) {
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
}

async function fetchBackendHealth() {
    const controller = typeof AbortController !== 'undefined' ? new AbortController() : null;
    const timeoutId = controller ? setTimeout(() => controller.abort(), 4000) : null;
    try {
        const res = await fetch('/health', { cache: 'no-store', signal: controller ? controller.signal : undefined });
        if (!res.ok) throw new Error('HTTP ' + res.status);
        const j = await res.json();
        if (!j || j.ok !== true) throw new Error('invalid health response');
        return j;
    } finally {
        if (timeoutId) clearTimeout(timeoutId);
    }
}

async function checkBackendHealth() {
    if (nativeDialogInFlight) return;
    if (backendHealthCheckInFlight) return;
    backendHealthCheckInFlight = true;
    try {
        await fetchBackendHealth();
        backendHealthFailures = 0;
        if (backendMarkedDisconnected && !backendRecoveredReloadPending) {
            backendRecoveredReloadPending = true;
            setReconnectBanner(true, 'Connection restored', 'The backend is responding again. Reloading this page now.', true);
            setTimeout(() => window.location.reload(), 900);
        } else if (!backendRecoveredReloadPending) {
            setReconnectBanner(false, '', '', false);
        }
        backendMarkedDisconnected = false;
    } catch (_) {
        backendHealthFailures += 1;
        if (backendHealthFailures >= backendHealthFailureThreshold) {
            backendMarkedDisconnected = true;
            backendRecoveredReloadPending = false;
            setReconnectBanner(
                true,
                'Connection lost, retrying...',
                '{backend_not_responding_message}',
                false
            );
        }
    } finally {
        backendHealthCheckInFlight = false;
    }
}

function startBackendHealthPolling() {
    if (backendHealthTimer) clearInterval(backendHealthTimer);
    checkBackendHealth();
    backendHealthTimer = setInterval(checkBackendHealth, backendHealthPollMs);
    window.addEventListener('online', checkBackendHealth);
    document.addEventListener('visibilitychange', () => {
        if (!document.hidden) checkBackendHealth();
    });
}

function retryBackendHealthCheck() {
    if (nativeDialogInFlight) return;
    checkBackendHealth();
}

function beginNativeDialog(message) {
    nativeDialogInFlight = true;
    backendHealthFailures = 0;
    backendMarkedDisconnected = false;
    backendRecoveredReloadPending = false;
    setReconnectBanner(false, '', '', false);
    setAlbumsRootStatus(message || 'Waiting for native folder picker...');
    const status = document.getElementById('status');
    if (status) status.innerText = message || 'Waiting for native folder picker...';
}

function endNativeDialog() {
    nativeDialogInFlight = false;
    backendHealthFailures = 0;
    setReconnectBanner(false, '', '', false);
}

function isRuntimeLogVisible() {
    const cb = document.getElementById('showRuntimeLog');
    return cb ? !!cb.checked : true;
}

function applyRuntimeLogVisibility() {
    const body = document.getElementById('runtimeLogBody');
    if (!body) return;
    body.style.display = isRuntimeLogVisible() ? '' : 'none';
}

function toggleRuntimeLogVisibility() {
    applyRuntimeLogVisibility();
    try {
        localStorage.setItem(runtimeLogVisibleStorageKey, isRuntimeLogVisible() ? '1' : '0');
    } catch (_) {}
}

function copyTextToClipboard(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
        return navigator.clipboard.writeText(text);
    }
    return new Promise((resolve, reject) => {
        try {
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
        } catch (e) {
            reject(e);
        }
    });
}

function ensureCopyableAlertDialog() {
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
    dlg.querySelector('#copyAlertCopyBtn').addEventListener('click', async () => {
        const text = dlg.querySelector('#copyAlertText')?.value || '';
        try {
            await copyTextToClipboard(text);
        } catch (_) {}
    });
}

function showCopyableAlert(message, title) {
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
}

window.alert = function(message) {
    showCopyableAlert(message, 'Message');
};

async function refreshRuntimeLog() {
    try {
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
    } catch (_) {}
}

function startRuntimeLogPolling() {
    if (runtimeLogTimer) clearInterval(runtimeLogTimer);
    refreshRuntimeLog();
    runtimeLogTimer = setInterval(refreshRuntimeLog, 1200);
}

async function copyRuntimeLog() {
    const box = document.getElementById('runtimeLog');
    const status = document.getElementById('status');
    if (!box) return;
    try {
        await copyTextToClipboard(box.value || '');
        if (status) status.innerText = 'Runtime log copied to clipboard.';
    } catch (e) {
        if (status) status.innerText = 'Failed to copy runtime log: ' + e;
    }
}

function setCountrySearchStatus(text) {
    const status = document.getElementById('countrySearchStatus');
    if (status) status.innerText = text;
}

function updateAlbumsRootHint(path) {
    const badge = document.getElementById('albumsRootResolved');
    if (!badge) return;
    const display = String(path || '').trim();
    badge.innerText = display ? ('Albums root: ' + display) : 'Albums root: auto';
}

function setAlbumsRootStatus(text, tone) {
    const status = document.getElementById('albumsRootStatus');
    if (!status) return;
    status.className = 'albumsRootStatus';
    if (tone === 'ok') status.classList.add('ok');
    if (tone === 'fail') status.classList.add('fail');
    status.innerText = text || 'Ready';
}

async function applyAlbumsRootResponse(j) {
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
}

async function browseAlbumsRootPath() {
    const status = document.getElementById('status');
    beginNativeDialog('Opening native folder picker...');
    try {
        const r = await fetch('/bootstrap-pick-albums-root', { method: 'POST' });
        const j = await r.json();
        await applyAlbumsRootResponse(j);
    } catch (e) {
        setAlbumsRootStatus('Failed to open folder picker: ' + e, 'fail');
        if (status) status.innerText = 'Failed to open folder picker: ' + e;
    } finally {
        endNativeDialog();
    }
}

async function setAlbumsRootPath() {
    const input = document.getElementById('albumsRootPath');
    const path = String(input?.value || '').trim();

    if (!path) {
        setAlbumsRootStatus('Enter a folder path or click Pick Albums Root first.', 'fail');
        const status = document.getElementById('status');
        if (status) status.innerText = 'Enter a folder path or click Pick Albums Root first.';
        return;
    }

    const status = document.getElementById('status');
    setAlbumsRootStatus('Applying Albums root...');
    try {
        const r = await fetch('/bootstrap-set-albums-root', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ path })
        });
        const j = await r.json();
        await applyAlbumsRootResponse(j);
    } catch (e) {
        setAlbumsRootStatus('Failed to update Albums root: ' + e, 'fail');
        if (status) status.innerText = 'Failed to update Albums root: ' + e;
    }
}

async function clearAlbumsRootPath() {
    const input = document.getElementById('albumsRootPath');
    if (input) input.value = '';
    const status = document.getElementById('status');
    setAlbumsRootStatus('Switching to auto-detect...');
    try {
        const r = await fetch('/bootstrap-set-albums-root', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ path: '' })
        });
        const j = await r.json();
        await applyAlbumsRootResponse(j);
    } catch (e) {
        setAlbumsRootStatus('Failed to clear Albums root override: ' + e, 'fail');
        if (status) status.innerText = 'Failed to clear Albums root override: ' + e;
    }
}

function onCountrySelectionChanged() {
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    if (searchInput && sel) searchInput.value = sel.value || '';
    try { if (sel && sel.value) localStorage.setItem(selectedCountryStorageKey, sel.value); } catch(_) {}
    renderCountryPanel();
    updateCountrySearchMatches(true);
}

function refreshCountrySearchDatalist(query) {
    const list = document.getElementById('countrySearchList');
    if (!list) return;
    const q = String(query || '').trim().toLowerCase();
    const source = q
        ? countries.filter(c => String(c.country || '').toLowerCase().includes(q))
        : countries;
    const top = source.slice(0, 40).map(c => c.country);
    list.innerHTML = top.map(name => `<option value="${name}"></option>`).join('');
}

function applyCountrySearchSelection() {
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    const status = document.getElementById('status');
    if (!sel || !searchInput) return;

    const typed = String(searchInput.value || '').trim();
    if (!typed) return;

    const exact = countries.find(c => String(c.country || '').toLowerCase() === typed.toLowerCase());
    if (exact) {
        sel.value = exact.country;
        renderCountryPanel();
        updateCountrySearchMatches(true);
        if (status) status.innerText = `Selected ${exact.country}.`;
        return;
    }

    const swCountry = document.getElementById('swCountry');
    const swCountryFixed = document.getElementById('swCountryFixed');
    const swCountryRow = document.getElementById('swCountryRow');
    const swCountryFixedRow = document.getElementById('swCountryFixedRow');
    const swAlbumeasyRow = document.getElementById('swAlbumeasyRow');
    if (swCountry) swCountry.value = typed;
    if (swCountryFixed) {
        if ('value' in swCountryFixed) swCountryFixed.value = typed;
        else swCountryFixed.innerText = typed;
    }
    if (swCountryRow) swCountryRow.style.display = '';
    if (swCountryFixedRow) swCountryFixedRow.style.display = 'none';
    if (swAlbumeasyRow) swAlbumeasyRow.style.display = '';
    document.getElementById('stampworldPanel').style.display = '';
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(typed);
    setCountrySearchStatus('Matches: 0/0');
    if (status) status.innerText = `No exact DB match for "${typed}". You can run StampWorld pull for this typed country.`;
}

function updateCountrySearchMatches(preserveCurrentSelection) {
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    if (!sel || !searchInput) return;

    const query = String(searchInput.value || '').trim().toLowerCase();
    refreshCountrySearchDatalist(query);
    if (!query) {
        countrySearchMatches = countries.map(c => c.country);
    } else {
        countrySearchMatches = countries
            .filter(c => String(c.country || '').toLowerCase().includes(query))
            .map(c => c.country);
    }

    if (!countrySearchMatches.length) {
        countrySearchIndex = -1;
        setCountrySearchStatus('Matches: 0/0');
        return;
    }

    if (preserveCurrentSelection) {
        const selectedCountry = String(sel.value || '');
        const selectedIdx = countrySearchMatches.findIndex(c => c === selectedCountry);
        if (selectedIdx >= 0) {
            countrySearchIndex = selectedIdx;
            setCountrySearchStatus(`Matches: ${countrySearchIndex + 1}/${countrySearchMatches.length}`);
            return;
        }
    }

    countrySearchIndex = 0;
    sel.value = countrySearchMatches[0];
    renderCountryPanel();
    setCountrySearchStatus(`Matches: 1/${countrySearchMatches.length}`);
}

function stepCountrySearch(delta) {
    const sel = document.getElementById('countrySel');
    if (!sel) return;

    if (!countrySearchMatches.length) {
        setCountrySearchStatus('Matches: 0/0');
        return;
    }

    if (countrySearchIndex < 0) countrySearchIndex = 0;
    const total = countrySearchMatches.length;
    countrySearchIndex = (countrySearchIndex + delta + total) % total;
    sel.value = countrySearchMatches[countrySearchIndex];
    const searchInput = document.getElementById('countrySearch');
    if (searchInput) searchInput.value = sel.value;
    renderCountryPanel();
    setCountrySearchStatus(`Matches: ${countrySearchIndex + 1}/${total}`);
}

function startLoadingPhases(country) {
    const txt = document.getElementById('loadingText');
    if (!txt) return;

    if (loadingPhaseTimer) {
        clearInterval(loadingPhaseTimer);
        loadingPhaseTimer = null;
    }

    const phases = [
        `Preparing data from ${country}`,
        'Opening database...',
        'Syncing processed Colnect candidates...',
        'Loading left StampWorld rows...',
        'Loading right Colnect candidates...',
        'Starting local web server...',
        'Opening compare view in browser...'
    ];

    let idx = 0;
    txt.innerText = phases[idx];
    loadingPhaseTimer = setInterval(() => {
        idx = (idx + 1) % phases.length;
        txt.innerText = phases[idx];
    }, 900);
}

function stopLoadingPhases() {
    if (loadingPhaseTimer) {
        clearInterval(loadingPhaseTimer);
        loadingPhaseTimer = null;
    }
}

function setMovingDots(active) {
    const dots = document.getElementById('loadingDotsTitle');
    if (!dots) return;

    if (!active) {
        if (loadingDotsTimer) {
            clearInterval(loadingDotsTimer);
            loadingDotsTimer = null;
        }
        stopLoadingPhases();
        dots.innerText = '';
        return;
    }

    if (loadingDotsTimer) return;
    let idx = 0;
    const frames = ['', '.', '..', '...'];
    loadingDotsTimer = setInterval(() => {
        idx = (idx + 1) % frames.length;
        dots.innerText = frames[idx];
    }, 260);
}

function setBusy(isBusy, message) {
    document.body.classList.toggle('busy', isBusy);
    const box = document.getElementById('loadingBox');
    const txt = document.getElementById('loadingText');
    if (txt && message) txt.innerText = message;
    if (box) box.style.display = isBusy ? 'block' : 'none';
    setMovingDots(isBusy);
}

function slugCountry(name) {
    return String(name || '').trim().replaceAll(' ', '-');
}

function populateYearSelect(id, startYear, endYear, defaultYear) {
    const sel = document.getElementById(id);
    if (!sel) return;
    const options = [];
    for (let y = startYear; y <= endYear; y++) {
        options.push(`<option value="${y}">${y}</option>`);
    }
    sel.innerHTML = options.join('');
    const clampedDefault = Math.min(Math.max(defaultYear, startYear), endYear);
    sel.value = String(clampedDefault);
}

function initYearSelectors() {
    const minYear = stampworldMinYear;
    const maxYear = new Date().getFullYear();
    const defaultStartYear = Math.max(minYear, maxYear - 2);
    const defaultEndYear = maxYear;

    let stickyStartYear = defaultStartYear;
    let stickyEndYear = defaultEndYear;
    try {
        const storedStart = parseInt(localStorage.getItem(stampworldStartYearStorageKey) || '', 10);
        const storedEnd = parseInt(localStorage.getItem(stampworldEndYearStorageKey) || '', 10);
        if (Number.isFinite(storedStart)) stickyStartYear = Math.min(Math.max(storedStart, minYear), maxYear);
        if (Number.isFinite(storedEnd)) stickyEndYear = Math.min(Math.max(storedEnd, minYear), maxYear);
    } catch (_) {}

    populateYearSelect('swStartYear', minYear, maxYear, stickyStartYear);
    populateYearSelect('swEndYear', minYear, maxYear, stickyEndYear);
}

function persistStampWorldYearRange() {
    const maxYear = new Date().getFullYear();
    const startYear = parseInt(document.getElementById('swStartYear')?.value || '', 10);
    const endYear = parseInt(document.getElementById('swEndYear')?.value || '', 10);
    try {
        if (Number.isFinite(startYear)) localStorage.setItem(stampworldStartYearStorageKey, String(Math.min(Math.max(startYear, stampworldMinYear), maxYear)));
        if (Number.isFinite(endYear)) localStorage.setItem(stampworldEndYearStorageKey, String(Math.min(Math.max(endYear, stampworldMinYear), maxYear)));
    } catch (_) {}
}

function stampworldPresetFlags() {
    const preset = document.getElementById('swPreset')?.value || 'basic';
    if (preset === 'overwrite_album') return '--overwrite-album';
    if (preset === 'overwrite_all') return '--overwrite-album --overwrite-images';
    if (preset === 'bounded_pages') return '--max-pages 25 --stop-after-no-hits 3';
    if (preset === 'accept_typo') return '--yes';
    return '';
}

function getEffectiveStampworldCountry(country) {
    const swFixed = document.getElementById('swCountryFixed');
    const swFixedRow = document.getElementById('swCountryFixedRow');
    if (swFixedRow && swFixedRow.style.display !== 'none' && swFixed) {
        const val = ('value' in swFixed ? swFixed.value : swFixed.innerText || '').trim();
        if (val) return val;
    }
    const swRaw = document.getElementById('swCountry');
    if (swRaw && swRaw.value.trim()) {
        return swRaw.value.trim();
    }
    if (swFixed) {
        const val = ('value' in swFixed ? swFixed.value : swFixed.innerText || '').trim();
        if (val) return val;
    }
    return (country || document.getElementById('countrySel')?.value || '').trim();
}

function suggestedStampworldCommand(country) {
    const c = getEffectiveStampworldCountry(country);
    stampworldCountryUrl.load(c);
    const currentYear = new Date().getFullYear();
    const defaultStartYear = Math.max(stampworldMinYear, currentYear - 2);
    const y1 = document.getElementById('swStartYear')?.value || String(defaultStartYear);
    const y2 = document.getElementById('swEndYear')?.value || String(currentYear);
    const presetFlags = stampworldPresetFlags();
    const extraFlags = (document.getElementById('swExtraFlags')?.value || '').trim();
    const suffix = [presetFlags, extraFlags].filter(Boolean).join(' ');
    return `./stamp_album_gen2 --source stampworld --site-name "${c}" --album-name "${c}" --start-year ${y1} --end-year ${y2}${suffix ? ' ' + suffix : ''}`;
}

function updateStampWorldRunStatus(text, isRunning, canOpenReview) {
    const badge = document.getElementById('swRunStatus');
    const openBtn = document.getElementById('swOpenReviewBtn');
    if (badge) badge.innerText = text || 'Not started';
    if (openBtn) openBtn.style.display = canOpenReview ? '' : 'none';
    if (isRunning) {
        setBusy(true, 'Running StampWorld pull...');
    }
}

async function pollStampWorldStatus() {
    try {
        stampworldPollTick += 1;
        const status = document.getElementById('status');
        if (status && stampworldPollTick % 5 === 1) {
            status.innerText = `StampWorld status poll #${stampworldPollTick}...`;
        }
        const r = await fetch('/api/stampworld-status', { cache: 'no-store' });
        const j = await r.json();
        if (!j.ok) {
            const msg = j.message || 'StampWorld status poll returned not-ok payload.';
            updateStampWorldRunStatus(msg, stampworldRunRequested, false);
            if (status) status.innerText = msg;
            return;
        }

        const noJobStartedYet = !j.running && !j.finished && !j.country && !j.db_path;
        if (noJobStartedYet && !stampworldRunRequested) {
            updateStampWorldRunStatus('Ready to run', false, false);
            if (status) status.innerText = 'Ready.';
            setBusy(false, '');
            return;
        }

        if (j.running) {
            updateStampWorldRunStatus(j.message || 'StampWorld pull running...', true, false);
            
            // Fetch and display real-time log
            try {
                const logR = await fetch('/api/stampworld-log-tail', { cache: 'no-store' });
                const logJ = await logR.json();
                if (logJ.ok && logJ.lines && logJ.lines.length > 0) {
                    const logPanel = document.getElementById('swPullLogPanel');
                    const logArea = document.getElementById('swPullLog');
                    if (logPanel && logArea) {
                        logPanel.style.display = '';
                        logArea.value = logJ.lines.join('\n');
                        logArea.scrollTop = logArea.scrollHeight;
                    }
                }
            } catch (e) {
                const status = document.getElementById('status');
                const msg = 'StampWorld log poll failed: ' + e;
                if (status) status.innerText = msg;
            }
            
            return;
        }

        if (!j.finished) {
            // Transient idle state: keep polling instead of stopping immediately.
            const idleMsg = j.message || 'StampWorld status is idle; waiting for run state...';
            updateStampWorldRunStatus(idleMsg, stampworldRunRequested, false);
            if (status) status.innerText = idleMsg;
            if (stampworldRunRequested && stampworldPollTick < 20) return;
        }

        if (stampworldRunPollTimer) {
            clearInterval(stampworldRunPollTimer);
            stampworldRunPollTimer = null;
        }
        setBusy(false, '');
        
        // Hide log panel when pull finishes
        const logPanel = document.getElementById('swPullLogPanel');
        if (logPanel) logPanel.style.display = 'none';

        if (j.finished) {
            const success = j.success === true;
            stampworldRunRequested = false;
            updateStampWorldRunStatus(j.message || (success ? 'Completed' : 'Failed'), false, success);
            const status = document.getElementById('status');
            if (status) status.innerText = j.message || (success ? 'StampWorld completed.' : 'StampWorld failed.');
            if (success && j.country) {
                const sel = document.getElementById('countrySel');
                if (sel && Array.from(sel.options).some(o => o.value === j.country)) sel.value = j.country;
                renderCountryPanel();
                await checkAlbumeasyStatusAfterPull();
            }
            await refreshState();
        } else {
            const timeoutMsg = 'StampWorld status did not transition from idle. Request may not have started.';
            stampworldRunRequested = false;
            updateStampWorldRunStatus(timeoutMsg, false, false);
            if (status) status.innerText = timeoutMsg;
        }
    } catch (e) {
        const status = document.getElementById('status');
        const msg = 'StampWorld status poll failed: ' + e;
        updateStampWorldRunStatus(msg, stampworldRunRequested, false);
        if (status) status.innerText = msg;
    }
}

async function runStampWorldPull() {
    const country = getEffectiveStampworldCountry(document.getElementById('countrySel')?.value);
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

    if (!country) {
        if (status) status.innerText = 'StampWorld country is required.';
        return;
    }

    updateStampWorldRunStatus('Starting StampWorld pull...', true, false);
    if (status) status.innerText = 'Starting StampWorld pull for ' + country + '...';
    stampworldRunRequested = true;

    try {
        await stampworldCountryUrl.ensureSaved(country);
        const r = await fetch('/api/stampworld-run', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ country: country, start_year: startYear, end_year: endYear, preset: preset, extra_flags: extraFlags })
        });
        const j = await r.json();
        if (!j.ok) {
            stampworldRunRequested = false;
            setBusy(false, '');
            updateStampWorldRunStatus(j.message || 'Failed to start', false, false);
            if (status) status.innerText = j.message || 'Failed to start StampWorld pull.';
            showStampWorldStartError(j.message);
            return;
        }

        updateStampWorldRunStatus('StampWorld pull accepted. Waiting for first status update...', true, false);
        if (status) status.innerText = j.message || 'StampWorld pull started.';
        if (stampworldRunPollTimer) clearInterval(stampworldRunPollTimer);
        stampworldRunPollTimer = setInterval(pollStampWorldStatus, 1400);
        await pollStampWorldStatus();
        await checkAlbumeasyStatusAfterPull();
    } catch (e) {
        stampworldRunRequested = false;
        setBusy(false, '');
        updateStampWorldRunStatus('Failed to start', false, false);
        if (status) status.innerText = 'StampWorld request failed: ' + e;
    }
}

function openReviewAfterStampWorld() {
    const country = getEffectiveStampworldCountry(document.getElementById('countrySel')?.value);
    if (!country) return;
    const sel = document.getElementById('countrySel');
    if (sel && Array.from(sel.options).some(o => o.value === country)) {
        sel.value = country;
    }
    startReview();
}

function ensureAlbumeasyPathDialog() {
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
}

async function browseAlbumeasyPathFromDialog() {
    const input = document.getElementById('albumeasyPathInput');
    const status = document.getElementById('albumeasyPathStatus');
    beginNativeDialog('Opening AlbumEasy CLI file picker...');
    try {
        const r = await fetch('/api/albumeasy-pick-path', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ current_path: String(input?.value || '').trim() || null })
        });
        const j = await r.json();
        if (j.ok && j.picked && j.cli_path) {
            if (input) input.value = String(j.cli_path);
            if (status) status.innerText = j.message || 'AlbumEasy path selected and saved.';
            return;
        }
        if (status) status.innerText = j.message || (j.picked ? 'Failed to save picked path.' : 'Cancelled.');
    } catch (e) {
        if (status) status.innerText = 'Failed to open AlbumEasy picker: ' + e;
    } finally {
        endNativeDialog();
    }
}

async function showAlbumeasyPathDialog(statusPayload) {
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

    return await new Promise((resolve) => {
        let settled = false;
        const settle = (value) => {
            if (settled) return;
            settled = true;
            cleanup();
            if (dlg.open) dlg.close();
            resolve(value);
        };

        const onBrowse = async () => {
            await browseAlbumeasyPathFromDialog();
        };
        const onSave = () => {
            settle(String(input.value || '').trim());
        };
        const onSkip = () => {
            settle(null);
        };
        const onCancel = () => {
            settle(null);
        };

        const cleanup = () => {
            browseBtn.removeEventListener('click', onBrowse);
            saveBtn.removeEventListener('click', onSave);
            skipBtn.removeEventListener('click', onSkip);
            dlg.removeEventListener('cancel', onCancel);
            dlg.removeEventListener('close', onCancel);
        };

        browseBtn.addEventListener('click', onBrowse);
        saveBtn.addEventListener('click', onSave);
        skipBtn.addEventListener('click', onSkip);
        dlg.addEventListener('cancel', onCancel, { once: true });
        dlg.addEventListener('close', onCancel, { once: true });

        if (!dlg.open) dlg.showModal();
        input.focus();
        input.select();
    });
}

async function saveAlbumeasyPath(path) {
    const saveResp = await fetch('/api/albumeasy-set-path', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ cli_path: path })
    });
    return await saveResp.json();
}

async function checkAlbumeasyStatusAfterPull() {
    try {
        const r = await fetch('/api/albumeasy-status');
        const j = await r.json();
        if (!j.ok) return;

        const answer = await showAlbumeasyPathDialog(j);
        if (answer === null || !String(answer).trim()) return;

        const saveJson = await saveAlbumeasyPath(answer);
        const status = document.getElementById('status');
        if (status) status.innerText = saveJson.message || 'AlbumEasy path updated.';
    } catch (_) {}
}

async function copySuggestedCommand() {
    const cmd = document.getElementById('swCmd')?.innerText || '';
    const status = document.getElementById('status');
    if (!cmd.trim()) {
        if (status) status.innerText = 'No command available to copy.';
        return;
    }

    try {
        if (navigator.clipboard && navigator.clipboard.writeText) {
            await navigator.clipboard.writeText(cmd);
        } else {
            const ta = document.createElement('textarea');
            ta.value = cmd;
            ta.style.position = 'fixed';
            ta.style.left = '-9999px';
            document.body.appendChild(ta);
            ta.focus();
            ta.select();
            document.execCommand('copy');
            document.body.removeChild(ta);
        }
        if (status) status.innerText = 'Suggested command copied to clipboard.';
    } catch (e) {
        if (status) status.innerText = `Copy failed: ${e}`;
    }
}

function renderCountryPanel() {
    const sel = document.getElementById('countrySel');
    const c = countries.find(x => x.country === sel.value);
    const swLead = document.getElementById('swPanelLead');
    const swHint = document.getElementById('swPanelHint');

    if (!c) {
        document.getElementById('countryPanel').innerHTML = '<em>No countries found under Albums/*/*.db</em>';
        document.getElementById('stampworldPanel').style.display = '';
        const swCountryRow = document.getElementById('swCountryRow');
        const swCountryFixedRow = document.getElementById('swCountryFixedRow');
        const swCountryFixed = document.getElementById('swCountryFixed');
        const swCountry = document.getElementById('swCountry');
        const dirPathEl = document.getElementById('colnectDirPath');
        const countBadge = document.getElementById('colnectCopyCountBadge');
        const copyStatus = document.getElementById('colnectCopyStatus');
        if (swCountry) swCountry.value = '';
        if (swCountryFixed) {
            if ('value' in swCountryFixed) swCountryFixed.value = '';
            else swCountryFixed.innerText = '';
        }
        if (swCountryRow) swCountryRow.style.display = '';
        if (swCountryFixedRow) swCountryFixedRow.style.display = 'none';
        if (dirPathEl) dirPathEl.innerText = '(select a country)';
        if (countBadge) countBadge.innerText = 'Files: 0';
        if (copyStatus) {
            copyStatus.className = 'colnectCopyStatus';
            copyStatus.style.display = '';
            copyStatus.textContent = 'Select a country to import Colnect - User Created CSV files.';
        }
        if (swLead) swLead.innerText = 'Run StampWorld pull to create a country database.';
        if (swHint) swHint.innerText = 'Type any country name below and run StampWorld pull. This field is editable and not limited to existing countries.';
        document.getElementById('swCmd').innerText = suggestedStampworldCommand('');
        updateStampWorldRunStatus('Ready to run', false, false);
        return;
    }

    document.getElementById('countryPanel').className = c.colnect_file_count > 0 || c.colnect_candidate_count > 0 ? 'panel ok' : 'panel';
    document.getElementById('countryPanel').innerHTML =
        `<strong>${c.country}</strong><br/>` +
        `<span class='badge'>DB: ${c.db_path}</span> ` +
        `<span class='badge'>Colnect files: ${c.colnect_file_count}</span> ` +
        `<span class='badge'>DB candidates: ${c.colnect_candidate_count}</span>`;

    const swCountryRow = document.getElementById('swCountryRow');
    const swCountryFixedRow = document.getElementById('swCountryFixedRow');
    const swCountryFixed = document.getElementById('swCountryFixed');
    const swAlbumeasyRow = document.getElementById('swAlbumeasyRow');
    document.getElementById('swCountry').value = c.country;
    if (swCountryFixed) {
        if ('value' in swCountryFixed) swCountryFixed.value = c.country;
        else swCountryFixed.innerText = c.country;
    }
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

    const swDirPathEl = document.getElementById('stampworldDirPath');
    const swCountBadge = document.getElementById('stampworldCopyCountBadge');
    if (swDirPathEl) swDirPathEl.innerText = c.stampworld_dir || '(unknown)';
    if (swCountBadge) swCountBadge.innerText = 'Files: ' + (c.stampworld_file_count || 0);
}

async function pickAndCopyColnectFile() {
    const country = document.getElementById('countrySel').value;
    if (!country) { appendColnectCopyLog('Select a country first.'); return; }
    const btn = document.getElementById('colnectPickBtn');
    const statusEl = document.getElementById('colnectCopyStatus');
    const logEl = document.getElementById('colnectCopyLog');
    if (btn) btn.disabled = true;
    if (statusEl) { statusEl.className = 'colnectCopyStatus'; statusEl.style.display = ''; statusEl.textContent = 'Opening file picker\u2026'; }
    if (logEl) logEl.style.display = '';
    try {
        const r = await fetch('/api/bootstrap-pick-colnect-file', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ country })
        });
        const j = await r.json();
        if (j.ok && j.picked) {
            if (statusEl) { statusEl.className = 'colnectCopyStatus ok'; statusEl.textContent = '\u2714 Copied: ' + (j.file_name || ''); }
            appendColnectCopyLog('\u2714 ' + (j.message || 'File copied'));
            await refreshState();
        } else if (j.ok && !j.picked) {
            if (statusEl) statusEl.textContent = 'Cancelled';
        } else {
            if (statusEl) { statusEl.className = 'colnectCopyStatus fail'; statusEl.textContent = '\u2718 ' + (j.message || 'Failed'); }
            appendColnectCopyLog('\u2718 ' + (j.message || 'Failed'));
        }
    } catch (e) {
        if (statusEl) { statusEl.className = 'colnectCopyStatus fail'; statusEl.textContent = '\u2718 Error: ' + e; }
        appendColnectCopyLog('Error: ' + e);
    } finally {
        if (btn) btn.disabled = false;
    }
}

function appendColnectCopyLog(text) {
    const logEl = document.getElementById('colnectCopyLog');
    if (!logEl) return;
    const el = document.createElement('div');
    el.className = 'line';
    el.textContent = new Date().toLocaleTimeString() + ' ' + text;
    logEl.appendChild(el);
    while (logEl.children.length > 120) logEl.removeChild(logEl.firstChild);
    logEl.scrollTop = logEl.scrollHeight;
}

async function pickAndCopyStampworldPdfFile() {
    const country = document.getElementById('countrySel').value;
    if (!country) { appendStampworldCopyLog('Select a country first.'); return; }
    const btn = document.getElementById('stampworldPickBtn');
    const statusEl = document.getElementById('stampworldCopyStatus');
    const logEl = document.getElementById('stampworldCopyLog');
    if (btn) btn.disabled = true;
    if (statusEl) { statusEl.className = 'colnectCopyStatus'; statusEl.style.display = ''; statusEl.textContent = 'Opening file picker\u2026'; }
    if (logEl) logEl.style.display = '';
    try {
        const r = await fetch('/api/bootstrap-pick-stampworld-file', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ country })
        });
        const j = await r.json();
        if (j.ok && j.picked) {
            if (statusEl) { statusEl.className = 'colnectCopyStatus ok'; statusEl.textContent = '\u2714 Copied: ' + (j.file_name || ''); }
            appendStampworldCopyLog('\u2714 ' + (j.message || 'File copied'));
            await refreshState();
        } else if (j.ok && !j.picked) {
            if (statusEl) statusEl.textContent = 'Cancelled';
        } else {
            if (statusEl) { statusEl.className = 'colnectCopyStatus fail'; statusEl.textContent = '\u2718 ' + (j.message || 'Failed'); }
            appendStampworldCopyLog('\u2718 ' + (j.message || 'Failed'));
        }
    } catch (e) {
        if (statusEl) { statusEl.className = 'colnectCopyStatus fail'; statusEl.textContent = '\u2718 Error: ' + e; }
        appendStampworldCopyLog('Error: ' + e);
    } finally {
        if (btn) btn.disabled = false;
    }
}

function appendStampworldCopyLog(text) {
    const logEl = document.getElementById('stampworldCopyLog');
    if (!logEl) return;
    const el = document.createElement('div');
    el.className = 'line';
    el.textContent = new Date().toLocaleTimeString() + ' ' + text;
    logEl.appendChild(el);
    while (logEl.children.length > 120) logEl.removeChild(logEl.firstChild);
    logEl.scrollTop = logEl.scrollHeight;
}


function revealColnectImportSection() {
    const panel = document.getElementById('colnectCopyPanel');
    if (!panel) return;
    if (!panel.open) {
        panel.setAttribute('open', '');
        try { localStorage.setItem(colnectCopyPanelStorageKey, 'open'); } catch (_) {}
    }
    panel.scrollIntoView({ behavior: 'smooth', block: 'start' });
    const summary = panel.querySelector('summary');
    if (summary && typeof summary.focus === 'function') {
        summary.focus();
    }
}

async function refreshState() {
    const status = document.getElementById('status');
    status.innerText = 'Scanning Albums...';
    setBusy(false, '');
    try {
        const r = await fetch('/bootstrap-state');
        const j = await r.json();
        countries = j.countries || [];
        if (typeof externalAlbums !== 'undefined') externalAlbums.refresh();
        const albumsRoot = String(j.albums_root || '').trim();
        updateAlbumsRootHint(albumsRoot);
        const rootInput = document.getElementById('albumsRootPath');
        if (rootInput) rootInput.value = albumsRoot;
        setAlbumsRootStatus(albumsRoot ? ('Using: ' + albumsRoot) : 'Using auto-detect Albums root', 'ok');
        const sel = document.getElementById('countrySel');
        const old = sel.value;
        let savedCountry = ''; try { savedCountry = localStorage.getItem(selectedCountryStorageKey) || ''; } catch(_) {}
        sel.innerHTML = countries.map(c => `<option value="${c.country}">${c.country}</option>`).join('');
        const preferred = old || savedCountry;
        if (preferred && countries.some(c => c.country === preferred)) sel.value = preferred;
        if (!sel.value && countries.length) sel.value = countries[0].country;
        const searchInput = document.getElementById('countrySearch');
        if (searchInput && sel.value) searchInput.value = sel.value;
        document.getElementById('countryCount').innerText = `Countries current count: ${countries.length}`;
        renderCountryPanel();
        updateCountrySearchMatches(true);
        if (countries.length) {
            status.innerText = '';
        } else {
            status.innerText = albumsRoot
                ? ('No country DBs found under ' + albumsRoot + '. Use Albums Root to switch folders or run a StampWorld pull first.')
                : 'No country DBs found. The app could not auto-detect Albums; set Albums Root or run a StampWorld pull first.';
        }
    } catch (e) {
        status.innerText = `Failed to load startup state: ${e}`;
    }
}

function promptCreateCountryOrTheme() {
    const dlg = document.getElementById('createCountryDialog');
    const input = document.getElementById('newCountryNameInput');
    const status = document.getElementById('createCountryStatus');
    if (!dlg || !input) return;
    input.value = '';
    if (status) {
        status.innerText = '';
        status.style.color = '';
    }
    if (!dlg.open) dlg.showModal();
    input.focus();
}

function closeCreateCountryDialog() {
    const dlg = document.getElementById('createCountryDialog');
    if (dlg && dlg.open) dlg.close();
}

async function submitCreateCountryOrTheme() {
    const input = document.getElementById('newCountryNameInput');
    const status = document.getElementById('createCountryStatus');
    const globalStatus = document.getElementById('status');
    const name = String(input?.value || '').trim();

    if (!name) {
        if (status) {
            status.innerText = 'Please enter a country or theme name.';
            status.style.color = '#a34545';
        }
        return;
    }

    if (status) {
        status.innerText = 'Creating directory structure and database...';
        status.style.color = '#1f6fa8';
    }

    try {
        const r = await fetch('/api/create-country', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name })
        });
        const j = await r.json();
        if (!j.ok) {
            if (status) {
                status.innerText = j.message || 'Failed to create country/theme.';
                status.style.color = '#a34545';
            }
            return;
        }

        closeCreateCountryDialog();
        await refreshState();
        const sel = document.getElementById('countrySel');
        if (sel && j.country) {
            sel.value = j.country;
            onCountrySelectionChanged();
        }
        const successMsg = j.message || `Created ${name} successfully.`;
        if (globalStatus) globalStatus.innerText = successMsg;
    } catch (e) {
        if (status) {
            status.innerText = 'Request failed: ' + e;
            status.style.color = '#a34545';
        }
    }
}

async function readSiteAccountResponse(res) {
    const text = await res.text();
    if (!text.trim()) {
        throw new Error(`Site Accounts returned an empty response (HTTP ${res.status}). Restart the application with the updated binary and reload this page.`);
    }
    let data;
    try {
        data = JSON.parse(text);
    } catch (_) {
        throw new Error(`Site Accounts returned invalid JSON (HTTP ${res.status}).`);
    }
    if (!res.ok || !data || !data.ok) {
        throw new Error(data?.message || `Site Accounts request failed (HTTP ${res.status}).`);
    }
    return data;
}

async function loadAccountAuthStatus() {
    try {
        const res = await fetch('/api/account-auth-status', { cache: 'no-store' });
        const data = await readSiteAccountResponse(res);
        if (data && data.ok) {
            if (data.stampworld) {
                const swInput = document.getElementById('swUsernameInput');
                if (swInput && !swInput.value) swInput.value = data.stampworld.username || '';
                const password = document.getElementById('swPasswordInput');
                if (password) password.placeholder = data.stampworld.has_password
                    ? 'Password already saved; leave blank to keep it' : 'StampWorld password';
            }
            if (data.colnect) {
                const colInput = document.getElementById('colUsernameInput');
                if (colInput && !colInput.value) colInput.value = data.colnect.username || '';
                const password = document.getElementById('colPasswordInput');
                if (password) password.placeholder = data.colnect.has_password
                    ? 'Password already saved; leave blank to keep it' : 'Colnect password';
            }
            return data;
        }
    } catch (err) {
        const statusEl = document.getElementById('siteAccountsStatus');
        if (statusEl) {
            statusEl.style.color = 'red';
            statusEl.innerText = 'Unable to load site accounts: ' + err.message;
        }
    }
    return null;
}

function openSiteAccountsDialog(reason = '') {
    const dialog = document.getElementById('siteAccountsDialog');
    if (!dialog) return;
    const reasonEl = document.getElementById('siteAccountsReason');
    reasonEl.textContent = reason ? reason + '\nSave your credentials, then run the StampWorld pull again.' : '';
    reasonEl.hidden = !reason;
    const statusEl = document.getElementById('siteAccountsStatus');
    if (statusEl) {
        statusEl.innerText = '';
        statusEl.style.color = '';
    }
    document.getElementById('swPasswordInput').value = '';
    document.getElementById('colPasswordInput').value = '';
    loadAccountAuthStatus();
    if (typeof dialog.showModal === 'function') {
        dialog.showModal();
    } else {
        dialog.style.display = 'block';
    }
}

function closeSiteAccountsDialog() {
    const dialog = document.getElementById('siteAccountsDialog');
    if (!dialog) return;
    if (typeof dialog.close === 'function') {
        dialog.close();
    } else {
        dialog.style.display = 'none';
    }
}

async function submitSiteAccounts() {
    const swUser = (document.getElementById('swUsernameInput')?.value || '').trim();
    const swPass = document.getElementById('swPasswordInput')?.value || '';
    const colUser = (document.getElementById('colUsernameInput')?.value || '').trim();
    const colPass = document.getElementById('colPasswordInput')?.value || '';
    const statusEl = document.getElementById('siteAccountsStatus');

    if (statusEl) {
        statusEl.style.color = 'var(--accent-ink,#2b4c7e)';
        statusEl.innerText = 'Saving site account credentials...';
    }

    try {
        const res = await fetch('/api/save-site-credentials', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                stampworld_username: swUser,
                stampworld_password: swPass || null,
                colnect_username: colUser,
                colnect_password: colPass || null
            })
        });
        const data = await readSiteAccountResponse(res);
        if (data && data.ok) {
            if (statusEl) {
                statusEl.style.color = 'green';
                statusEl.innerText = data.message || 'Saved successfully!';
            }
            setTimeout(() => {
                closeSiteAccountsDialog();
            }, 800);
        } else {
            if (statusEl) {
                statusEl.style.color = 'red';
                statusEl.innerText = (data && data.message) ? data.message : 'Failed to save credentials.';
            }
        }
    } catch (err) {
        if (statusEl) {
            statusEl.style.color = 'red';
            statusEl.innerText = 'Error: ' + err.message;
        }
    }
}

async function startReview() {
    const sel = document.getElementById('countrySel');
    if (!sel.value) {
        document.getElementById('status').innerText = 'Select a country first.';
        return;
    }

    async function waitForCompareAndRedirect(maxAttempts, delayMs) {
        for (let attempt = 0; attempt < maxAttempts; attempt++) {
            await new Promise(r => setTimeout(r, delayMs));
            try {
                const rdy = await fetch('/compare-ready', { cache: 'no-store' });
                if (rdy.ok) {
                    window.location.href = '/';
                    return true;
                }
            } catch (_) {}
        }
        return false;
    }

    document.getElementById('status').innerText = `Starting Review UI for ${sel.value}...`;
    setBusy(true, `Preparing data from ${sel.value}`);
    startLoadingPhases(sel.value);
    try {
        let r = null;
        let lastStartError = null;
        for (let attempt = 0; attempt < 3; attempt++) {
            try {
                r = await fetch('/start-review', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    cache: 'no-store',
                    body: JSON.stringify({ country: sel.value })
                });
                lastStartError = null;
                break;
            } catch (e) {
                lastStartError = e;
                if (attempt < 2) await new Promise(res => setTimeout(res, 250));
            }
        }

        if (!r) {
            document.getElementById('status').innerText = 'Switching to compare view... checking server readiness.';
            document.getElementById('loadingText').innerText = 'Connection briefly dropped while switching modes. Verifying compare view...';
            const readyAfterDrop = await waitForCompareAndRedirect(24, 250);
            if (readyAfterDrop) return;
            throw lastStartError || new Error('Failed to reach /start-review');
        }

        const j = await r.json();
        if (!j.ok) {
            setBusy(false, '');
            document.getElementById('status').innerText = j.message || 'Failed to start review.';
            return;
        }
        stopLoadingPhases();
        document.getElementById('loadingText').innerText = 'Launching compare view...';
        document.getElementById('status').innerText = j.message || 'Launching review...';
        // Poll /compare-ready until the review UI server is accepting requests,
        // then redirect. This avoids a race between data loading and the redirect.
        (async function pollReady() {
            const ready = await waitForCompareAndRedirect(60, 500);
            if (!ready) window.location.href = '/';
        })();
    } catch (e) {
        setBusy(false, '');
        document.getElementById('status').innerText = `Failed to start: ${e}`;
    }
}

async function logoutApp() {
    const status = document.getElementById('status');
    if (status) status.innerText = 'Logging out and closing Review UI...';
    setBusy(true, 'Closing Review UI...');
    try {
        await fetch('/logout', { method: 'POST' });
    } catch (_) {
        // Even if the response is interrupted, continue with shutdown UX.
    }
    setBusy(false, '');
    document.body.innerHTML = "<div style='font-family:sans-serif;margin:20px;'><h2>Review UI closed</h2><p>You can close this browser tab now.</p></div>";
}

async function importColnectFiles() {
    revealColnectImportSection();
    const sel = document.getElementById('countrySel');
    const country = sel.value;
    const status = document.getElementById('status');
    const statusBox = document.getElementById('colnectCopyStatus');
    const logEl = document.getElementById('colnectCopyLog');
    
    if (!country) {
        if (status) status.innerText = 'Select a country first.';
        if (statusBox) {
            statusBox.className = 'colnectCopyStatus fail';
            statusBox.textContent = 'Select a country first.';
        }
        return;
    }

    if (status) status.innerText = 'Importing Colnect files for ' + country + '...';
    setBusy(true, 'Importing Colnect CSV files for ' + country + '...');
    if (statusBox) {
        statusBox.className = 'colnectCopyStatus';
        statusBox.textContent = 'Starting import for ' + country + '...';
    }
    appendColnectCopyLog('--- import started for ' + country + ' ---');

    let shownLogCount = 0;

    async function pollImportLog() {
        try {
            const r = await fetch('/runtime-log', { cache: 'no-store' });
            const j = await r.json();
            const lines = j.lines || [];
            const newLines = lines.slice(shownLogCount);
            shownLogCount = lines.length;
            for (const line of newLines) {
                appendColnectCopyLog(line);
                if (statusBox) {
                    statusBox.className = 'colnectCopyStatus';
                    statusBox.textContent = line;
                }
            }
        } catch (_) {}
    }

    const logPollTimer = setInterval(pollImportLog, 700);

    try {
        const r = await fetch('/api/colnect-import', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ country })
        });
        const j = await r.json();
        clearInterval(logPollTimer);
        await pollImportLog();
        setBusy(false, '');

        if (j.ok && j.stamps_imported > 0) {
            const doneMsg = j.message || ('Imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)');
            if (status) status.innerText = doneMsg;
            if (statusBox) {
                statusBox.className = 'colnectCopyStatus ok';
                statusBox.textContent = doneMsg;
            }
            appendColnectCopyLog('✔ ' + doneMsg);
            await refreshState();
        } else if (j.ok && j.stamps_imported === 0) {
            const msg = j.message || 'No stamps were imported.';
            const finalMsg = 'No stamps imported. ' + msg;
            if (status) status.innerText = finalMsg;
            if (statusBox) {
                statusBox.className = 'colnectCopyStatus';
                statusBox.textContent = finalMsg;
            }
            appendColnectCopyLog(finalMsg);
            if (j.errors && j.errors.length > 0) {
                for (const err of j.errors) appendColnectCopyLog('! ' + err);
            }
        } else {
            const msg = j.message || 'Import failed';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nErrors:\n' + j.errors.join('\n')
                : '';
            if (status) status.innerText = 'Import unavailable: ' + msg;
            if (statusBox) {
                statusBox.className = 'colnectCopyStatus fail';
                statusBox.textContent = msg;
            }
            appendColnectCopyLog('✘ ' + msg);
            if (details) appendColnectCopyLog(details.replace(/^\n+/, ''));
        }
    } catch (e) {
        clearInterval(logPollTimer);
        setBusy(false, '');
        if (status) status.innerText = 'Import error';
        if (statusBox) {
            statusBox.className = 'colnectCopyStatus fail';
            statusBox.textContent = 'Import error: ' + e;
        }
        appendColnectCopyLog('Error: ' + e);
    }
}

async function generateAlbumPdfNow() {
    const sel = document.getElementById('countrySel');
    const country = String((sel && sel.value) || '').trim();
    const status = document.getElementById('status');

    if (bootstrapPdfRequestInFlight) {
        if (status) status.innerText = 'PDF generation already in progress. Please wait.';
        return;
    }

    if (!country) {
        if (status) status.innerText = 'Select a country first.';
        return;
    }

    const startYearVal = document.getElementById('pdfStartYear')?.value.trim();
    const endYearVal   = document.getElementById('pdfEndYear')?.value.trim();
    const startYear    = startYearVal ? parseInt(startYearVal, 10) : null;
    const endYear      = endYearVal   ? parseInt(endYearVal, 10)   : null;

    const yearDesc = (startYear || endYear)
        ? ` (years ${startYear || 'start'}–${endYear || 'end'})` : '';
    if (status) status.innerText = 'Generating album text and PDF for ' + country + yearDesc + '...';
    setBusy(true, 'Generating album and PDF for ' + country + yearDesc + '...');
    bootstrapPdfRequestInFlight = true;

    try {
        const r = await fetch('/api/generate-pdf-now', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ country: country, start_year: startYear, end_year: endYear })
        });
        const j = await r.json();
        setBusy(false, '');

        if (!j.ok) {
            if (status) status.innerText = j.message || 'PDF generation failed.';
            alert('PDF generation failed.\n\n' + (j.message || 'Unknown error.'));
            return;
        }

        const outputPath = j.generated_output_path ? ('\nOutput: ' + j.generated_output_path) : '';
        const pdfPath = j.generated_pdf_path ? ('\nPDF: ' + j.generated_pdf_path) : '';
        if (status) status.innerText = j.message || 'PDF generation completed.';
        alert((j.message || 'PDF generation completed.') + outputPath + pdfPath);
    } catch (e) {
        setBusy(false, '');
        const msg = 'Generate PDF request failed: ' + e;
        if (status) status.innerText = msg;
        alert(msg);
    } finally {
        bootstrapPdfRequestInFlight = false;
    }
}

document.getElementById('swStartYear').addEventListener('change', () => {
    persistStampWorldYearRange();
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
});
document.getElementById('swEndYear').addEventListener('change', () => {
    persistStampWorldYearRange();
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
});
document.getElementById('swCountry')?.addEventListener('input', () => {
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
});
document.getElementById('swCountryFixed')?.addEventListener('input', () => {
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
});
document.getElementById('swPreset').addEventListener('change', () => {
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
});
document.getElementById('swExtraFlags').addEventListener('input', () => {
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
});

document.getElementById('albumsRootPath').addEventListener('keydown', (ev) => {
    if (ev.key === 'Enter') {
        ev.preventDefault();
        setAlbumsRootPath();
    }
});

document.getElementById('newCountryNameInput')?.addEventListener('keydown', (ev) => {
    if (ev.key === 'Enter') {
        ev.preventDefault();
        submitCreateCountryOrTheme();
    }
});

initYearSelectors();
ensureCopyableAlertDialog();
try {
    const v = localStorage.getItem(runtimeLogVisibleStorageKey);
    const cb = document.getElementById('showRuntimeLog');
    if (cb && (v === '0' || v === '1')) cb.checked = v === '1';
} catch (_) {}
applyRuntimeLogVisibility();
startRuntimeLogPolling();
startBackendHealthPolling();
refreshState();
pollStampWorldStatus();
{footer_year_update_js}
