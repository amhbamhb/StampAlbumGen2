const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const test = require('node:test');

for (const view of ['bootstrap', 'compare']) {
    test(`${view}: site accounts load, preserve passwords, and report API errors`, async () => {
        const source = fs.readFileSync(
            path.join(__dirname, '..', 'src', 'review_ui', 'views', `${view}.js`), 'utf8');
        const start = source.indexOf('async function readSiteAccountResponse(');
        const end = source.indexOf(
            view === 'bootstrap' ? '\nasync function startReview(' : '\nfunction copyReviewStampWorldCommand(',
            source.indexOf('async function submitSiteAccounts('));
        const elements = Object.fromEntries(
            ['swUsernameInput', 'swPasswordInput', 'colUsernameInput', 'colPasswordInput', 'siteAccountsStatus']
                .map(id => [id, { value: '', style: {}, innerText: '', placeholder: '' }]));
        elements.siteAccountsDialog = { showModal() {}, close() {} };
        let sent;
        let response = {
            ok: true, status: 200,
            text: async () => JSON.stringify({
                ok: true,
                stampworld: { username: 'sw-test', has_password: true },
                colnect: { username: 'col-test', has_password: false }
            })
        };
        const context = vm.createContext({
            document: { getElementById: id => elements[id] },
            fetch: async (_url, options) => {
                if (options?.body) sent = JSON.parse(options.body);
                return response;
            },
            setTimeout: () => {}
        });
        vm.runInContext(source.slice(start, end), context);
        await context.loadAccountAuthStatus();
        assert.equal(elements.swUsernameInput.value, 'sw-test');
        assert.match(elements.swPasswordInput.placeholder, /already saved/);
        assert.equal(elements.swPasswordInput.value, '');
        assert.equal(elements.colUsernameInput.value, 'col-test');
        await context.submitSiteAccounts();
        assert.equal(sent.stampworld_password, null);
        assert.equal(sent.colnect_password, null);
        elements.swPasswordInput.value = 'replacement-secret';
        await context.submitSiteAccounts();
        assert.equal(sent.stampworld_password, 'replacement-secret');
        response = { ok: false, status: 404, text: async () => '' };
        await context.loadAccountAuthStatus();
        assert.match(elements.siteAccountsStatus.innerText, /Unable to load.*HTTP 404/);
        await context.submitSiteAccounts();
        assert.match(elements.siteAccountsStatus.innerText, /empty response.*HTTP 404/);
        response = { ok: false, status: 500, text: async () => '{"ok":false,"message":"write failed"}' };
        await context.submitSiteAccounts();
        assert.equal(elements.siteAccountsStatus.innerText, 'Error: write failed');
        response = { ok: true, status: 200, text: async () => '<html>' };
        await context.submitSiteAccounts();
        assert.match(elements.siteAccountsStatus.innerText, /invalid JSON/);
    });
}
