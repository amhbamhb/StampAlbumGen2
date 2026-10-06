const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const script = fs.readFileSync(path.join(__dirname, '../src/review_ui/views/external_album.js'), 'utf8');
const markup = fs.readFileSync(path.join(__dirname, '../src/review_ui/views/external_album.html'), 'utf8');
const argument = (value, quoted = false) => ({ value, quoted });
function fixture(status = 'ready') {
    return {
        name: 'Brazil-Imported', display_name: '<img onerror=alert(1)> Brazil',
        revision: 3, status, message: '', warnings: ['Legacy image spacing is preserved'],
        reference_pdf: 'reference.pdf', generated_pdf: 'generated.pdf',
        globals: [{ name: 'ALBUM_PAGES_SIZE', source_line: 1, args: [argument('217'), argument('280')] }],
        pages: [1, 2].map(index => ({ index, commands: [
            { name: 'PAGE_START', args: [], source_line: 10 * index },
            { name: 'ROW_START_FS', source_line: 11 * index, args: ['CN', '6', '0.1', '5'].map(v => argument(v)) },
            { name: 'STAMP_ADD_IMG', source_line: 12 * index,
                args: [argument('30'), argument('30'), argument('Images/1.jpg', true),
                    argument('', true), argument('<script>bad()</script>', true), argument('', true)] }
        ] }))
    };
}

function harness(custom) {
    const elements = new Map();
    const requests = [];
    const nativeDialogs = [];
    function element(tag = 'div') {
        return {
            tag, value: '', checked: false, disabled: false, hidden: false, open: false,
            children: [], listeners: {}, files: [],
            get options() { return this.children; },
            appendChild(child) {
                this.children.push(child);
                if (this.tag === 'select' && this.children.length === 1) this.value = child.value;
            },
            replaceChildren() { this.children = []; },
            addEventListener(event, fn) { this.listeners[event] = fn; },
            showModal() { this.open = true; },
            close() { this.open = false; },
            removeAttribute(name) { delete this[name]; }
        };
    }
    for (const match of markup.matchAll(/<(input|select|dialog|[a-z][a-z0-9]*)\b[^>]*\bid="([^"]+)"/g)) {
        elements.set(match[2], element(match[1]));
    }
    const context = vm.createContext({
        document: { getElementById: id => elements.get(id), createElement: element },
        setTimeout: fn => { fn(); return 1; },
        beginNativeDialog: message => nativeDialogs.push(message),
        endNativeDialog: () => nativeDialogs.push('closed'),
        refreshState: async () => {},
        async fetch(url, options = {}) {
            const data = options.body ? JSON.parse(options.body) : undefined;
            requests.push({ url, data });
            const result = custom ? await custom(url, data) : { ok: true, albums: [] };
            return { ok: result.ok, status: result.ok ? 200 : 400, json: async () => result };
        }
    });
    vm.runInContext(script + '\nglobalThis.controller = externalAlbums;', context);
    return { elements, requests, nativeDialogs, controller: context.controller };
}

const inspection = { files: 661, bytes: 12345, pages: 85, placements: 655,
    unique_images: 649, commands: ['PAGE_START', 'STAMP_ADD_IMG'], warnings: [], errors: [] };

test('source folder picker uses the shared native-dialog health guard and fills the single entry', async () => {
    const h = harness(url => url.endsWith('/pick')
        ? { ok: true, source: 'C:\\Stamp Albums\\Brazil', entries: ['Brazil.txt'] }
        : { ok: true, albums: [] });
    await h.controller.pick();
    assert.equal(h.elements.get('externalSource').value, 'C:\\Stamp Albums\\Brazil');
    assert.equal(h.elements.get('externalEntry').value, 'Brazil.txt');
    assert.equal(h.nativeDialogs.length, 2);
    assert.equal(h.nativeDialogs[1], 'closed');
    assert.equal(h.elements.get('externalImportInputs').disabled, false);
});

test('wizard blocks import until inspection and full-folder consent, and invalidates stale inspection', async () => {
    const h = harness(url => url.endsWith('inspect') ? { ok: true, inspection } : { ok: true, albums: [] });
    h.controller.openImport();
    h.elements.get('externalSource').value = '/source/Brazil';
    h.elements.get('externalEntry').value = 'Brazil.txt';
    await h.controller.inspect();
    assert.equal(h.elements.get('externalImportRun').disabled, true);
    h.elements.get('externalCopyConsent').checked = true;
    h.elements.get('externalCopyConsent').listeners.change();
    assert.equal(h.elements.get('externalImportRun').disabled, false);
    h.elements.get('externalEntry').value = 'other.txt';
    h.elements.get('externalEntry').listeners.input();
    assert.equal(h.elements.get('externalImportRun').disabled, true);
    await h.controller.importAlbum();
    assert.equal(h.requests.some(r => r.url.endsWith('/import')), false);
});

test('unsupported commands keep the conversion blocked and expose exact diagnostics as text', async () => {
    const h = harness(url => url.endsWith('inspect')
        ? { ok: true, inspection: { ...inspection, errors: ['Line 74: UNSUPPORTED <img src=x>'] } }
        : { ok: true, albums: [] });
    await h.controller.inspect();
    h.elements.get('externalCopyConsent').checked = true;
    h.elements.get('externalCopyConsent').listeners.change();
    assert.equal(h.elements.get('externalImportRun').disabled, true);
    assert.match(h.elements.get('externalInspection').textContent, /Line 74: UNSUPPORTED <img src=x>/);
});

test('draft disables edits, validated requires explicit fidelity consent, and ready enables editing', async () => {
    let album = fixture('draft');
    const h = harness(url => url.includes('/load?') ? { ok: true, album } : { ok: true, albums: [] });
    await h.controller.openAlbum(album.name);
    assert.equal(h.elements.get('externalEditorInputs').disabled, true);
    album = fixture('validated');
    await h.controller.openAlbum(album.name);
    assert.equal(h.elements.get('externalApproval').hidden, false);
    await h.controller.approve();
    assert.equal(h.requests.some(r => r.url.endsWith('/approve')), false);
    assert.match(h.elements.get('externalEditorStatus').textContent, /Compare both PDFs/);
    album = fixture('ready');
    await h.controller.openAlbum(album.name);
    assert.equal(h.elements.get('externalEditorInputs').disabled, false);
    assert.equal(h.elements.get('externalApproval').hidden, true);
});

test('editor uses safe DOM text and sends only the selected page with its revision', async () => {
    let current = fixture();
    const h = harness((url, data) => {
        if (url.includes('/load?')) return { ok: true, album: current };
        if (url.endsWith('/edit')) {
            assert.equal(data.page, 2);
            assert.equal(data.revision, 3);
            assert.equal(data.commands.length, 4);
            assert.equal(data.commands[3].name, 'STAMP_ADD_IMG');
            assert.equal(data.commands[3].args[0].value, '30');
            current = { ...current, revision: 4 };
            return { ok: true, job: 17 };
        }
        if (url.includes('/job?')) return { ok: true, job: { running: false, result: current } };
        return { ok: true, albums: [] };
    });
    await h.controller.openAlbum(current.name);
    assert.equal(h.elements.get('externalEditorTitle').textContent, current.display_name);
    const rows = h.elements.get('externalCommands').children;
    assert.equal(rows[2].children[1].children[4].children[0].value, '<script>bad()</script>');
    h.elements.get('externalPage').value = '2';
    h.controller.selectPage();
    h.controller.add('STAMP_ADD_IMG');
    await h.controller.save();
    assert.ok(h.requests.some(r => r.url.endsWith('/edit')));
    assert.equal(h.elements.get('externalEditorInputs').disabled, false);
    assert.match(h.elements.get('externalEditorStatus').textContent, /Revision 4/);
});

test('save failure retains unsaved commands and restores controls for correction', async () => {
    const album = fixture();
    const h = harness(url => {
        if (url.includes('/load?')) return { ok: true, album };
        if (url.endsWith('/edit')) return { ok: false, message: 'Page overflow: no room for stamp' };
        return { ok: true, albums: [] };
    });
    await h.controller.openAlbum(album.name);
    h.controller.add('PAGE_TEXT_CENTRE');
    await h.controller.save();
    assert.match(h.elements.get('externalEditorStatus').textContent, /Page overflow/);
    assert.equal(h.elements.get('externalCommands').children.length, 4);
    assert.equal(h.elements.get('externalEditorInputs').disabled, false);
});
