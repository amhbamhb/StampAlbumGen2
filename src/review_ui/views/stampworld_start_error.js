function showStampWorldStartError(message) {
    const reason = String(message || 'Failed to start StampWorld pull.');
    const needsCredentials = /credentials|site accounts|register/i.test(reason);
    let dialog = document.getElementById('stampworldStartErrorDialog');
    if (!dialog) {
        dialog = document.createElement('dialog');
        dialog.id = 'stampworldStartErrorDialog';
        dialog.setAttribute('aria-labelledby', 'stampworldStartErrorTitle');
        dialog.setAttribute('aria-describedby', 'stampworldStartErrorReason');
        dialog.style.cssText = 'padding:20px;border-radius:8px;border:1px solid #ccc;max-width:540px;width:90%;box-sizing:border-box;';
        const title = document.createElement('h3');
        title.id = 'stampworldStartErrorTitle';
        title.textContent = 'StampWorld pull could not start';
        const explanation = document.createElement('p');
        explanation.id = 'stampworldStartErrorReason';
        explanation.style.cssText = 'white-space:pre-wrap;overflow-wrap:anywhere;';
        const actions = document.createElement('div');
        actions.style.cssText = 'display:flex;gap:8px;justify-content:flex-end;';
        const dismiss = document.createElement('button');
        dismiss.id = 'stampworldStartErrorDismiss';
        dismiss.type = 'button';
        dismiss.onclick = () => dialog.close();
        const accounts = document.createElement('button');
        accounts.id = 'stampworldStartErrorAccounts';
        accounts.type = 'button';
        accounts.className = 'primary';
        accounts.textContent = 'Enter Credentials';
        accounts.onclick = () => {
            const accountReason = document.getElementById('stampworldStartErrorReason').textContent;
            dialog.close();
            openSiteAccountsDialog(accountReason);
        };
        actions.append(dismiss, accounts);
        dialog.append(title, explanation, actions);
        document.body.appendChild(dialog);
    }
    document.getElementById('stampworldStartErrorReason').textContent = reason;
    document.getElementById('stampworldStartErrorAccounts').hidden = !needsCredentials;
    document.getElementById('stampworldStartErrorDismiss').textContent = needsCredentials ? 'Not Now' : 'Close';
    if (!dialog.open) dialog.showModal();
}
