const assert = require('node:assert/strict');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { test } = require('node:test');

test('all workspace Cargo binary sources are tracked in Git', () => {
    const root = path.resolve(__dirname, '..');
    const metadata = spawnSync('cargo', [
        'metadata', '--no-deps', '--format-version', '1', '--offline',
    ], { cwd: root, encoding: 'utf8' });
    assert.equal(metadata.status, 0, metadata.stderr);
    const { packages, workspace_members } = JSON.parse(metadata.stdout);

    for (const pkg of packages.filter(pkg => workspace_members.includes(pkg.id))) {
        for (const target of pkg.targets.filter(target => target.kind.includes('bin'))) {
            const source = path.relative(root, target.src_path);
            const tracked = spawnSync('git', [
                'ls-files', '--error-unmatch', '--', source,
            ], { cwd: root, encoding: 'utf8' });
            assert.equal(tracked.status, 0,
                `${pkg.name} target ${target.name} requires an untracked source: ${source}`);
        }
    }
});
