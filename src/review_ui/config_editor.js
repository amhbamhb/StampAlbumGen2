/*
 * Live JavaScript for the Review UI configuration editor.
 *
 * Formatting guidance:
 * - Keep one statement or logical operation per line.
 * - Use four-space indentation inside functions and conditionals.
 * - Keep DOM lookups near the code that uses them.
 * - Keep user-visible text in template literals when it contains a value.
 * - This file is embedded by config_editor.rs; do not add <script> tags here.
 */

const configEditorState = { open: false, loading: false, saving: false, sections: [], originalValues: {}, albumName: '', hasAlbumOverride: false, showOverridesOnly: false, saveTarget: 'global' };
function cfgHtml(v) { return String(v).replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('"', '&quot;').replaceAll("'", '&#39;'); }
function stripTomlQuotes(v) { const s = String(v ?? '').trim(); if ((s.startsWith('"') && s.endsWith('"')) || (s.startsWith("'") && s.endsWith("'"))) { return s.slice(1, -1); } return s; }
function configEditorBackdrop() { return document.getElementById('configEditorBackdrop'); }
function configEditorSectionsHost() { return document.getElementById('cfgEditorSections'); }
function configEditorLoadState() { return document.getElementById('cfgEditorLoadState'); }
function configEditorStatusBadge() { return document.getElementById('cfgEditorStatus'); }
function configEditorFieldCountBadge() { return document.getElementById('cfgEditorFieldCount'); }
function configEditorSearchInput() { return document.getElementById('cfgSearchInput'); }
function configEditorTitle() { return document.getElementById('cfgEditorTitle'); }
function configEditorAlbumBanner() { return document.getElementById('cfgAlbumBanner'); }
function configEditorSaveTargetBlock() { return document.getElementById('cfgSaveTargetBlock'); }
function configEditorSaveTargetSelect() { return document.getElementById('cfgSaveTarget'); }
function configEditorDeleteAlbumBtn() { return document.getElementById('cfgDeleteAlbumBtn'); }
function setConfigEditorStatus(message) {
    const badge = configEditorStatusBadge();
    if (badge) badge.innerText = message;
    if (typeof window.setConfigStatus === 'function') {
        window.setConfigStatus(message);
        return;
    }
    const statusEl = document.getElementById('status');
    if (statusEl) statusEl.innerText = message;
}
function onConfigSaveTargetChanged() {
    const el = configEditorSaveTargetSelect();
    if (!el) return; configEditorState.saveTarget = el.value === 'album' ? 'album' : 'global';
    const targetName = configEditorState.saveTarget === 'album' ?
        `Album override (${configEditorState.albumName || 'current'})` : 'global config.toml';
    setConfigEditorStatus(`Save target: ${targetName}`);
}
// Keep the editor title, album status, save target, and delete action consistent
// whenever the editor switches between global and album-specific configuration.
function refreshConfigEditorModeUi() {
    const albumName = String(configEditorState.albumName || '').trim();
    const inAlbumMode = albumName.length > 0;

    const titleEl = configEditorTitle();
    if (titleEl) {
        titleEl.innerText = inAlbumMode
            ? `Edit config for ${albumName}`
            : 'Edit config.toml';
    }

    const banner = configEditorAlbumBanner();
    if (banner) {
        if (inAlbumMode) {
            banner.style.display = 'inline-flex';
            const canFilterOverrides = configEditorState.hasAlbumOverride;
            banner.disabled = !canFilterOverrides;
            banner.classList.toggle(
                'cfgAlbumBannerActive',
                canFilterOverrides && configEditorState.showOverridesOnly
            );
            banner.setAttribute(
                'aria-pressed',
                canFilterOverrides && configEditorState.showOverridesOnly ? 'true' : 'false'
            );
            banner.innerText = canFilterOverrides
                ? `Album override active: ${albumName}${configEditorState.showOverridesOnly ? ' (showing overrides)' : ''}`
                : `Using global config for ${albumName}`;
        } else {
            banner.style.display = 'none';
            banner.disabled = true;
            banner.classList.remove('cfgAlbumBannerActive');
            banner.setAttribute('aria-pressed', 'false');
            banner.innerText = '';
        }
    }

    const saveTargetBlock = configEditorSaveTargetBlock();
    if (saveTargetBlock) {
        saveTargetBlock.style.display = inAlbumMode ? 'inline-flex' : 'none';
    }

    const saveTargetSelect = configEditorSaveTargetSelect();
    if (saveTargetSelect) {
        if (!inAlbumMode) {
            configEditorState.saveTarget = 'global';
            saveTargetSelect.value = 'global';
        } else if (
            configEditorState.saveTarget !== 'global' &&
            configEditorState.saveTarget !== 'album'
        ) {
            configEditorState.saveTarget = 'album';
            saveTargetSelect.value = 'album';
        } else {
            saveTargetSelect.value = configEditorState.saveTarget;
        }

        const albumOption = saveTargetSelect.querySelector(
            "option[value='album']"
        );
        if (albumOption) {
            albumOption.textContent = `Album override - ${albumName || 'current'}`;
        }
    }

    const deleteBtn = configEditorDeleteAlbumBtn();
    if (deleteBtn) {
        deleteBtn.classList.toggle(
            'cfgHidden',
            !(inAlbumMode && configEditorState.hasAlbumOverride)
        );
    }
}
function configFieldInputId(fieldId) { return 'cfg-input-' + String(fieldId || '').replaceAll('.', '__').replaceAll('/', '_'); }
function isBooleanValue(v) { const s = String(v ?? '').trim().toLowerCase(); return s === 'true' || s === 'false'; }
function isNumericValue(v) { return /^-?\d+(\.\d+)?$/.test(String(v ?? '').trim()); }
function isFontField(field) { const key = String(field?.key || '').toLowerCase(); const id = String(field?.id || '').toLowerCase(); return key.endsWith('_font') || id.endsWith('_font'); }
function configFieldKnownOptions(fieldId, key) {
    const id = String(fieldId || '').toLowerCase();
    const k = String(key || '').toLowerCase();
    if (id === 'stamps.stamp_detail_layout' || k === 'stamp_detail_layout') return ['inline', 'separate_rows', 'minimal'];
    if (id === 'header.album_pages_header_position' || id === 'footer.album_pages_footer_number_position' || k.endsWith('_position')) return ['L', 'R', 'C', 'LR', 'RL'];
    if (id === 'data_sources.expand_grouped_ranges' || k === 'expand_grouped_ranges') return ['placeholders', 'detail_fetch'];
    if (id === 'wikipedia.wikipedia_paragraph_alignment' || k.endsWith('_alignment')) return ['JUSTIFIED', 'LEFT', 'CENTER', 'RIGHT'];
    if (id === 'wikipedia.wikipedia_pagination_strategy' || k.endsWith('_pagination_strategy')) return ['balanced', 'conservative', 'strict'];
    return null;
}
function configFieldType(field) {
    const current = String(field?.current_value ?? '').trim();
    const template = String(field?.template_value ?? '').trim();
    if (isBooleanValue(current) || isBooleanValue(template)) return 'bool';
    if (isFontField(field)) return 'font';
    if (configFieldKnownOptions(field?.id, field?.key)) return 'enum';
    if (isNumericValue(current) || isNumericValue(template)) return 'number';
    return 'text';
}
function configFieldHelpHtml(field) {
    const comments = Array.isArray(field?.comments) ? field.comments : [];
    const notes = [];
    if (comments.length) notes.push(...comments);
    if (isFontField(field)) notes.push('AlbumEasy font codes often used: HN, HB, TN, TB, TR, TI, BI, CN, CO.');
    const type = configFieldType(field);
    if (type === 'bool') notes.push('Boolean field: choose true or false.');
    if (type === 'number') notes.push('Numeric field: enter a valid integer or decimal number.');
    if (!notes.length) return '';
    const text = notes.map(line => cfgHtml(line)).join('\n');
    return `<details class='cfgInfoDetail'><summary>Help</summary><div class='cfgInfoBody'>${text}</div></details>`;
}
function renderConfigValueInput(field) {
    const fieldId = String(field?.id || '');
    const inputId = configFieldInputId(fieldId);
    const current = String(field?.current_value ?? '');
    const type = configFieldType(field);
    const knownOptions = configFieldKnownOptions(field?.id, field?.key);
    const fontOptions = ['HN', 'HB', 'TN', 'TB', 'TR', 'TI', 'BI', 'CN', 'CO'];
    const normBool = String(current).trim().toLowerCase() === 'true' ? 'true' : 'false';

    if (type === 'bool') {
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'><option value='true' ${normBool === 'true' ? 'selected' : ''}>true</option><option value='false' ${normBool === 'false' ? 'selected' : ''}>false</option></select><div class='cfgHelpText'>Type: boolean</div>`;
    }

    if (type === 'font') {
        const rawFont = stripTomlQuotes(current);
        const options = fontOptions.map(opt => `<option value='${cfgHtml(opt)}' ${rawFont === opt ? 'selected' : ''}>${cfgHtml(opt)}</option>`).join('');
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'>${options}</select><div class='cfgHelpText'>Type: font code</div>`;
    }

    if (type === 'enum' && Array.isArray(knownOptions) && knownOptions.length) {
        const rawEnum = stripTomlQuotes(current);
        const fallbackOption = knownOptions.includes(rawEnum) ? '' : `<option value='${cfgHtml(rawEnum)}' selected>${cfgHtml(rawEnum || '(empty)')}</option>`;
        const options = knownOptions.map(opt => `<option value='${cfgHtml(opt)}' ${rawEnum === opt ? 'selected' : ''}>${cfgHtml(opt)}</option>`).join('');
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'>${fallbackOption}${options}</select><div class='cfgHelpText'>Type: option list</div>`;
    }

    if (type === 'number') {
        return `<input id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' type='number' step='any' value='${cfgHtml(current)}' oninput='markConfigEditorDirty()'/><div class='cfgHelpText'>Type: number</div>`;
    }

    return `<input id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' type='text' value='${cfgHtml(current)}' oninput='markConfigEditorDirty()'/>`;
}
function showConfigEditorLoading(message) { const loadState = configEditorLoadState(); const host = configEditorSectionsHost(); if (host) host.innerHTML = ''; if (loadState) { loadState.innerText = message; loadState.classList.remove('cfgHidden'); } }
function hideConfigEditorLoading() { const loadState = configEditorLoadState(); if (loadState) loadState.classList.add('cfgHidden'); }
function renderConfigEditorSections() {
    const host = configEditorSectionsHost();
    if (!host) return;

    const rawSections = Array.isArray(configEditorState.sections) ? configEditorState.sections : [];
    const startupFieldKeys = new Set(['albumeasy_cli_path']);
    const startupFields = [];
    const normalizedSections = [];

    rawSections.forEach(section => {
        const fields = Array.isArray(section?.fields) ? section.fields : [];
        const keepFields = [];
        fields.forEach(field => {
            const key = String(field?.key || '').trim();
            const id = String(field?.id || '').trim();
            const idTail = id.includes('.') ? id.split('.').pop() : id;
            if (startupFieldKeys.has(key) || startupFieldKeys.has(idTail)) {
                startupFields.push({ ...field, __startup_section_title: 'Startup Quick Settings' });
            } else {
                keepFields.push(field);
            }
        });
        if (keepFields.length) normalizedSections.push({ ...section, fields: keepFields });
    });

    const sections = [];
    if (startupFields.length) {
        sections.push({
            id: 'startup.quick',
            title: 'Startup Quick Settings',
            comments: ['Frequently-used startup values. This section is expanded by default.'],
            fields: startupFields,
            __defaultOpen: true,
        });
    }
    sections.push(...normalizedSections);

    let fieldCount = 0;
    host.innerHTML = sections
        .map(section => {
            const fields = Array.isArray(section.fields) ? section.fields : [];
            fieldCount += fields.length;
            const sectionId = cfgHtml(section.id || section.title || 'section');
            const defaultOpen = !!section.__defaultOpen;
            const collapsedClass = defaultOpen ? '' : ' collapsed';
            const sectionComments =
                Array.isArray(section.comments) && section.comments.length
                    ? `<p>${section.comments.map(line => cfgHtml(line)).join(' ')}</p>`
                    : '';
            const rows = fields
                .map(field => {
                    const searchCorpus = [
                        section.title || '',
                        field.key || '',
                        field.id || '',
                        ...(Array.isArray(field.comments) ? field.comments : []),
                    ]
                        .join(' ')
                        .toLowerCase();
                    const fieldType = configFieldType(field);
                    const albumBadge = field?.is_album_override
                        ? "<span class='badge cfgAlbumBadge'>album override</span>"
                        : '';
                    return `<div class='cfgFieldRow' data-album-override='${field?.is_album_override ? '1' : '0'}' data-config-search='${cfgHtml(searchCorpus)}'><div class='cfgStaticPane'><div class='cfgKeyRow'><strong>${cfgHtml(field.key || '')}</strong><span class='badge'>${cfgHtml(field.from_template === false ? 'Preserved key' : 'Template key')}</span>${albumBadge}<span class='cfgTypeTag'>${cfgHtml(fieldType)}</span></div><div class='cfgTemplateBox'><div>Template value</div><code>${cfgHtml(field.template_value || '')}</code></div>${configFieldHelpHtml(field)}</div><div class='cfgEditPane'><label class='cfgValueLabel' for='${cfgHtml(configFieldInputId(field.id))}'>Current value</label>${renderConfigValueInput(field)}</div></div>`;
                })
                .join('');
            return `<section class='cfgSection${collapsedClass}' data-section-id='${sectionId}' data-default-open='${defaultOpen ? '1' : '0'}'><div class='cfgSectionHead' onclick='toggleConfigSection(this.closest(".cfgSection"))'><div style='display:flex;align-items:center;gap:8px;'><button class='cfgSectionToggle' tabindex='-1' aria-label='Toggle section'>▼</button><div><h4>${cfgHtml(section.title || 'Section')}</h4>${sectionComments}</div></div><span class='badge'>Fields: ${fields.length}</span></div><div class='cfgSectionBody'>${rows}</div></section>`;
        })
        .join('');

    const badge = configEditorFieldCountBadge();
    if (badge) badge.innerText = `Fields: ${fieldCount}`;
    filterConfigEditorFields();
}
function toggleConfigSection(sectionEl) { if (!sectionEl) return; sectionEl.classList.toggle('collapsed'); }
function collectConfigEditorValues() { const values = {}; document.querySelectorAll('.cfgValueInput[data-field-id]').forEach(el => { values[el.getAttribute('data-field-id')] = String(el.value ?? ''); }); return values; }
function configEditorIsDirty() { const currentValues = collectConfigEditorValues(); const originalValues = configEditorState.originalValues || {}; const currentKeys = Object.keys(currentValues).sort(); const originalKeys = Object.keys(originalValues).sort(); if (currentKeys.length !== originalKeys.length) return true; for (let idx = 0; idx < currentKeys.length; idx += 1) { if (currentKeys[idx] !== originalKeys[idx]) return true; if (String(currentValues[currentKeys[idx]]) !== String(originalValues[currentKeys[idx]])) return true; } return false; }
function markConfigEditorDirty() { if (!configEditorState.open) return; setConfigEditorStatus(configEditorIsDirty() ? 'Config changes pending' : 'Config unchanged'); }
function openConfigEditorBackdrop() { const backdrop = configEditorBackdrop(); if (backdrop) backdrop.style.display = 'flex'; configEditorState.open = true; }
function closeConfigEditorBackdrop() { const backdrop = configEditorBackdrop(); if (backdrop) backdrop.style.display = 'none'; configEditorState.open = false; }
async function openConfigEditor() { if (configEditorState.loading || configEditorState.saving) return; configEditorState.loading = true; openConfigEditorBackdrop(); showConfigEditorLoading('Loading config template, comments, and current values...'); setConfigEditorStatus('Loading config editor...'); try { const response = await fetch('/api/config-editor'); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to load config editor data.'); configEditorState.sections = payload.sections || []; configEditorState.albumName = String(payload.album_name || ''); configEditorState.hasAlbumOverride = !!payload.has_album_override; configEditorState.showOverridesOnly = false; configEditorState.saveTarget = configEditorState.albumName ? 'album' : 'global'; hideConfigEditorLoading(); renderConfigEditorSections(); configEditorState.originalValues = collectConfigEditorValues(); refreshConfigEditorModeUi(); const searchInput = configEditorSearchInput(); if (searchInput) searchInput.value = ''; setConfigEditorStatus('Config editor ready'); } catch (error) { showConfigEditorLoading('Failed to load config editor: ' + error); setConfigEditorStatus('Config editor failed to load'); } finally { configEditorState.loading = false; } }
function filterConfigEditorFields() {
    const query = String(configEditorSearchInput()?.value || '').trim().toLowerCase();
    const overridesOnly = configEditorState.showOverridesOnly;
    const sections = document.querySelectorAll('.cfgSection');
    sections.forEach(section => {
        const rows = section.querySelectorAll('.cfgFieldRow[data-config-search]');
        if (!query && !overridesOnly) {
            rows.forEach(row => row.classList.remove('cfgHidden'));
            const isDefaultOpen = section.getAttribute('data-default-open') === '1';
            section.classList.toggle('collapsed', !isDefaultOpen);
            return;
        }
        let anyVisible = false;
        rows.forEach(row => {
            const corpus = String(row.getAttribute('data-config-search') || '');
            const matchesQuery = !query || corpus.includes(query);
            const matchesOverride = !overridesOnly || row.getAttribute('data-album-override') === '1';
            const hidden = !matchesQuery || !matchesOverride;
            row.classList.toggle('cfgHidden', hidden);
            if (!hidden) anyVisible = true;
        });
        if (anyVisible) section.classList.remove('collapsed');
        else section.classList.add('collapsed');
    });
}
function toggleConfigEditorOverrideFilter() {
    if (!configEditorState.hasAlbumOverride) return;
    configEditorState.showOverridesOnly = !configEditorState.showOverridesOnly;
    refreshConfigEditorModeUi();
    filterConfigEditorFields();
    setConfigEditorStatus(
        configEditorState.showOverridesOnly
            ? 'Showing album override fields only'
            : 'Showing all config fields'
    );
}
function configEditorSaveTargetLabel() { return configEditorState.saveTarget === 'album' ? `album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; }
function configEditorFieldMetaMap() { const fieldMeta = {}; (Array.isArray(configEditorState.sections) ? configEditorState.sections : []).forEach(section => { (Array.isArray(section.fields) ? section.fields : []).forEach(field => { fieldMeta[field.id] = { key: field.key || field.id || 'field', comments: Array.isArray(field.comments) ? field.comments : [], section: section.title || 'Section' }; }); }); return fieldMeta; }
function configEditorChangeList(values = collectConfigEditorValues()) { const originalValues = configEditorState.originalValues || {}; const metaMap = configEditorFieldMetaMap(); const changes = []; Object.keys(values).forEach(fieldId => { const before = String(originalValues[fieldId] ?? ''); const after = String(values[fieldId] ?? ''); if (before === after) return; const meta = metaMap[fieldId] || { key: fieldId, comments: [], section: 'Section' }; let kind = 'updated'; if (!before && after) kind = 'added'; else if (before && !after) kind = 'removed'; const reason = meta.comments.length ? meta.comments[0] : ''; changes.push({ fieldId, key: meta.key, section: meta.section, before, after, kind, reason }); }); return changes; }
function configEditorChangeSummaryHtml(values = collectConfigEditorValues()) {
    const changes = configEditorChangeList(values); if (!changes.length) return '<p class="cfgConfirmBody">No field values have changed yet.</p>'; const buckets = { added: [], removed: [], updated: [] }; changes.forEach(change => { buckets[change.kind].push(change); }); const renderItem = change => {
        const beforeText = cfgHtml(change.before || '(empty)');
        const afterText = cfgHtml(change.after || '(empty)');
        const reasonText = change.reason ? `<div class='cfgConfirmReason'>Reason: ${cfgHtml(change.reason)}</div>` : '';
        return `<li><strong>${cfgHtml(change.key)}</strong> <span class='cfgChangeKind'>${cfgHtml(change.kind)}</span> <span class='cfgChangeValue'>${beforeText} → ${afterText}</span>${reasonText}</li>`;
    };
    const sections = [];
    if (buckets.added.length) sections.push(`<div class='cfgConfirmGroup'><div class='cfgConfirmGroupTitle'>Added</div><ul>${buckets.added.map(renderItem).join('')}</ul></div>`);
    if (buckets.removed.length) sections.push(`<div class='cfgConfirmGroup'><div class='cfgConfirmGroupTitle'>Removed</div><ul>${buckets.removed.map(renderItem).join('')}</ul></div>`);
    if (buckets.updated.length) sections.push(`<div class='cfgConfirmGroup'><div class='cfgConfirmGroupTitle'>Updated</div><ul>${buckets.updated.map(renderItem).join('')}</ul></div>`);
    return `<div class='cfgConfirmBody'>${sections.join('')}</div>`;
}
function configEditorSaveConfirmChoice(values = collectConfigEditorValues()) { return new Promise(resolve => { const changes = configEditorChangeList(values); const targetLabel = configEditorSaveTargetLabel(); const changeSummary = configEditorChangeSummaryHtml(values); const changeCountText = changes.length ? `Changed fields (${changes.length})` : 'No field values have changed yet.'; const dlg = document.createElement('dialog'); dlg.className = 'cfgConfirmDialog'; dlg.innerHTML = `<h4 class='cfgConfirmTitle'>Save config changes?</h4>` + `<p class='cfgConfirmBody'>This will write to ${cfgHtml(targetLabel)} and reload the review UI.</p>` + `<p class='cfgConfirmBody'><strong>${cfgHtml(changeCountText)}</strong></p>` + changeSummary + "<div class='cfgConfirmActions'>" + "<button id='_cfg_cancel' class='cfgSecondaryBtn'>Keep Editing</button>" + "<button id='_cfg_save' class='cfgPrimaryBtn'>Save</button>" + "</div>"; document.body.appendChild(dlg); dlg.showModal(); const finish = choice => { dlg.close(); dlg.remove(); resolve(choice); }; dlg.querySelector('#_cfg_save').onclick = () => finish('save'); dlg.querySelector('#_cfg_cancel').onclick = () => finish('cancel'); dlg.addEventListener('cancel', () => finish('cancel')); }); }
function configEditorExitChoice(message) { return new Promise(resolve => { const dlg = document.createElement('dialog'); dlg.className = 'cfgConfirmDialog'; const target = configEditorState.saveTarget === 'album' ? `album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; dlg.innerHTML = `<h4 class='cfgConfirmTitle'>${cfgHtml(message || 'Save config changes?')}</h4>` + `<p class='cfgConfirmBody'>Save applies changes to ${cfgHtml(target)} and reloads the review UI. Discard closes the editor without saving.</p>` + "<div class='cfgConfirmActions'>" + "<button id='_cfg_cancel' class='cfgSecondaryBtn'>Continue Editing</button>" + "<button id='_cfg_discard' class='cfgDangerBtn'>Discard</button>" + "<button id='_cfg_save' class='cfgPrimaryBtn'>Save</button>" + "</div>"; document.body.appendChild(dlg); dlg.showModal(); const finish = choice => { dlg.close(); dlg.remove(); resolve(choice); }; dlg.querySelector('#_cfg_save').onclick = () => finish('save'); dlg.querySelector('#_cfg_discard').onclick = () => finish('discard'); dlg.querySelector('#_cfg_cancel').onclick = () => finish('cancel'); dlg.addEventListener('cancel', () => finish('cancel')); }); }
async function requestCloseConfigEditor() { if (!configEditorState.open || configEditorState.loading || configEditorState.saving) { closeConfigEditorBackdrop(); return; } if (!configEditorIsDirty()) { closeConfigEditorBackdrop(); setConfigEditorStatus('Config editor closed'); return; } const choice = await configEditorExitChoice('Save config changes before closing?'); if (choice === 'save') { await saveConfigEditorChanges(true); return; } if (choice === 'discard') { closeConfigEditorBackdrop(); setConfigEditorStatus('Config changes discarded'); } }
async function saveConfigEditorChanges(skipConfirm = false) { if (configEditorState.saving) return false; const values = collectConfigEditorValues(); if (!skipConfirm) { const confirmChoice = await configEditorSaveConfirmChoice(values); if (confirmChoice !== 'save') return false; } configEditorState.saving = true; const saveTarget = configEditorState.saveTarget === 'album' ? 'album' : 'global'; const endpoint = saveTarget === 'album' ? '/api/config-editor-save-album' : '/api/config-editor-save'; const savingLabel = saveTarget === 'album' ? `Saving album override (${configEditorState.albumName || 'current'})...` : 'Saving global config.toml...'; setConfigEditorStatus(savingLabel); if (typeof setBusy === 'function') setBusy(true, savingLabel); try { const response = await fetch(endpoint, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ values }) }); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to save config values.'); configEditorState.originalValues = { ...values }; closeConfigEditorBackdrop(); setConfigEditorStatus(payload.message || 'Config saved.'); if (typeof setBusy === 'function') setBusy(false, ''); if (payload.reload_required) { if (typeof setBusy === 'function') setBusy(true, 'Config saved. Reloading review UI...'); setTimeout(() => { window.location.reload(); }, 1600); } return true; } catch (error) { if (typeof setBusy === 'function') setBusy(false, ''); setConfigEditorStatus('Config save failed'); alert('Config save failed: ' + error); return false; } finally { configEditorState.saving = false; } }
async function deleteAlbumConfigOverride() { if (configEditorState.saving || configEditorState.loading) return; if (!configEditorState.albumName) return; const ok = confirm(`Delete album override config for ${configEditorState.albumName}?\n\nThis removes Albums/${configEditorState.albumName}/config.toml and falls back to global config.toml.`); if (!ok) return; configEditorState.saving = true; setConfigEditorStatus(`Deleting album override for ${configEditorState.albumName}...`); if (typeof setBusy === 'function') setBusy(true, `Deleting album override for ${configEditorState.albumName}...`); try { const response = await fetch('/api/config-editor-delete-album-override', { method: 'POST' }); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to delete album override.'); closeConfigEditorBackdrop(); setConfigEditorStatus(payload.message || 'Album override deleted.'); if (typeof setBusy === 'function') setBusy(false, ''); if (payload.reload_required) { if (typeof setBusy === 'function') setBusy(true, 'Album override deleted. Reloading review UI...'); setTimeout(() => { window.location.reload(); }, 1600); } } catch (error) { if (typeof setBusy === 'function') setBusy(false, ''); setConfigEditorStatus('Delete override failed'); alert('Delete override failed: ' + error); } finally { configEditorState.saving = false; } }
window.addEventListener('beforeunload', event => { if (!configEditorState.open || !configEditorIsDirty()) return; event.preventDefault(); event.returnValue = ''; });
configEditorBackdrop()?.addEventListener('click', event => { if (event.target === configEditorBackdrop()) requestCloseConfigEditor(); });
