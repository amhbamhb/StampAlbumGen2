const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const test = require('node:test');

function harness(fetch) {
    const elements = Object.fromEntries(
        ['stampworldCountryUrl', 'saveStampworldCountryUrl', 'stampworldCountryUrlStatus']
            .map(id => [id, {
                value: '', textContent: '', disabled: false, placeholder: '', listeners: {},
                addEventListener(name, callback) { this.listeners[name] = callback; }
            }]));
    const context = vm.createContext({
        document: { getElementById: id => elements[id] }, fetch
    });
    const source = fs.readFileSync(path.join(__dirname, '..', 'src', 'review_ui', 'views', 'country_url.js'), 'utf8');
    vm.runInContext(source + '\nglobalThis.Controller = StampWorldCountryUrl;', context);
    let country = 'South Korea';
    const controller = new context.Controller(() => country);
    return {
        controller, elements,
        country(value) { country = value; },
        edit(value) {
            elements.stampworldCountryUrl.value = value;
            elements.stampworldCountryUrl.listeners.input();
        }
    };
}

function response(url, custom = false) {
    return { ok: true, status: 200, json: async () => ({ ok: true, url, custom }) };
}

test('automatic URL, custom restore, save-before-pull and clearing to automatic', async () => {
    const url = 'https://www.stampworld.com/en/stamps/South-Korea/';
    const sent = [];
    let saved = false;
    const h = harness(async (_endpoint, options) => {
        if (options.body) {
            const body = JSON.parse(options.body);
            sent.push(body);
            saved = !!body.url;
        }
        return response(url, saved);
    });
    await h.controller.load('South Korea');
    assert.equal(h.elements.stampworldCountryUrl.value, '');
    assert.equal(h.elements.stampworldCountryUrl.placeholder, url);
    assert.match(h.elements.stampworldCountryUrlStatus.textContent, /Automatic URL/);
    h.edit(url);
    await h.controller.ensureSaved('South Korea');
    assert.deepEqual(sent, [{ country: 'South Korea', url }]);
    assert.match(h.elements.stampworldCountryUrlStatus.textContent, /Saved URL/);
    const restored = harness(async () => response(url, true));
    await restored.controller.load('South Korea');
    assert.equal(restored.elements.stampworldCountryUrl.value, url);
    h.edit('');
    await h.controller.ensureSaved('South Korea');
    assert.deepEqual(sent[1], { country: 'South Korea', url: '' });
    assert.match(h.elements.stampworldCountryUrlStatus.textContent, /Automatic URL restored/);
});

test('country switching ignores an earlier response and never saves it to the new country', async () => {
    let finishOld;
    const sent = [];
    const h = harness(async (endpoint, options) => {
        if (options.body) {
            sent.push(JSON.parse(options.body));
            return response('https://www.stampworld.com/en/stamps/British-Canada/', true);
        }
        if (endpoint.includes('South%20Korea')) return new Promise(resolve => { finishOld = resolve; });
        return response('https://www.stampworld.com/en/stamps/British-Canada/');
    });
    const old = h.controller.load('South Korea');
    h.country('British Canada');
    await h.controller.load('British Canada');
    finishOld(response('https://www.stampworld.com/en/stamps/South-Korea/', true));
    await old;
    assert.equal(h.elements.stampworldCountryUrl.value, '');
    assert.match(h.elements.stampworldCountryUrl.placeholder, /British-Canada/);
    h.edit('https://www.stampworld.com/en/stamps/British-Canada/');
    await h.controller.ensureSaved('British Canada');
    assert.equal(sent.length, 1);
    assert.equal(sent[0].country, 'British Canada');
});

test('load failure is visible, blocks pulling, and can be retried', async () => {
    let failed = true;
    const h = harness(async () => {
        if (failed) throw new Error('mapping file unreadable');
        return response('https://www.stampworld.com/en/stamps/South-Korea/');
    });
    await h.controller.load('South Korea');
    assert.match(h.elements.stampworldCountryUrlStatus.textContent, /mapping file unreadable/);
    assert.equal(h.elements.stampworldCountryUrl.disabled, true);
    await assert.rejects(h.controller.ensureSaved('South Korea'), /unavailable/);
    failed = false;
    await h.controller.ensureSaved('South Korea');
    assert.equal(h.elements.stampworldCountryUrl.disabled, false);
});

test('invalid save remains dirty, reports the error and prevents the pull', async () => {
    const h = harness(async (_endpoint, options) => options.body
        ? { ok: false, status: 400, json: async () => ({ ok: false, message: 'Use a StampWorld URL' }) }
        : response('https://www.stampworld.com/en/stamps/South-Korea/'));
    await h.controller.load('South Korea');
    h.edit('https://example.invalid/');
    await assert.rejects(h.controller.ensureSaved('South Korea'), /Use a StampWorld URL/);
    assert.equal(h.controller.dirty, true);
    assert.equal(h.elements.stampworldCountryUrl.disabled, false);
    await h.elements.saveStampworldCountryUrl.listeners.click();
    await new Promise(resolve => setImmediate(resolve));
    assert.match(h.elements.stampworldCountryUrlStatus.textContent, /Use a StampWorld URL/);
});

test('a country change during a save blocks starting the outdated pull', async () => {
    let finishSave;
    const h = harness(async (_endpoint, options) => options.body
        ? new Promise(resolve => { finishSave = resolve; })
        : response('https://www.stampworld.com/en/stamps/South-Korea/'));
    await h.controller.load('South Korea');
    h.edit('https://www.stampworld.com/en/stamps/South-Korea/');
    const save = h.controller.ensureSaved('South Korea');
    await new Promise(resolve => setImmediate(resolve));
    await assert.rejects(h.controller.ensureSaved('South Korea'), /already in progress/);
    h.country('British Canada');
    await h.controller.load('British Canada');
    finishSave(response('https://www.stampworld.com/en/stamps/South-Korea/', true));
    await assert.rejects(save, /selected country changed/);
    assert.equal(h.controller.saving, false);
});
