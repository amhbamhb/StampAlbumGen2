use super::{render_config_editor_assets, MatcherPayload};

pub(super) fn render_bootstrap_html(default_inbox_path: Option<&str>) -> String {
    let inbox_hint = default_inbox_path.unwrap_or("samples/colnect_inbox");
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset='utf-8'>
<title>Stamp Matcher Setup</title>
<style>
body {{ font-family: sans-serif; margin: 20px; background: linear-gradient(180deg, #edf4f8, #f8fbfd); color: #1e3548; opacity: 0; animation: bootstrapFadeIn 180ms ease-out forwards; }}
.card {{ max-width: 960px; background: #fff; border: 1px solid #d7e2eb; border-radius: 12px; padding: 16px; box-shadow: 0 10px 24px rgba(28, 56, 82, 0.08); }}
h1 {{ margin: 0 0 8px 0; }}
p {{ margin: 0 0 10px 0; color: #395970; }}
.row {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.badge {{ display: inline-block; border: 1px solid #d2dfea; background: #f6fbff; border-radius: 999px; padding: 2px 8px; font-size: 12px; }}
.panel {{ border: 1px solid #e2e9ef; border-left: 5px solid #2f6fa3; border-radius: 10px; padding: 12px; background: #fbfdff; margin-top: 12px; }}
.warn {{ border-left-color: #9b6a20; background: #fffaf2; }}
.ok {{ border-left-color: #2f8f63; background: #f4fcf8; }}
label {{ font-size: 13px; font-weight: 600; }}
select, input {{ padding: 7px 8px; border: 1px solid #c8d5e2; border-radius: 8px; min-width: 170px; }}
button {{ padding: 7px 11px; border-radius: 8px; border: 1px solid #c8d5e2; background: #fff; cursor: pointer; }}
button.primary {{ background: #2f6fa3; color: #fff; border-color: #2f6fa3; }}
button.danger {{ background: #a34545; color: #fff; border-color: #a34545; }}
button:hover {{ background: #f1f6fa; }}
code {{ background: #eef4f8; border: 1px solid #d4e2ee; border-radius: 6px; padding: 2px 6px; }}
.status {{ margin-top: 10px; min-height: 18px; font-size: 13px; color: #35566d; }}
.loadingBox {{ display: none; margin-top: 12px; padding: 12px; border: 1px solid #d7e2eb; border-left: 5px solid #2f6fa3; border-radius: 10px; background: #f5fafe; }}
.spinner {{ width: 16px; height: 16px; border: 3px solid #d7e2eb; border-top-color: #2f6fa3; border-radius: 50%; display: inline-block; animation: spin 0.9s linear infinite; }}
.loadingRow {{ display: flex; align-items: center; gap: 10px; }}
.dots {{ display: inline-block; min-width: 18px; text-align: left; }}
body.busy {{ cursor: progress; }}
body.busy button, body.busy select, body.busy input {{ opacity: 0.65; pointer-events: none; }}
@keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@keyframes bootstrapFadeIn {{ from {{ opacity: 0; transform: translateY(2px); }} to {{ opacity: 1; transform: translateY(0); }} }}
</style>
</head>
<body>
<div class='card'>
    <h1>Start Review UI</h1>
    <p>No database was selected on startup. Pick a country to open its Review UI.</p>
    <div class='row'>
        <label for='countrySel'>Country</label>
        <select id='countrySel' onchange='onCountrySelectionChanged()'></select>
        <button onclick='refreshState()'>Refresh</button>
        <button onclick='openConfigEditor()'>Edit Config</button>
        <button class='primary' onclick='startReview()'>Open Review</button>
        <button onclick='generateAlbumPdfNow()'>Generate PDF Now</button>
        <button onclick='importColnectFiles()'>Import Colnect</button>
        <button class='danger' onclick='logoutApp()'>Logout</button>
    </div>
    <div class='row'>
        <label for='countrySearch'>Search</label>
        <input id='countrySearch' type='text' placeholder='Find country...' oninput='updateCountrySearchMatches(true)' />
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
        <strong>No Colnect files found for this country.</strong>
        <p style='margin-top:6px;'>You can run StampWorld pull first, then import Colnect CSV files and come back.</p>
        <div class='row'>
            <label>StampWorld Country</label>
            <input id='swCountry' type='text' />
            <label>Start Year</label>
            <select id='swStartYear'></select>
            <label>End Year</label>
            <select id='swEndYear'></select>
        </div>
        <div class='row'>
            <span class='badge'>Colnect inbox hint: {}</span>
        </div>
        <div class='row'>
            <strong>Suggested command:</strong>
            <code id='swCmd'></code>
            <button onclick='copySuggestedCommand()'>Copy</button>
        </div>
    </div>
    <div id='status' class='status'></div>
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

function setCountrySearchStatus(text) {{
    const status = document.getElementById('countrySearchStatus');
    if (status) status.innerText = text;
}}

function onCountrySelectionChanged() {{
    renderCountryPanel();
    updateCountrySearchMatches(true);
}}

function updateCountrySearchMatches(preserveCurrentSelection) {{
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    if (!sel || !searchInput) return;

    const query = String(searchInput.value || '').trim().toLowerCase();
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

function suggestedStampworldCommand(country) {{
    const c = document.getElementById('swCountry').value || country || '';
    const y1 = document.getElementById('swStartYear').value || '1840';
    const y2 = document.getElementById('swEndYear').value || '2026';
    return `./stamp_album_gen2 --source stampworld --site-name "${{c}}" --album-name "${{c}}" --start-year ${{y1}} --end-year ${{y2}}`;
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
    if (!c) {{
        document.getElementById('countryPanel').innerHTML = '<em>No countries found under Albums/*/*.db</em>';
        document.getElementById('stampworldPanel').style.display = '';
        document.getElementById('swCmd').innerText = suggestedStampworldCommand('');
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
    document.getElementById('stampworldPanel').style.display = (c.colnect_file_count === 0 && c.colnect_candidate_count === 0) ? '' : 'none';
}}

async function refreshState() {{
    const status = document.getElementById('status');
    status.innerText = 'Scanning Albums...';
    setBusy(false, '');
    try {{
        const r = await fetch('/bootstrap-state');
        const j = await r.json();
        countries = j.countries || [];
        const sel = document.getElementById('countrySel');
        const old = sel.value;
        sel.innerHTML = countries.map(c => `<option value="${{c.country}}">${{c.country}}</option>`).join('');
        if (old && countries.some(c => c.country === old)) sel.value = old;
        if (!sel.value && countries.length) sel.value = countries[0].country;
        document.getElementById('countryCount').innerText = `Countries: ${{countries.length}}`;
        renderCountryPanel();
        updateCountrySearchMatches(true);
        status.innerText = countries.length ? 'Select a country and open review.' : 'No country DBs found. Create one by running a StampWorld pull first.';
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
    document.getElementById('status').innerText = `Starting Review UI for ${{sel.value}}...`;
    setBusy(true, `Preparing data from ${{sel.value}}`);
    startLoadingPhases(sel.value);
    try {{
        const r = await fetch('/start-review', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country: sel.value }})
        }});
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
            for (let attempt = 0; attempt < 60; attempt++) {{
                await new Promise(r => setTimeout(r, 500));
                try {{
                    const rdy = await fetch('/compare-ready');
                    if (rdy.ok) {{
                        window.location.href = '/';
                        return;
                    }}
                }} catch (_) {{}}
            }}
            // Fallback after 30 s: redirect anyway
            window.location.href = '/';
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
    
    if (!country) {{
        if (status) status.innerText = 'Select a country first.';
        return;
    }}

    if (status) status.innerText = 'Importing Colnect files for ' + country + '...';
    setBusy(true, 'Importing Colnect CSV files for ' + country + '...');

    try {{
        const r = await fetch('/api/colnect-import', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();
        setBusy(false, '');
        if (j.ok && j.stamps_imported > 0) {{
            if (status) status.innerText = 'Imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)';
            alert('Successfully imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s).');
            await refreshState();
        }} else if (j.ok && j.stamps_imported === 0) {{
            const msg = j.message || 'No stamps were imported.';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nDetails:\n' + j.errors.join('\n')
                : '\n\nOpen the Review UI for this country and verify the configured colnect inbox path.';
            if (status) status.innerText = 'No stamps imported';
            alert(msg + details);
        }} else {{
            const msg = j.message || 'Import failed';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nErrors:\n' + j.errors.join('\n')
                : '';
            if (status) status.innerText = 'Import unavailable: ' + msg;
            alert(msg + details);
        }}
    }} catch (e) {{
        setBusy(false, '');
        if (status) status.innerText = 'Import error';
        alert('Import error: ' + e);
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

initYearSelectors();
refreshState();
</script>
{config_editor_assets}
</body>
</html>"#,
        inbox_hint,
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
<title>Stamp Matcher UI</title>
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
.sessionSection {{ border: 1px solid #d7e2eb; border-radius: 8px; background: #f9fcff; padding: 8px; }}
.sessionSection:nth-child(1) {{ background: #f2f8ff; border-color: #cddff2; }}
.sessionSection:nth-child(2) {{ background: #f2fbf6; border-color: #cfe8da; }}
.sessionSection:nth-child(3) {{ background: #fff8ef; border-color: #ead8bc; }}
.sessionSection:nth-child(4) {{ background: #f5f6f8; border-color: #d5dbe3; }}
.sessionSectionTitle {{ margin: 0 0 6px 0; font-size: 12px; font-weight: 700; color: #2a4458; letter-spacing: 0.2px; }}
.sessionRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.actionRows {{ display: flex; flex-direction: column; gap: 8px; margin-bottom: 10px; }}
.actionBar {{ border: 1px solid #dbe4ec; border-radius: 10px; padding: 9px 10px; background: #fbfdff; display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.actionBar.match {{ border-left: 5px solid var(--accent-green); }}
.actionBar.candidate {{ border-left: 5px solid var(--accent-amber); }}
.actionBarTitle {{ font-size: 12px; font-weight: 700; color: #2a4458; margin-right: 6px; }}
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
@media (max-width: 980px) {{
    .row {{ flex-direction: column; }}
    .panelBody {{ max-height: 42vh; }}
}}
</style>
</head>
<body>
<div class='wrap'>
    <div class='top'>
        <button class='neutral' onclick='returnToStartReviewPage()'>Back To Country Selection</button>
        <span class='badge'>Switch country without closing Review UI</span>
    </div>
    <div id='startGuide' class='startGuide'>
        <h2>Start Review Session</h2>
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
    <div class='controlGroups'>
        <div class='groupCard session'>
            <h4>Session</h4>
            <div class='sessionStack'>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>Configuration</div>
                    <div class='sessionRow'>
                        <button onclick='openConfigEditor()'>Edit Local Config</button>
                        <label for='defaultCatalogSelect'>Local Catalog</label>
                        <select id='defaultCatalogSelect' onchange='changeCountryDefaultCatalog()'>
                            <option value='sg'>Stanley Gibbons (SG)</option>
                            <option value='sn'>Scott (SN)</option>
                            <option value='mi'>Michel (MI)</option>
                            <option value='yt'>Yvert (YT)</option>
                            <option value='afa'>AFA</option>
                            <option value='nvp'>NVP</option>
                            <option value='un'>Unificato (UN)</option>
                            <option value='col'>Colnect (COL)</option>
                        </select>
                        <button onclick='saveCatalogDefaultToConfig()'>Update Local Config Default</button>
                    </div>
                </div>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>Import Colnect</div>
                    <div class='sessionRow'>
                        <button onclick='importColnectFilesReview()'>Import Colnect</button>
                    </div>
                </div>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>PDF</div>
                    <div class='sessionRow'>
                        <button onclick='generatePdfNowReview()'>Generate PDF Now</button>
                        <label for='reviewPdfStartYear' style='font-size:0.9em;margin:0 0 0 0.4em;'>Year</label>
                        <input id='reviewPdfStartYear' type='number' placeholder='Start' min='1840' max='2100' style='width:78px;padding:2px 4px;font-size:0.9em'/>
                        <span style='font-size:0.9em;'>–</span>
                        <input id='reviewPdfEndYear' type='number' placeholder='End' min='1840' max='2100' style='width:78px;padding:2px 4px;font-size:0.9em'/>
                        <span id='pdfStatus' class='badge'></span>
                    </div>
                </div>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>Navigation</div>
                    <div class='sessionRow'>
                        <button onclick='returnToStartReviewPage()'>Back To Country Selection</button>
                        <button class='neutral' onclick='closeReviewUi()'>Close Review UI</button>
                        <span id='status' class='badge'>Ready</span>
                    </div>
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
        <label><input type='checkbox' id='f_replace_sg'/> replace SG number</label>
        <label><input type='checkbox' id='f_right_unmatched_only' onchange='render()'/> right unmatched only</label>
        <label><input type='checkbox' id='f_show_imageless_left' onchange='toggleImagelessLeftRows()'/> show left rows without images</label>
                <label>order
                        <select id='sortMode' onchange='changeSortMode()'>
                                <option value='sg'>SG number (default)</option>
                                <option value='year_sg'>Year then SG</option>
                                <option value='series_sg'>Series then SG</option>
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
                        <label>SG / Ref Code <input id='leftEditorSg' type='text' /></label>
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
                        <label>Ord SG <input id='leftEditorColnectOrdSg' type='text' /></label>
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
window.__returnToStartInFlight = false;

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
function catalogDisplayName(code) {{
    const c = String(code || '').trim().toLowerCase();
    if (c === 'sn') return 'Scott';
    if (c === 'sg') return 'Stanley Gibbons';
    if (c === 'mi') return 'Michel';
    if (c === 'yt') return 'Yvert';
    if (c === 'afa') return 'AFA';
    if (c === 'nvp') return 'NVP';
    if (c === 'un') return 'Unificato';
    if (c === 'col') return 'Colnect';
    return c ? c.toUpperCase() : 'Catalog';
}}

function getEffectiveDefaultCatalogCode() {{
    return String(countryDefaultCatalogCode || configDefaultCatalogCode || 'sg').toLowerCase();
}}

function changeCountryDefaultCatalog() {{
    const sel = document.getElementById('defaultCatalogSelect');
    if (!sel) return;
    countryDefaultCatalogCode = String(sel.value || 'sg').toLowerCase();
    try {{
        localStorage.setItem(countryCatalogStorageKey, countryDefaultCatalogCode);
    }} catch (_) {{}}
    render();
}}

async function saveCatalogDefaultToConfig() {{
    const sel = document.getElementById('defaultCatalogSelect');
    const statusEl = document.getElementById('status');
    const code = String(sel?.value || '').toLowerCase();
    if (!code) {{
        alert('Choose a catalog first.');
        return;
    }}

    if (statusEl) statusEl.innerText = 'Updating config default catalog...';
    try {{
        const r = await fetch('/api/default-catalog-config', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ default_catalog_code: code }})
        }});
        const j = await r.json();
        if (j.ok) {{
            if (statusEl) statusEl.innerText = 'Config default updated';
            alert(j.message || 'Config updated.');
        }} else {{
            if (statusEl) statusEl.innerText = 'Config update failed';
            alert(j.message || 'Failed to update config default.');
        }}
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Config update error';
        alert('Config update error: ' + e);
    }}
}}
function normalizeCatalogNumber(v) {{
    let s = String(v || '').trim();
    if (!s) return '';
    const colonIdx = s.indexOf(':');
    if (colonIdx >= 0) s = s.slice(colonIdx + 1).trim();
    const m = s.match(/^([A-Za-z]+)\s+(.+)$/);
    if (m && m[2]) return String(m[2]).trim();
    return s;
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
    if (leftIdx !== before) render();
}}

function stepLeftSearch(delta) {{
    updateLeftSearchMatches(true);
    if (!leftSearchMatches.length) return;

    const total = leftSearchMatches.length;
    if (leftSearchMatchPos < 0) leftSearchMatchPos = 0;
    leftSearchMatchPos = (leftSearchMatchPos + delta + total) % total;
    leftIdx = leftSearchMatches[leftSearchMatchPos];
    setLeftSearchStatus(`Matches: ${{leftSearchMatchPos + 1}}/${{total}}`);
    render();
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
    render();
}}

function firstLeft() {{
    if (!left.length) return;
    leftIdx = 0;
    render();
}}

function lastLeft() {{
    if (!left.length) return;
    leftIdx = left.length - 1;
    render();
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

function render() {{
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
    return;
  }}

  const seriesEl = document.getElementById('f_series');
  if (seriesEl && !seriesEl.dataset.userTouched) {{
    seriesEl.checked = checkboxDefaults.f_series;
  }}

  const l = left[leftIdx];
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
            <input id='leftSearchInput' type='text' placeholder='SG, series, name...' oninput='onLeftSearchInput()' value='${{html(leftSearchQuery)}}'/>
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
            <input id='rightSearchInput' type='text' placeholder='ID, SG, name, series...' oninput='onRightSearchInput()' value='${{html(rightSearchQuery)}}'/>
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
                <strong>Base Stamp</strong> SG=${{html(fmt(l.sg_number))}} | Year=${{html(fmt(l.year))}} | Series=${{html(fmt(l.series))}}
            </div>
            <div class='panelBody'>
                ${{imageBox(leftImg, 'StampWorld image')}}
                <table class='kv'>
                    ${{row('SG', l.sg_number)}}
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
    return;
  }}

  const key = String(l.rowid);
  document.getElementById('rightPos').innerText = `Candidate ${{(rightIdxByLeft[key] || 0) + 1}}/${{avail.length}}`;
        updateRightSearchStatus();
    ensureRightImageUrl(r);
  const rightImg = b(r.image_abs) ? `/image?path=${{encodeURIComponent(r.image_abs)}}` : r.image_url;
    const catPairs = (r.catalog_code_pairs || []).map(p =>
        `<option value='${{html(p.code)}}' data-cat='${{html(p.catalog_type)}}'>${{html(catalogDisplayName(p.catalog_type))}} (${{html(String(p.catalog_type || '').toUpperCase())}}): ${{html(p.code)}}</option>`
    ).join('');
    const selectedCode = selectedCatalogCodeForCurrent(r);
  document.getElementById('rightPanel').innerHTML = `
        <div class='panelCard'>
            <div class='panelHead'>
               
                <div style='margin-top:8px;'>
                    <label>Catalog Code (for optional SG replace):</label>
                    <select id='catalogCodeSelect' onchange='rememberCatalogCode()'>
                        <option value=''>-- none --</option>
                        ${{catPairs}}
                    </select>
                </div>
            </div>
            <div class='panelBody'>
                ${{imageBox(rightImg, 'Colnect candidate image')}}
                <table class='kv'>
                    ${{row('Colnect ID', r.colnect_id)}}
                    ${{row('SG', r.sg_number)}}
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
            ${{row('ORD SG', r.ord_sg)}}
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
    if (sel && selectedCode) sel.value = selectedCode;
}}

function rememberCatalogCode() {{
    const l = left[leftIdx];
    if (!l) return;
    const sel = document.getElementById('catalogCodeSelect');
    if (!sel) return;
    selectedCatalogCodeByLeft[l.rowid] = sel.value || '';
    const selectedOpt = sel.options[sel.selectedIndex];
    selectedCatalogTypeByLeft[l.rowid] = selectedOpt?.dataset?.cat || '';
}}

function prevLeft() {{
  if (!left.length) return;
  leftIdx = (leftIdx - 1 + left.length) % left.length;
  render();
}}
function nextLeft() {{
  if (!left.length) return;
  leftIdx = (leftIdx + 1) % left.length;
  render();
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
  document.getElementById('status').innerText = `Assigned rowid=${{l.rowid}} -> colnect_id=${{r.colnect_id}}`;
  render();
}}
function clearAssignment() {{
  const l = left[leftIdx];
  if (!l) return;
  delete assignedByLeft[l.rowid];
    delete selectedCatalogCodeByLeft[l.rowid];
    delete selectedCatalogTypeByLeft[l.rowid];
  document.getElementById('status').innerText = `Cleared assignment for rowid=${{l.rowid}}`;
  render();
}}

async function applyMatches() {{
    const matches = Object.entries(assignedByLeft).map(([leftRowid, colnectId]) => {{
        const candidateMap = {{}};
        rightAll.forEach(c => {{ candidateMap[c.colnect_id] = c; }});
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
        document.getElementById('status').innerText = 'No assigned matches selected. Use Assign Candidate first.';
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
    for (const colnectId of (j.processed_colnect_ids || [])) {{
        removeCandidateFromUi(colnectId);
    }}
  document.getElementById('status').innerText = `Applied: updated=${{j.updated_rows}}, skipped=${{j.skipped_rows}}, errors=${{(j.errors||[]).length}}`;
  if ((j.errors || []).length) {{
    alert(j.errors.join('\n'));
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
        document.getElementById('status').innerText = 'No current candidate to add';
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

    document.getElementById('leftEditorSg').value = base?.sg_number || '';
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
        alert('Enter an SG / reference code before saving.');
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
        document.getElementById('status').innerText = 'No current candidate to add';
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
    document.getElementById('status').innerText = j.message || 'Added to stamps';
    render();
}}

async function deleteCurrentCandidate() {{
    const r = currentRightCandidate();
    if (!r) {{
        document.getElementById('status').innerText = 'No current candidate to delete';
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
    document.getElementById('status').innerText = j.message || 'Soft-deleted candidate';
    render();
}}

async function undoLastDelete() {{
    const last = deletedCandidatesStack.pop();
    if (!last) {{
        document.getElementById('status').innerText = 'Nothing to undo';
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
    document.getElementById('status').innerText = j.message || 'Undo complete';
    render();
}}

async function closeReviewUi() {{
    // Show a 3-button modal: Generate & Close | Close Only | Cancel
    const choice = await new Promise(resolve => {{
        const defaultPdfChecked = pdfGenerationDefaults.enabledByDefault && pdfGenerationDefaults.cliConfigured;
        const pdfOptionHtml =
            '<label style="display:flex;align-items:flex-start;gap:0.65em;margin:0 0 1.1em;text-align:left;font-size:0.95em;line-height:1.4">' +
            `<input id="_dlg_pdf" type="checkbox" ${{defaultPdfChecked ? 'checked' : ''}} ${{pdfGenerationDefaults.cliConfigured ? '' : 'disabled'}} style="margin-top:0.2em" />` +
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
            const generatePdf = v === 'generate' && !!dlg.querySelector('#_dlg_pdf')?.checked;
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

    const j = await res.json();
    const details = [
        j.generated_output_path ? `\nOutput: ${{j.generated_output_path}}` : '',
        j.generated_pdf_path ? `\nPDF: ${{j.generated_pdf_path}}` : ''
    ].join('');
    alert((j.message || 'Review UI shutdown request sent.') + details);

    // window.close may be blocked for tabs not opened by script; provide fallback.
    window.close();
    setTimeout(() => {{ window.location.href = 'about:blank'; }}, 150);
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
    const statusEl = document.getElementById('pdfStatus');
    const originalText = statusEl?.innerText || '';
    if (window.__reviewPdfRequestInFlight === true) {{
        if (statusEl) statusEl.innerText = 'PDF generation already in progress. Please wait.';
        return;
    }}

    const startYearVal = document.getElementById('reviewPdfStartYear')?.value.trim();
    const endYearVal   = document.getElementById('reviewPdfEndYear')?.value.trim();
    const startYear    = startYearVal ? parseInt(startYearVal, 10) : null;
    const endYear      = endYearVal   ? parseInt(endYearVal, 10)   : null;
    const yearDesc     = (startYear || endYear) ? ` (years ${{startYear || 'start'}}–${{endYear || 'end'}})` : '';
    if (statusEl) statusEl.innerText = 'Generating album text and PDF' + yearDesc + '...';
    window.__reviewPdfRequestInFlight = true;

    try {{
        const res = await fetch('/generate-pdf-now', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ start_year: startYear, end_year: endYear }})
        }});
        const j = await res.json();
        if (!j.ok) {{
            if (statusEl) statusEl.innerText = 'PDF generation failed';
            alert(j.message || 'PDF generation failed.');
            return;
        }}

        const details = [
            j.generated_output_path ? `\nOutput: ${{j.generated_output_path}}` : '',
            j.generated_pdf_path ? `\nPDF: ${{j.generated_pdf_path}}` : ''
        ].join('');
        if (statusEl) statusEl.innerText = j.message || 'PDF generation completed';
        alert((j.message || 'PDF generation completed.') + details);
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'PDF generation request failed';
        alert('PDF generation request failed: ' + e);
    }} finally {{
        window.__reviewPdfRequestInFlight = false;
        setTimeout(() => {{
            if (statusEl) statusEl.innerText = originalText;
        }}, 1600);
    }}
}}

async function importColnectFilesReview() {{
    const statusEl = document.getElementById('status');
    const originalText = statusEl?.innerText || 'Ready';
    
    if (statusEl) statusEl.innerText = 'Importing Colnect files...';
    
    try {{
        const r = await fetch('/api/colnect-import', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();
            if (j.ok && j.stamps_imported > 0) {{
                if (statusEl) statusEl.innerText = 'Imported ' + j.stamps_imported + ' stamps';
                alert('Successfully imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)!\n\nRefreshing...');
                setTimeout(() => {{
                    location.reload();
                }}, 500);
            }} else if (j.ok && j.stamps_imported === 0) {{
                if (statusEl) statusEl.innerText = 'No stamps imported';
                const msg = j.message || 'No stamps were imported.';
                const details = (j.errors && j.errors.length > 0)
                    ? '\n\nCheck inbox path and CSV files:\n' + j.errors.join('\n')
                    : '\n\nCheck that CSV files exist in the configured colnect_inbox_path in config.toml.';
                alert(msg + details);
            }} else {{
                if (statusEl) statusEl.innerText = 'Import failed';
                const msg = j.message || 'Import failed';
                const details = (j.errors && j.errors.length > 0)
                    ? '\n\nErrors:\n' + j.errors.join('\n')
                    : '';
                alert(msg + details);
            }}
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Import error';
        alert('Import error: ' + e);
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
    countryDefaultCatalogCode = storedCatalog || configDefaultCatalogCode;
}} catch (_) {{
    countryDefaultCatalogCode = configDefaultCatalogCode;
}}

const defaultCatalogSelect = document.getElementById('defaultCatalogSelect');
if (defaultCatalogSelect) {{
    defaultCatalogSelect.value = countryDefaultCatalogCode || 'sg';
}}

const sortModeSelect = document.getElementById('sortMode');
if (sortModeSelect) sortModeSelect.value = sortMode;
const showImagelessLeftToggle = document.getElementById('f_show_imageless_left');
if (showImagelessLeftToggle) showImagelessLeftToggle.checked = showImagelessLeftRowsDefault;
sortLeftRows(left[leftIdx]?.rowid);

render();
</script>
{config_editor_assets}
</body>
</html>"#,
        json = json,
        config_editor_assets = render_config_editor_assets()
    )
}
