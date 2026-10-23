const externalAlbums = (() => {
    let inspectionKey = '';
    let album = null;
    let commands = [];
    let busy = false;
    let inspectionReport = null;
    const labels = {
        ALBUM_PAGES_SIZE: ['Width', 'Height'],
        ALBUM_PAGES_MARGINS: ['Left', 'Right', 'Top', 'Bottom'],
        ALBUM_PAGES_MARGINSE: ['Left (even pages)', 'Right (even pages)', 'Top', 'Bottom'],
        ALBUM_PAGES_BORDER: ['Outer line width', 'First inner line width', 'Second inner line width', 'Line spacing'],
        ALBUM_PAGES_SPACING: ['Stamp spacing', 'Row spacing'],
        ALBUM_PAGES_NUMBER: ['Font', 'Size (points)', 'Alignment', 'Starting number', 'Prefix', 'Suffix'],
        ALBUM_PAGES_TITLE: ['Font', 'Size (points)', 'Title'],
        STAMP_BOXES_SIZE_ADJUST: ['Box size adjustment'],
        ALBUM_STAMP_IMG_SETTING: ['Horizontal (legacy units)', 'Vertical (legacy units)', 'Stretch'],
        ALBUM_STAMP_NEW_IMG_SETTING: ['Horizontal', 'Vertical', 'Stretch'],
        ROW_START_FS: ['Font', 'Size (points)', 'Border width', 'Stamp spacing'],
        STAMP_ADD_IMG: ['Box width', 'Box height', 'Image relative path', 'Left caption', 'Centre caption', 'Right caption', 'Optional NO_FRAME'],
        PAGE_TEXT_CENTRE: ['Font', 'Size (points)', 'Text', 'Optional spacing']
    };
    const element = id => document.getElementById(id);
    const text = (id, value) => { element(id).textContent = value; };
    const clone = value => JSON.parse(JSON.stringify(value));
    const source = () => ({
        source: element('externalSource').value.trim(),
        entry: element('externalEntry').value.trim()
    });
    const sourceKey = () => JSON.stringify(source());

    async function api(action, data) {
        const response = await fetch('/api/external-album/' + action, data === undefined ? {} : {
            method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(data)
        });
        const result = await response.json();
        if (!response.ok || !result.ok) throw new Error(result.message || `Request failed (${response.status})`);
        return result;
    }

    function setBusy(value) {
        busy = value;
        element('externalImportInputs').disabled = value;
        element('externalEditorInputs').disabled = value || !album || album.status !== 'ready';
        ['externalImportClose', 'externalEditorClose', 'externalApprove', 'externalRetry'].forEach(id => {
            element(id).disabled = value;
        });
        element('externalFidelityConsent').disabled = value;
        element('externalCopyConsent').disabled = value;
        updateImportButton();
    }

    function updateImportButton() {
        element('externalImportRun').disabled = busy || !inspectionKey ||
            inspectionKey !== sourceKey() || !element('externalCopyConsent').checked;
    }

    async function refresh() {
        try {
            const result = await api('list');
            const list = element('externalAlbumList');
            list.replaceChildren();
            result.albums.forEach(item => {
                const button = document.createElement('button');
                button.type = 'button';
                button.textContent = `${item.display_name} (${item.status})`;
                button.addEventListener('click', () => openAlbum(item.name));
                list.appendChild(button);
            });
            text('externalAlbumStatus', result.albums.length ? '' : 'No imported albums yet. Drafts will appear here too.');
        } catch (error) {
            text('externalAlbumStatus', error.message);
        }
    }

    function openImport() {
        if (busy) return;
        inspectionKey = '';
        inspectionReport = null;
        element('externalCopyConsent').checked = false;
        text('externalInspection', '');
        text('externalImportStatus', '');
        updateImportButton();
        element('externalImportDialog').showModal();
    }

    async function pick() {
        setBusy(true);
        beginNativeDialog('Selecting an external AlbumEasy folder (Albums Root will not change)...');
        text('externalImportStatus', 'Opening folder chooser...');
        try {
            const result = await api('pick', {});
            if (result.source) {
                element('externalSource').value = result.source;
                const choices = element('externalEntryChoices');
                choices.replaceChildren();
                result.entries.forEach(entry => {
                    const option = document.createElement('option');
                    option.value = entry;
                    choices.appendChild(option);
                });
                element('externalEntry').value = result.entries.length === 1 ? result.entries[0] : '';
                inspectionKey = '';
                inspectionReport = null;
            }
            text('externalImportStatus', result.source ? 'Select the entry filename, name the new album, then inspect.' : 'Folder selection cancelled.');
        } catch (error) {
            text('externalImportStatus', error.message + ' You can also type the folder path.');
        } finally {
            endNativeDialog();
            setBusy(false);
            await refreshState();
        }
    }

    async function inspect() {
        inspectionKey = '';
        inspectionReport = null;
        setBusy(true);
        text('externalImportStatus', 'Inspecting commands, images and folder contents...');
        try {
            const key = sourceKey();
            const { inspection: result } = await api('inspect', source());
            inspectionReport = { source: source(), inspection: result };
            const report = [
                `Files to copy: ${result.files}; bytes: ${result.bytes}`,
                `Pages: ${result.pages}; placements: ${result.placements}; unique images: ${result.unique_images}`,
                `Commands: ${result.commands.join(', ')}`,
                ...result.warnings.map(w => 'WARNING: ' + w),
                ...result.errors.map(e => 'BLOCKER: ' + e)
            ];
            text('externalInspection', report.join('\n'));
            if (!result.errors.length) inspectionKey = key;
            text('externalImportStatus', result.errors.length
                ? 'Resolve the blockers before importing. The originals have not been changed.'
                : 'Inspection passed. Confirm the full-folder copy before proceeding.');
        } catch (error) { text('externalImportStatus', error.message); }
        finally { setBusy(false); }
    }

    async function awaitJob(result, statusId) {
        while (true) {
            await new Promise(resolve => setTimeout(resolve, 750));
            const { job } = await api('job?id=' + result.job);
            if (job.running) {
                text(statusId, 'Working: copying / validating / rendering. Keep this window open; the main UI remains responsive.');
                continue;
            }
            if (job.error) throw new Error(job.error);
            if (!job.result) throw new Error('Job completed without an album result. Refresh the imported-album list.');
            return job.result;
        }
    }

    async function importAlbum() {
        if (busy || !inspectionKey || inspectionKey !== sourceKey() || !element('externalCopyConsent').checked) return;
        setBusy(true);
        text('externalImportStatus', 'Starting import...');
        try {
            const result = await api('import', { ...source(), name: element('externalName').value.trim() });
            album = await awaitJob(result, 'externalImportStatus');
            element('externalImportDialog').close();
            renderAlbum();
            await refresh();
        } catch (error) {
            text('externalImportStatus', error.message + ' Refresh imported albums to find any retained draft.');
            await refresh();
        } finally { setBusy(false); }
    }

    async function openAlbum(name) {
        if (busy) return;
        try {
            const result = await api('load?name=' + encodeURIComponent(name));
            album = result.album;
            renderAlbum();
        } catch (error) { text('externalAlbumStatus', error.message); }
    }

    function renderAlbum() {
        text('externalEditorTitle', album.display_name);
        text('externalEditorStatus', `${album.status} | Revision ${album.revision} | ${album.message || ''}`);
        text('externalCompatibility', album.warnings.join('\n'));
        const url = '/api/external-album/pdf?name=' + encodeURIComponent(album.name);
        ['Reference', 'Generated'].forEach(kind => {
            const link = element(`external${kind}Pdf`);
            const available = kind === 'Reference' ? album.reference_pdf : album.generated_pdf;
            link.hidden = !available;
            if (available) link.href = url + '&which=' + kind.toLowerCase() + '&revision=' + album.revision;
            else link.removeAttribute('href');
        });
        element('externalApproval').hidden = album.status !== 'validated';
        element('externalFidelityConsent').checked = false;
        element('externalRetry').hidden = album.status === 'ready';
        const select = element('externalPage');
        const previous = select.value;
        select.replaceChildren();
        [{ index: 0, title: 'Album-wide settings (affect all pages)' },
            ...album.pages.map(page => ({ index: page.index, title: `Page ${page.index}` }))
        ].forEach(page => {
            const option = document.createElement('option');
            option.value = String(page.index);
            option.textContent = page.title;
            select.appendChild(option);
        });
        if ([...select.options].some(option => option.value === previous)) select.value = previous;
        else if (album.pages.length) select.value = String(album.pages[0].index);
        selectPage();
        setBusy(busy);
        if (!element('externalEditorDialog').open) element('externalEditorDialog').showModal();
    }

    function selectPage() {
        const page = Number(element('externalPage').value);
        commands = clone(page === 0 ? album.globals : album.pages.find(p => p.index === page).commands);
        renderCommands();
    }

    function renderCommands() {
        const container = element('externalCommands');
        container.replaceChildren();
        commands.forEach((command, index) => {
            const row = document.createElement('div');
            row.className = 'externalAlbumCommand';
            const title = document.createElement('strong');
            title.textContent = `${index + 1}. ${command.name} (source line ${command.source_line || 'new'})`;
            row.appendChild(title);
            const args = document.createElement('div');
            args.className = 'externalAlbumArgs';
            command.args.forEach((argument, argIndex) => {
                const label = document.createElement('label');
                const description = labels[command.name]?.[argIndex] || `Parameter ${argIndex + 1}`;
                label.textContent = description;
                const input = document.createElement('input');
                input.type = 'text';
                input.value = argument.value;
                input.title = description + (argument.quoted ? ' (text; no surrounding quotes)' : ' (AlbumEasy value)');
                input.addEventListener('input', () => { argument.value = input.value; });
                label.appendChild(input);
                args.appendChild(label);
            });
            row.appendChild(args);
            if (command.name !== 'PAGE_START') {
                const remove = document.createElement('button');
                remove.type = 'button';
                remove.textContent = 'Remove';
                remove.addEventListener('click', () => { commands.splice(index, 1); renderCommands(); });
                row.appendChild(remove);
            }
            ['Move up', 'Move down'].forEach((title, direction) => {
                const button = document.createElement('button');
                button.type = 'button';
                button.textContent = title;
                const target = index + (direction ? 1 : -1);
                button.disabled = target < 0 || target >= commands.length || command.name === 'PAGE_START' || commands[target]?.name === 'PAGE_START';
                button.addEventListener('click', () => {
                    [commands[index], commands[target]] = [commands[target], commands[index]];
                    renderCommands();
                });
                row.appendChild(button);
            });
            container.appendChild(row);
        });
    }

    function add(name) {
        if (!album || album.status !== 'ready' || busy) return;
        if (Number(element('externalPage').value) === 0) {
            text('externalEditorStatus', 'Select a page before adding a stamp, heading or row.');
            return;
        }
        const argument = (value, quoted = false) => ({ value, quoted });
        const defaults = {
            STAMP_ADD_IMG: [argument('30'), argument('30'), argument('Images/example.jpg', true),
                argument('', true), argument('', true), argument('', true)],
            PAGE_TEXT_CENTRE: [argument('HN'), argument('12'), argument('New heading', true)],
            ROW_START_FS: [argument('CN'), argument('6'), argument('0.1'), argument('5')]
        };
        commands.push({ name, args: defaults[name], source_line: 0 });
        renderCommands();
    }

    async function save() {
        if (!album || album.status !== 'ready' || busy) return;
        setBusy(true);
        text('externalEditorStatus', 'Saving layout and validating the new PDF...');
        try {
            const result = await api('edit', {
                name: album.name, page: Number(element('externalPage').value),
                commands, revision: album.revision
            });
            album = await awaitJob(result, 'externalEditorStatus');
            renderAlbum();
        } catch (error) { text('externalEditorStatus', error.message + ' Your unsaved form changes remain available.'); }
        finally { setBusy(false); }
    }

    async function retry() {
        if (!album || busy) return;
        setBusy(true);
        try {
            album = await awaitJob(await api('retry', { name: album.name }), 'externalEditorStatus');
            renderAlbum();
            await refresh();
        } catch (error) { text('externalEditorStatus', error.message); }
        finally { setBusy(false); }
    }

    async function approve() {
        if (!album || busy) return;
        if (!element('externalFidelityConsent').checked) {
            text('externalEditorStatus', 'Compare both PDFs and select the fidelity confirmation first.');
            return;
        }
        setBusy(true);
        try {
            album = (await api('approve', { name: album.name, revision: album.revision })).album;
            renderAlbum();
            await refresh();
        } catch (error) { text('externalEditorStatus', error.message); }
        finally { setBusy(false); }
    }

    async function upload() {
        if (!album || album.status !== 'ready' || busy) return;
        const file = element('externalImageUpload').files[0];
        if (!file || !['image/png', 'image/jpeg'].includes(file.type)) {
            text('externalUploadedPath', 'Select a PNG or JPEG image first.');
            return;
        }
        if (file.size > 5000000) {
            text('externalUploadedPath', 'Image exceeds the 5 MB upload limit.');
            return;
        }
        setBusy(true);
        try {
            const data_url = await new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.onload = () => resolve(reader.result);
                reader.onerror = () => reject(new Error('Cannot read the selected image.'));
                reader.readAsDataURL(file);
            });
            const result = await api('image', { name: album.name, data_url });
            text('externalUploadedPath', 'Image copied: ' + result.path + '. Paste this path into the placement image field, then save the page.');
        } catch (error) { text('externalUploadedPath', error.message); }
        finally { setBusy(false); }
    }

    function download(value, filename) {
        const url = URL.createObjectURL(new Blob([JSON.stringify(value, null, 2)], { type: 'application/json' }));
        const link = document.createElement('a');
        link.href = url;
        link.download = filename;
        document.body.appendChild(link);
        link.click();
        link.remove();
        setTimeout(() => URL.revokeObjectURL(url), 1000);
    }

    function downloadReport() {
        if (!inspectionReport) {
            text('externalImportStatus', 'Inspect the source before saving its report.');
            return;
        }
        download(inspectionReport, 'albumeasy-inspection.json');
    }

    function downloadAlbumReport() {
        if (!album) return;
        download({
            name: album.name, revision: album.revision, status: album.status,
            warnings: album.warnings, message: album.message,
            globals: album.globals, pages: album.pages
        }, 'albumeasy-compatibility.json');
    }

    ['externalSource', 'externalEntry'].forEach(id => {
        element(id)?.addEventListener('input', () => {
            inspectionKey = '';
            inspectionReport = null;
            updateImportButton();
        });
    });
    element('externalCopyConsent')?.addEventListener('change', updateImportButton);
    ['externalImportDialog', 'externalEditorDialog'].forEach(id => {
        element(id)?.addEventListener('cancel', event => { if (busy) event.preventDefault(); });
    });
    const closeImport = () => { if (!busy) element('externalImportDialog').close(); };
    const closeEditor = () => { if (!busy) element('externalEditorDialog').close(); };
    refresh();
    return { openImport, closeImport, closeEditor, pick, inspect, importAlbum, refresh,
        openAlbum, selectPage, add, save, retry, approve, upload, downloadReport, downloadAlbumReport };
})();
