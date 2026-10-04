const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const views = path.join(__dirname, '../src/review_ui/views');
const helper = fs.readFileSync(path.join(views, 'stampworld_start_error.js'), 'utf8');
const missing = 'StampWorld account login credentials are missing. Please register and save your username and password in Site Accounts before pulling StampWorld data.';

function functionSource(file, name, nextName) {
    const source = fs.readFileSync(path.join(views, file), 'utf8');
    const start = source.indexOf(`function ${name}(`);
    const end = source.indexOf(`\n${nextName}`, start);
    assert.ok(start >= 0 && end > start, `locate ${name}`);
    return source.slice(start, end);
}

function harness() {
    const elements = new Map();
    function element(tag) {
        return {
            tag, style: {}, value: '', textContent: '', hidden: false, open: false,
            children: [],
            setAttribute(name, value) { this[name] = value; },
            append(...children) {
                this.children.push(...children);
                for (const child of children) if (child.id) elements.set(child.id, child);
            },
            appendChild(child) { this.append(child); },
            showModal() { assert.equal(this.open, false); this.open = true; },
            close() { this.open = false; }
        };
    }
    for (const id of ['siteAccountsDialog', 'siteAccountsReason', 'siteAccountsStatus',
        'swPasswordInput', 'colPasswordInput', 'status']) {
        const el = element('div');
        el.id = id;
        elements.set(id, el);
    }
    const context = vm.createContext({
        document: { getElementById: id => elements.get(id), createElement: element, body: element('body') },
        loadAccountAuthStatus() {},
        setBusy() {},
        updateStampWorldRunStatus() {},
        updateReviewStampWorldStatus() {},
        logRuntimeCompare() {},
        emitReviewClientLog() {},
        getEffectiveStampworldCountry: () => 'Spain',
        stampworldCountryUrl: { async ensureSaved() {} },
        stampworldMinYear: 1840,
        stampworldRunRequested: false,
        stampworldRunPollTimer: null,
        reviewStampWorldRunRequested: false,
        reviewStampWorldRunPollTimer: null,
        reviewCountryName: 'Spain',
        setInterval() { throw new Error('Rejected pull must not start polling'); },
        async fetch() { return { status: 409, async json() { return { ok: false, started: false, message: missing }; } }; }
    });
    vm.runInContext(helper, context);
    return { context, elements };
}

for (const [file, run, next] of [
    ['bootstrap.js', 'runStampWorldPull', 'function openReviewAfterStampWorld('],
    ['compare.js', 'runReviewStampWorldPull', 'async function readSiteAccountResponse(']
]) {
    test(`${file}: rejected credential pull opens explanation, then existing accounts, on every attempt`, async () => {
        const { context, elements } = harness();
        vm.runInContext(functionSource(file, 'openSiteAccountsDialog', 'function closeSiteAccountsDialog('), context);
        vm.runInContext('async ' + functionSource(file, run, next), context);
        for (let attempt = 0; attempt < 2; attempt++) {
            await context[run]();
            assert.equal(context.stampworldRunRequested, false);
            assert.equal(context.reviewStampWorldRunRequested, false);
            const dialog = elements.get('stampworldStartErrorDialog');
            assert.equal(dialog.open, true);
            assert.equal(elements.get('stampworldStartErrorReason').textContent, missing);
            assert.equal(elements.get('stampworldStartErrorAccounts').hidden, false);
            assert.equal(elements.get('stampworldStartErrorDismiss').textContent, 'Not Now');
            elements.get('swPasswordInput').value = 'unsaved password';
            elements.get('stampworldStartErrorAccounts').onclick();
            assert.equal(dialog.open, false);
            assert.equal(elements.get('siteAccountsDialog').open, true);
            assert.ok(elements.get('siteAccountsReason').textContent.includes(missing));
            assert.ok(elements.get('siteAccountsReason').textContent.includes('run the StampWorld pull again'));
            assert.equal(elements.get('swPasswordInput').value, '');
            assert.equal(elements.get('siteAccountsReason').hidden, false);
            elements.get('siteAccountsDialog').close();
        }
        context.openSiteAccountsDialog();
        assert.equal(elements.get('siteAccountsReason').hidden, true);
        assert.equal(elements.get('siteAccountsReason').textContent, '');
    });
}

test('Not Now dismisses without opening Site Accounts; repeated errors update rather than duplicate dialogs', () => {
    const { context, elements } = harness();
    context.openSiteAccountsDialog = () => { throw new Error('Not Now must not open accounts'); };
    context.showStampWorldStartError(missing);
    const dialog = elements.get('stampworldStartErrorDialog');
    elements.get('stampworldStartErrorDismiss').onclick();
    assert.equal(dialog.open, false);
    assert.equal(elements.get('siteAccountsDialog').open, false);
    context.showStampWorldStartError('CREDENTIALS are missing');
    context.showStampWorldStartError('<img src=x onerror=alert(1)> invalid year range');
    assert.equal(elements.get('stampworldStartErrorDialog'), dialog);
    assert.equal(elements.get('stampworldStartErrorAccounts').hidden, true);
    assert.equal(elements.get('stampworldStartErrorDismiss').textContent, 'Close');
    assert.equal(elements.get('stampworldStartErrorReason').textContent, '<img src=x onerror=alert(1)> invalid year range');
    assert.equal(elements.get('stampworldStartErrorReason').children.length, 0);
});

test('An already-running review pull reattaches without prompting for credentials', async () => {
    const { context, elements } = harness();
    let polls = 0;
    context.fetch = async () => ({
        status: 409,
        async json() { return { ok: false, message: 'A StampWorld pull is already running' }; }
    });
    context.setInterval = () => 1;
    context.pollReviewStampWorldStatus = async () => { polls++; };
    vm.runInContext('async ' + functionSource('compare.js', 'runReviewStampWorldPull', 'async function readSiteAccountResponse('), context);
    await context.runReviewStampWorldPull();
    assert.equal(context.reviewStampWorldRunRequested, true);
    assert.equal(polls, 1);
    assert.equal(elements.has('stampworldStartErrorDialog'), false);
});

test('Accepted pulls on both screens still start polling without a failure dialog', async () => {
    for (const [file, run, next] of [
        ['bootstrap.js', 'runStampWorldPull', 'function openReviewAfterStampWorld('],
        ['compare.js', 'runReviewStampWorldPull', 'async function readSiteAccountResponse(']
    ]) {
        const { context, elements } = harness();
        let polls = 0;
        context.fetch = async () => ({
            status: 200,
            async json() { return { ok: true, started: true, message: 'Started' }; }
        });
        context.setInterval = () => 1;
        context.pollStampWorldStatus = async () => { polls++; };
        context.pollReviewStampWorldStatus = async () => { polls++; };
        context.checkAlbumeasyStatusAfterPull = async () => {};
        vm.runInContext('async ' + functionSource(file, run, next), context);
        await context[run]();
        assert.equal(polls, 1);
        assert.equal(elements.has('stampworldStartErrorDialog'), false);
    }
});
