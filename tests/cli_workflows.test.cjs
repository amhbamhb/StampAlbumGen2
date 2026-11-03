const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { test } = require('node:test');

const root = path.resolve(__dirname, '..');

for (const name of ['cli-parity.yml', 'verify-cli-docs.yml']) {
    test(`${name} uses headless Cargo builds and runs`, () => {
        const workflow = fs.readFileSync(
            path.join(root, '.github', 'workflows', name), 'utf8');
        const commands = [...workflow.matchAll(/^\s+run:\s+(cargo (?:build|run)\b[^\r\n]*)/gm)]
            .map(match => match[1]);
        assert.ok(commands.length > 0, 'Expected Cargo commands in workflow');
        for (const command of commands) {
            const cargoArgs = command.split(/\s+--(?:\s|$)/, 1)[0].split(/\s+/);
            assert.ok(cargoArgs.includes('--no-default-features'),
                `Native dialogs must be disabled: ${command}`);
            assert.ok(!cargoArgs.includes('--all-features') && !cargoArgs.includes('--features'),
                `Headless CLI checks must not enable native-dialog features: ${command}`);
        }
    });
}

test('headless Linux dependency graph excludes native dialogs and Wayland', () => {
    const result = spawnSync('cargo', [
        'tree', '--no-default-features',
        '--target', 'x86_64-unknown-linux-gnu', '--prefix', 'none',
    ], { cwd: root, encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr);
    assert.doesNotMatch(result.stdout, /^(?:rfd|wayland-[\w-]+) v/m);
});
