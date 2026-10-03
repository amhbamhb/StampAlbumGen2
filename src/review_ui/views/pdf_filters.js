class ReviewPdfFilters {
    constructor(country) {
        this.storageKey = 'reviewui.pdf.colnectFilters.' + country;
        this.selection = { emission: [], format: [] };
        this.options = { emission: [], format: [] };
        this.ready = false;
        this.disabled = false;
        this.requestId = 0;
        this.storageWarning = '';
    }

    status(message) {
        const element = document.getElementById('reviewPdfFilterStatus');
        if (element) element.textContent = [message, this.storageWarning].filter(Boolean).join(' ');
    }

    async init() {
        try {
            const saved = JSON.parse(localStorage.getItem(this.storageKey) || 'null');
            if (saved !== null) {
                for (const field of ['emission', 'format']) {
                    if (!Array.isArray(saved[field]) || saved[field].some(value => typeof value !== 'string')) {
                        throw new Error('Invalid saved PDF filters');
                    }
                }
                this.selection = { emission: saved.emission, format: saved.format };
            }
        } catch (error) {
            this.storageWarning = `Could not restore PDF filters; using All. ${error.message}`;
        }
        this.render();
        document.getElementById('reviewPdfRefreshFilters')?.addEventListener('click', () => this.refresh());
        await this.refresh();
    }

    async refresh() {
        const requestId = ++this.requestId;
        this.ready = false;
        this.status('Loading country filter choices...');
        try {
            const response = await fetch('/api/review-pdf-options', { cache: 'no-store' });
            const result = await response.json();
            if (!response.ok || result.ok !== true) {
                throw new Error(result.message || `HTTP ${response.status}`);
            }
            for (const field of ['emission', 'format']) {
                if (!Array.isArray(result.options?.[field])
                    || result.options[field].some(value => typeof value !== 'string')) {
                    throw new Error('Invalid country filter choices');
                }
            }
            if (requestId !== this.requestId) return false;
            this.options = result.options;
            this.ready = true;
            this.render();
            const unavailable = ['emission', 'format'].some(field =>
                this.selection[field].some(selected => !this.options[field].some(value =>
                    this.normalize(value) === this.normalize(selected))));
            this.status(unavailable
                ? 'Some remembered choices are no longer available; they remain selected. Review them or choose All.'
                : 'Choices come from this country\'s album records. Select several values, or All.');
            return true;
        } catch (error) {
            if (requestId !== this.requestId) return false;
            this.status(`Could not load PDF filter choices: ${error.message}. Use Refresh filter choices to retry.`);
            return false;
        }
    }

    normalize(value) {
        return value.trim().toLowerCase();
    }

    getSelection() {
        return { emission: [...this.selection.emission], format: [...this.selection.format] };
    }

    assertReady() {
        if (!this.ready) throw new Error('PDF filter choices are unavailable. Refresh them before generating.');
    }

    change(field, value, checked) {
        if (this.disabled) return;
        if (value === null) {
            this.selection[field] = [];
        } else {
            const remaining = this.selection[field].filter(item => this.normalize(item) !== this.normalize(value));
            this.selection[field] = checked ? [...remaining, value] : remaining;
        }
        try {
            localStorage.setItem(this.storageKey, JSON.stringify(this.selection));
            this.storageWarning = '';
        } catch (error) {
            this.storageWarning = `Selections apply now but could not be remembered: ${error.message}`;
        }
        this.render();
        this.status('PDF filters updated. Values within a field use OR; Emission and Format use AND.');
    }

    setDisabled(disabled) {
        this.disabled = disabled;
        for (const field of ['emission', 'format']) {
            const container = document.getElementById(`reviewPdf${field}Choices`);
            container?.querySelectorAll('input').forEach(input => { input.disabled = disabled; });
        }
        const refresh = document.getElementById('reviewPdfRefreshFilters');
        if (refresh) refresh.disabled = disabled;
    }

    render() {
        for (const field of ['emission', 'format']) {
            const container = document.getElementById(`reviewPdf${field}Choices`);
            const summary = document.getElementById(`reviewPdf${field}Summary`);
            if (!container || !summary) continue;
            const selected = this.selection[field];
            const name = field === 'emission' ? 'Emission' : 'Format';
            summary.textContent = `${name}: ${selected.length ? selected.map(value => value || 'Unknown / not recorded').join(', ') : 'All'}`;
            container.replaceChildren();
            const addChoice = (value, text, checked) => {
                const label = document.createElement('label');
                const input = document.createElement('input');
                input.type = 'checkbox';
                input.checked = checked;
                input.disabled = this.disabled;
                input.addEventListener('change', () => this.change(field, value, input.checked));
                label.append(input, document.createTextNode(text));
                container.append(label);
            };
            addChoice(null, 'All (default)', selected.length === 0);
            const values = [...this.options[field]];
            for (const value of selected) {
                if (!values.some(item => this.normalize(item) === this.normalize(value))) values.push(value);
            }
            for (const value of values) {
                const available = this.options[field].some(item => this.normalize(item) === this.normalize(value));
                const label = (value || 'Unknown / not recorded') + (available ? '' : ' (not currently available)');
                addChoice(value, label, selected.some(item => this.normalize(item) === this.normalize(value)));
            }
        }
    }
}
