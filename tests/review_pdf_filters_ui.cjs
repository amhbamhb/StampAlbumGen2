const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const test = require('node:test');

const source = fs.readFileSync(path.join(__dirname, '../src/review_ui/views/pdf_filters.js'), 'utf8');

function fixture(storage = new Map()) {
    function element() {
        return {
            children: [], listeners: {}, textContent: '', disabled: false,
            append(...children) { this.children.push(...children); },
            replaceChildren() { this.children = []; },
            addEventListener(event, callback) { this.listeners[event] = callback; },
            querySelectorAll() { return this.children.map(label => label.children[0]); }
        };
    }
    const ids = ['reviewPdfFilterStatus', 'reviewPdfRefreshFilters'];
    for (const field of ['emission', 'format']) {
        ids.push(`reviewPdf${field}Summary`, `reviewPdf${field}Choices`);
    }
    const elements = Object.fromEntries(ids.map(id => [id, element()]));
    let response = {
        ok: true, options: { emission: ['Official', 'Definitive', ''], format: ['Stamp', 'Sheet'] }
    };
    const context = vm.createContext({
        document: {
            getElementById: id => elements[id],
            createElement: element,
            createTextNode: text => ({ textContent: text })
        },
        localStorage: {
            getItem: key => storage.get(key) ?? null,
            setItem: (key, value) => storage.set(key, value)
        },
        fetch: async url => {
            assert.equal(url, '/api/review-pdf-options');
            return { ok: response.ok, status: response.ok ? 200 : 500, json: async () => response };
        }
    });
    vm.runInContext(source + '\nglobalThis.Filters = ReviewPdfFilters;', context);
    return { context, elements, storage, setResponse: value => { response = value; } };
}

test('checkbox dropdowns default to All, support multiple values, and restore per country', async () => {
    const f = fixture();
    const filters = new f.context.Filters('Country A');
    await filters.init();
    assert.equal(f.elements.reviewPdfemissionSummary.textContent, 'Emission: All');
    assert.equal(f.elements.reviewPdfformatSummary.textContent, 'Format: All');
    const official = f.elements.reviewPdfemissionChoices.children[1].children[0];
    official.checked = true;
    official.listeners.change();
    filters.change('emission', 'Definitive', true);
    filters.change('format', 'Stamp', true);
    assert.deepEqual(JSON.parse(JSON.stringify(filters.getSelection())), {
        emission: ['Official', 'Definitive'], format: ['Stamp']
    });
    assert.equal(f.elements.reviewPdfemissionChoices.children[0].children[0].checked, false);
    const restored = new f.context.Filters('Country A');
    await restored.init();
    assert.deepEqual(JSON.parse(JSON.stringify(restored.getSelection())), {
        emission: ['Official', 'Definitive'], format: ['Stamp']
    });
    const other = new f.context.Filters('Country B');
    await other.init();
    assert.equal(other.getSelection().emission.length, 0);
    filters.change('emission', null, true);
    assert.equal(filters.getSelection().emission.length, 0);
    assert.equal(filters.getSelection().format[0], 'Stamp');
    filters.change('format', 'Stamp', false);
    assert.equal(filters.getSelection().format.length, 0);
    assert.equal(f.elements.reviewPdfformatChoices.children[0].children[0].checked, true);
});

test('unknown, unavailable saved choices, disabled controls and refresh errors stay explicit', async () => {
    const f = fixture();
    const filters = new f.context.Filters('Country A');
    await filters.init();
    filters.change('emission', '', true);
    assert.match(f.elements.reviewPdfemissionSummary.textContent, /Unknown/);
    filters.change('emission', 'Revenue', true);
    await filters.refresh();
    assert.match(f.elements.reviewPdfFilterStatus.textContent, /no longer available/);
    assert.match(f.elements.reviewPdfemissionChoices.children.at(-1).children[1].textContent, /not currently available/);
    filters.setDisabled(true);
    assert.ok(f.elements.reviewPdfemissionChoices.querySelectorAll().every(input => input.disabled));
    filters.change('emission', null, true);
    assert.equal(filters.getSelection().emission.length, 2);
    filters.setDisabled(false);
    f.setResponse({ ok: false, message: 'database error' });
    assert.equal(await filters.refresh(), false);
    assert.match(f.elements.reviewPdfFilterStatus.textContent, /database error/);
    assert.throws(() => filters.assertReady(), /unavailable/);
    assert.equal(filters.getSelection().emission.length, 2);
    f.setResponse({ ok: true, options: { emission: ['Revenue', ''], format: ['New Format'] } });
    await filters.refresh();
    filters.assertReady();
    assert.ok(f.elements.reviewPdfformatChoices.children.some(label => label.children[1].textContent === 'New Format'));
});

test('storage failures and invalid saved filters are visible rather than silently remembered', async () => {
    const f = fixture(new Map([['reviewui.pdf.colnectFilters.Country A', '{"emission":42}']]));
    const filters = new f.context.Filters('Country A');
    await filters.init();
    assert.match(f.elements.reviewPdfFilterStatus.textContent, /Could not restore/);
    f.context.localStorage.setItem = () => { throw new Error('storage blocked'); };
    filters.change('emission', 'Official', true);
    assert.equal(filters.getSelection().emission[0], 'Official');
    assert.match(f.elements.reviewPdfFilterStatus.textContent, /could not be remembered.*storage blocked/);
});
