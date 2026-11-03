const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { test } = require('node:test');

const workflow = fs.readFileSync(
    path.join(__dirname, '..', '.github', 'workflows', 'release-combined.yml'), 'utf8');
const validation = workflow.match(/          python3 - <<'PY'\n([\s\S]*?)          PY\n/);
assert.ok(validation, 'Release tag validation must exist');
const script = `python3 - <<'PY'\n${validation[1].replace(/^          /gm, '')}PY\n`;

test('release workflow builds existing commits without source publication', () => {
    assert.doesNotMatch(workflow, /prepare-version|new_version|version_type|cargo generate-lockfile/);
    assert.doesNotMatch(workflow, /git (?:add|commit|push|tag)\b/);
    assert.equal(
        workflow.match(/ref: \$\{\{ needs.validate-release.outputs.source_sha \}\}/g)?.length, 2);
    assert.match(workflow, /tag_name: \$\{\{ needs.validate-release.outputs.release_tag \}\}/);
    assert.match(workflow, /group: release-\$\{\{ inputs.release_tag \}\}/);
    assert.match(workflow, /cancel-in-progress: false/);
});

for (const scenario of [
    { tag: 'v1.2.4', version: '1.2.4', annotated: true },
    { tag: 'v1.2.4', version: '1.2.4', annotated: false },
    { tag: 'v1.2.5', version: '1.2.4', error: /does not exist/ },
    { tag: 'v1.2.4', version: '1.2.3', error: /does not match Cargo package version/ },
    { tag: 'v1.2.4;echo unsafe', version: '1.2.4', error: /must have the form/ },
    { tag: 'v01.2.4', version: '1.2.4', error: /must have the form/ },
]) {
    test(`validate ${scenario.tag}, package ${scenario.version}, annotated ${scenario.annotated}`, () => {
        const workspace = fs.mkdtempSync(path.join(os.tmpdir(), 'release-tag-'));
        const git = (...args) => {
            const result = spawnSync('git', args, { cwd: workspace, encoding: 'utf8' });
            assert.equal(result.status, 0, result.stderr);
            return result.stdout.trim();
        };
        try {
            git('init', '-q');
            git('config', 'user.name', 'Release Test');
            git('config', 'user.email', 'release-test@example.invalid');
            fs.writeFileSync(path.join(workspace, 'Cargo.toml'),
                `[package]\nname = "fixture"\nversion = "${scenario.version}"\n`);
            git('add', 'Cargo.toml');
            git('-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Fixture');
            if (scenario.annotated) {
                git('tag', '-a', 'v1.2.4', '-m', 'Fixture release');
            } else {
                git('tag', 'v1.2.4');
            }
            const sourceSha = git('rev-parse', 'HEAD');
            // Validate the tagged version rather than the workflow branch's version.
            fs.writeFileSync(path.join(workspace, 'Cargo.toml'),
                '[package]\nname = "fixture"\nversion = "9.0.0"\n');
            git('add', 'Cargo.toml');
            git('-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Later version');
            const before = git('show-ref');
            const output = path.join(workspace, '.outputs');
            for (let attempt = 0; attempt < 2; attempt++) {
                fs.writeFileSync(output, '');
                const result = spawnSync('bash', ['-e', '-c', script], {
                    cwd: workspace, encoding: 'utf8',
                    env: { ...process.env, RELEASE_TAG: scenario.tag, GITHUB_OUTPUT: output },
                });
                if (scenario.error) {
                    assert.notEqual(result.status, 0);
                    assert.match(result.stderr, scenario.error);
                    assert.equal(fs.readFileSync(output, 'utf8'), '');
                } else {
                    assert.equal(result.status, 0, result.stderr);
                    assert.equal(fs.readFileSync(output, 'utf8'),
                        `release_tag=v1.2.4\nsource_sha=${sourceSha}\n`);
                }
                assert.equal(git('show-ref'), before, 'Validation must not modify refs');
                assert.equal(git('diff', '--exit-code'), '', 'Validation must not modify files');
            }
        } finally {
            fs.rmSync(workspace, { recursive: true, force: true });
        }
    });
}
