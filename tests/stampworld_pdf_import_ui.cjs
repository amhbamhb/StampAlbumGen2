const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const test = require('node:test');

for (const view of ['bootstrap', 'compare']) {
    function fixture() {
        const source = fs.readFileSync(path.join(__dirname, '../src/review_ui/views', view + '.js'), 'utf8');
        const start = source.indexOf(view === 'bootstrap'
            ? 'async function pickAndCopyStampworldPdfFile()'
            : 'async function importReviewStampworldFile()');
        const end = source.indexOf(view === 'bootstrap'
            ? '\nfunction appendStampworldCopyLog('
            : '\nfunction initReviewStampWorldPanel(', start);
        const elements = Object.fromEntries([
            'countrySel', 'stampworldPickBtn', 'stampworldCopyStatus', 'stampworldCopyLog',
            'reviewSwImportBtn', 'reviewSwImportStatus', 'f_show_imageless_left'
        ].map(id => [id, { value: 'Great Britain', disabled: false, style: {}, textContent: '', checked: false }]));
        const requests = [];
        const logs = [];
        let refreshes = 0;
        let response = { ok: true, picked: true, message: 'found=22, imported=22, skipped=0' };
        let refreshOK = true;
        const context = vm.createContext({
            document: { getElementById: id => elements[id] },
            reviewCountryName: 'Great Britain',
            appendStampworldCopyLog: text => logs.push(text),
            logRuntimeCompare: text => logs.push(text),
            refreshState: async () => { refreshes++; },
            reloadLeftRowsFromServer: async () => { refreshes++; return refreshOK; },
            fetch: async (url, options) => {
                requests.push({ url, body: JSON.parse(options.body) });
                return { ok: response.ok, json: async () => response };
            }
        });
        vm.runInContext(source.slice(start, end), context);
        return {
            elements, requests, logs,
            run: () => view === 'bootstrap' ? context.pickAndCopyStampworldPdfFile() : context.importReviewStampworldFile(),
            setResponse: value => { response = value; },
            failRefresh: () => { refreshOK = false; },
            refreshes: () => refreshes,
            status: () => elements[view === 'bootstrap' ? 'stampworldCopyStatus' : 'reviewSwImportStatus'],
            button: () => elements[view === 'bootstrap' ? 'stampworldPickBtn' : 'reviewSwImportBtn']
        };
    }

    test(`${view}: imports into selected country, shows counts, refreshes and restores button`, async () => {
        const f = fixture();
        await f.run();
        assert.deepEqual(f.requests, [{ url: '/api/stampworld-import-file', body: { country: 'Great Britain' } }]);
        assert.match(f.status().textContent, /found=22, imported=22, skipped=0/);
        assert.equal(f.refreshes(), 1);
        assert.equal(f.button().disabled, false);
        if (view === 'compare') assert.equal(f.elements.f_show_imageless_left.checked, true);
    });

    test(`${view}: cancellation and parser/database errors never report import success`, async () => {
        const f = fixture();
        f.setResponse({ ok: true, picked: false, message: 'Cancelled' });
        await f.run();
        assert.equal(f.refreshes(), 0);
        assert.equal(f.status().textContent, 'Cancelled');
        f.setResponse({ ok: false, message: 'Database insert failed: disk full' });
        await f.run();
        assert.match(f.status().textContent, /Database insert failed: disk full/);
        assert.equal(f.refreshes(), 0);
        assert.equal(f.button().disabled, false);
    });

    if (view === 'compare') {
        test('compare: refresh failure distinguishes persisted import from stale display', async () => {
            const f = fixture();
            f.failRefresh();
            await f.run();
            assert.match(f.status().textContent, /Import completed.*refreshing.*failed/);
            assert.equal(f.button().disabled, false);
        });
    } else {
        test('bootstrap: selecting a country is required before opening picker', async () => {
            const f = fixture();
            f.elements.countrySel.value = '';
            await f.run();
            assert.equal(f.requests.length, 0);
            assert.match(f.logs[0], /Select a country first/);
        });
    }
}
