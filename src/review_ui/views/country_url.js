class StampWorldCountryUrl {
    constructor(getCountry) {
        this.getCountry = getCountry;
        this.country = null;
        this.ready = false;
        this.dirty = false;
        this.saving = false;
        this.requestId = 0;
        this.input = document.getElementById('stampworldCountryUrl');
        this.button = document.getElementById('saveStampworldCountryUrl');
        this.status = document.getElementById('stampworldCountryUrlStatus');
        this.input?.addEventListener('input', () => {
            this.dirty = true;
            this.message('Unsaved URL. Save it now, or it will be saved before the next pull.');
        });
        this.button?.addEventListener('click', () => {
            this.ensureSaved(this.getCountry()).catch(error => this.message(error.message));
        });
    }

    message(text) {
        if (this.status) this.status.textContent = text;
    }

    load(country) {
        country = String(country || '').trim();
        if (country === this.country && this.ready) return Promise.resolve();
        if (country === this.country && this.loading) return this.loading;
        this.country = country;
        this.ready = false;
        this.dirty = false;
        const requestId = ++this.requestId;
        if (this.input) { this.input.value = ''; this.input.disabled = true; }
        if (this.button) this.button.disabled = true;
        if (!country || country === 'Unknown') {
            this.message('Select or enter a country to configure its StampWorld URL.');
            return Promise.resolve();
        }
        this.message('Loading saved StampWorld URL...');
        this.loading = (async () => {
            try {
                const response = await fetch('/api/stampworld-country-url?country=' + encodeURIComponent(country), { cache: 'no-store' });
                const data = await response.json();
                if (!response.ok || !data.ok) throw new Error(data.message || `HTTP ${response.status}`);
                if (typeof data.url !== 'string' || typeof data.custom !== 'boolean') throw new Error('Invalid country URL response');
                if (requestId !== this.requestId) return;
                this.ready = true;
                if (this.input) {
                    this.input.value = data.custom ? data.url : '';
                    this.input.placeholder = data.url;
                    this.input.disabled = false;
                }
                if (this.button) this.button.disabled = false;
                this.message(data.custom
                    ? `Saved URL for ${country}: ${data.url}`
                    : `Automatic URL for ${country}: ${data.url}`);
            } catch (error) {
                if (requestId === this.requestId) this.message('Could not load StampWorld URL: ' + error.message);
            } finally {
                if (requestId === this.requestId) this.loading = null;
            }
        })();
        return this.loading;
    }

    async ensureSaved(country) {
        country = String(country || '').trim();
        await this.load(country);
        if (!this.ready || this.country !== country) throw new Error('StampWorld URL settings are unavailable. Select the country again to retry.');
        if (String(this.getCountry() || '').trim() !== country) throw new Error('The selected country changed. Retry the pull for the current country.');
        if (this.saving) throw new Error('A country URL save is already in progress. Please wait and retry.');
        if (!this.dirty) return;
        const value = this.input?.value || '';
        this.saving = true;
        if (this.input) this.input.disabled = true;
        if (this.button) this.button.disabled = true;
        try {
            const response = await fetch('/api/stampworld-country-url', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ country, url: value })
            });
            const data = await response.json();
            if (!response.ok || !data.ok) throw new Error(data.message || `HTTP ${response.status}`);
            if (this.country !== country || String(this.getCountry() || '').trim() !== country) throw new Error('The selected country changed while saving. Retry for the current country.');
            this.dirty = false;
            this.message(data.custom ? `Saved URL for ${country}: ${data.url}` : `Automatic URL restored for ${country}: ${data.url}`);
            if (this.input) {
                this.input.value = data.custom ? data.url : '';
                this.input.placeholder = data.url;
            }
        } finally {
            this.saving = false;
            if (this.country === country) {
                if (this.input) this.input.disabled = false;
                if (this.button) this.button.disabled = false;
            }
        }
    }
}
