const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { test } = require('node:test');

test('version helper supports sh and Bash without modifying dry-run fixtures', () => {
    const workspace = fs.mkdtempSync(path.join(os.tmpdir(), 'version shell test '));
    try {
        fs.mkdirSync(path.join(workspace, 'scripts'));
        fs.mkdirSync(path.join(workspace, '.github'));
        const script = path.join(workspace, 'scripts', 'update_versions.sh');
        fs.copyFileSync(path.join(__dirname, '..', 'scripts', 'update_versions.sh'), script);
        fs.chmodSync(script, 0o755);
        const files = {
            'Cargo.toml': '[package]\nname = "version_fixture"\nversion = "1.2.3"\n',
            'README.md': '# Version: 1.2.3\n',
            'RELEASE_NOTES.md': '# Stamp Album Generator v1.2.3 Release Notes\n',
            '.github/instructions': '# Version: 1.2.3\n',
        };
        for (const [name, content] of Object.entries(files)) {
            fs.writeFileSync(path.join(workspace, name), content);
        }
        const git = spawnSync('git', ['init', '--quiet', workspace], { encoding: 'utf8' });
        assert.equal(git.status, 0, git.stderr);

        // Relative paths and spaces exercise argument/path preservation on re-exec.
        for (const shell of ['sh', 'bash']) {
            for (const [args, expected] of [
                [['minor'], '1.3.0'],
                [['major'], '2.0.0'],
                [['patch'], '1.2.4'],
                [['--set', '2.4.6'], '2.4.6'],
            ]) {
                const result = spawnSync(shell,
                    ['scripts/update_versions.sh', ...args, '--dry-run'],
                    { cwd: workspace, encoding: 'utf8' });
                assert.equal(result.status, 0, `${shell}: ${result.stderr}`);
                assert.ok(result.stdout.includes(`New version: ${expected}`), result.stdout);
                assert.ok(result.stdout.includes('Dry-run mode: no files were modified.'));
            }
            const help = spawnSync(shell, [script, '--help'],
                { cwd: os.tmpdir(), encoding: 'utf8' });
            assert.equal(help.status, 0, help.stderr);
            assert.ok(help.stdout.includes('Usage:'));
            const invalid = spawnSync(shell, [script, '--set', 'invalid', '--dry-run'],
                { cwd: workspace, encoding: 'utf8' });
            assert.equal(invalid.status, 1, invalid.stderr);
            assert.ok(invalid.stderr.includes('--set requires semantic version X.Y.Z'));
        }
        for (const [name, content] of Object.entries(files)) {
            assert.equal(fs.readFileSync(path.join(workspace, name), 'utf8'), content);
        }
        assert.equal(fs.existsSync(path.join(workspace, 'docs')), false);
        assert.equal(fs.existsSync(path.join(workspace, 'target')), false);
    } finally {
        fs.rmSync(workspace, { recursive: true, force: true });
    }
});
