const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { test } = require('node:test');

const root = path.resolve(__dirname, '..');
const workflow = fs.readFileSync(path.join(root, '.github/workflows/release-combined.yml'), 'utf8');

function command(cwd, executable, args, env = {}) {
    return spawnSync(executable, args, {
        cwd, encoding: 'utf8', env: { ...process.env, ...env },
    });
}

function success(result) {
    assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
    return result.stdout.trim();
}

function fixture(t, version = '1.13.2', highestTag = 'v1.14.0') {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'automatic-release-'));
    t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
    const repo = path.join(dir, 'source');
    const remote = path.join(dir, 'origin.git');
    const git = (cwd, ...args) => success(command(cwd, 'git',
        cwd === remote ? ['--git-dir', remote, ...args] : args));
    fs.mkdirSync(repo);
    git(repo, 'init', '-q', '-b', 'main');
    git(repo, 'config', 'user.name', 'Release Test');
    git(repo, 'config', 'user.email', 'release-test@example.invalid');
    fs.mkdirSync(path.join(repo, 'scripts'));
    fs.mkdirSync(path.join(repo, '.github'));
    fs.mkdirSync(path.join(repo, 'src'));
    for (const script of ['update_versions.sh', 'prepare_release.sh', 'publish_release_source.sh']) {
        fs.copyFileSync(path.join(root, 'scripts', script), path.join(repo, 'scripts', script));
    }
    fs.writeFileSync(path.join(repo, 'Cargo.toml'),
        `[package]\nname = "release_fixture"\nversion = "${version}"\nedition = "2021"\n`);
    fs.writeFileSync(path.join(repo, 'src/main.rs'), 'fn main() {}\n');
    fs.writeFileSync(path.join(repo, 'README.md'), `# Stamp Album Generator v${version}\n`);
    fs.writeFileSync(path.join(repo, 'RELEASE_NOTES.md'),
        `# Stamp Album Generator v${version} Release Notes\n`);
    fs.writeFileSync(path.join(repo, '.github/instructions'), `# Version: ${version}\n`);
    success(command(repo, 'cargo', ['generate-lockfile', '--offline']));
    git(repo, 'add', '.');
    git(repo, '-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Fixture source');
    git(dir, 'init', '--bare', '-q', '-b', 'main', remote);
    git(repo, 'remote', 'add', 'origin', remote);
    git(repo, 'push', '-q', 'origin', 'main');
    if (highestTag) {
        git(repo, 'tag', highestTag);
        git(repo, 'push', '-q', 'origin', highestTag);
    }
    const env = {
        GITHUB_RUN_ID: '12345', RUNNER_TEMP: dir, GITHUB_OUTPUT: path.join(dir, 'outputs'),
        RELEASE_BRANCH: 'main', VERSION_TYPE: 'patch',
    };
    const run = (cwd, script, extra = {}) => {
        fs.writeFileSync(env.GITHUB_OUTPUT, '');
        const result = command(cwd, 'bash', [`scripts/${script}`], { ...env, ...extra });
        const outputs = Object.fromEntries(fs.readFileSync(env.GITHUB_OUTPUT, 'utf8').trim()
            .split('\n').filter(Boolean).map(line => {
                const index = line.indexOf('=');
                return [line.slice(0, index), line.slice(index + 1)];
            }));
        return { result, outputs };
    };
    const clone = (name, sha) => {
        const target = path.join(dir, name);
        git(dir, 'clone', '-q', remote, target);
        git(target, 'config', 'user.name', 'Release Test');
        git(target, 'config', 'user.email', 'release-test@example.invalid');
        if (sha) git(target, 'checkout', '-q', '--detach', sha);
        return target;
    };
    const prepare = (cwd = repo, extra) => {
        const prepared = run(cwd, 'prepare_release.sh', extra);
        success(prepared.result);
        return prepared.outputs;
    };
    const publish = (cwd, outputs) => run(cwd, 'publish_release_source.sh', {
        RELEASE_TAG: outputs.release_tag, BASE_SHA: outputs.base_sha,
        TREE_SHA: outputs.tree_sha, REUSED_SHA: outputs.reused_sha,
    });
    return { dir, repo, remote, git, run, clone, prepare, publish };
}

test('one-button workflow tests both platforms before source publication', () => {
    assert.match(workflow, /default: patch/);
    assert.match(workflow, /options: \[patch, minor, major\]/);
    assert.doesNotMatch(workflow, /inputs\.release_tag|validate-release|generate-lockfile/);
    assert.match(workflow, /group: application-release/);
    assert.match(workflow, /cancel-in-progress: false/);
    assert.match(workflow, /publish-source:[\s\S]*?needs: \[prepare-release, build, windows-sign\]/);
    assert.match(workflow, /cargo test --release --workspace --lib --no-default-features --locked/);
    assert.match(workflow, /cargo test --release --workspace --lib --features rfd --locked/);
    assert.match(workflow, /test "\$\(git write-tree\)" = "\$TREE_SHA"/);
    assert.match(workflow, /ref: \$\{\{ needs.publish-source.outputs.source_sha \}\}/);
    assert.match(workflow, /Download unsigned Windows artifact/);
    assert.match(workflow, /already published; keeping its assets unchanged/);
    assert.doesNotMatch(workflow, /continue-on-error|softprops|find .*rm -f/);
});

for (const [increment, expected] of [['patch', 'v1.14.1'], ['minor', 'v1.15.0'], ['major', 'v2.0.0']]) {
    test(`${increment} prepares ${expected} above existing tags without publication`, t => {
        const f = fixture(t);
        const before = f.git(f.remote, 'show-ref');
        const outputs = f.prepare(f.repo, { VERSION_TYPE: increment });
        assert.equal(outputs.release_tag, expected);
        assert.equal(outputs.reused_sha, '');
        assert.equal(f.git(f.remote, 'show-ref'), before);
        assert.equal(f.git(f.repo, 'rev-parse', 'HEAD'), outputs.base_sha);
        assert.equal(f.git(f.repo, 'write-tree'), outputs.tree_sha);
        assert.match(fs.readFileSync(path.join(f.repo, 'Cargo.lock'), 'utf8'),
            new RegExp(`version = "${expected.slice(1).replaceAll('.', '\\.')}"`));
        assert.equal(fs.existsSync(path.join(f.repo, 'docs/versions')), false);
        const build = f.clone('build', outputs.base_sha);
        f.git(build, 'apply', '--index', path.join(f.dir, 'release-source/release-source.patch'));
        assert.equal(f.git(build, 'write-tree'), outputs.tree_sha);
        success(command(build, 'cargo', ['metadata', '--no-deps', '--format-version', '1', '--offline', '--locked']));
    });
}

test('package version takes precedence over older tags', t => {
    const f = fixture(t, '2.0.0', 'v1.14.0');
    assert.equal(f.prepare().release_tag, 'v2.0.1');
});

test('publishes the tested tree atomically and reuses the same version on retry', t => {
    const f = fixture(t);
    const outputs = f.prepare();
    const publish = f.clone('publish', outputs.base_sha);
    const published = f.publish(publish, outputs);
    success(published.result);
    const sha = published.outputs.source_sha;
    assert.equal(f.git(f.remote, 'rev-parse', 'main'), sha);
    assert.equal(f.git(f.remote, 'rev-parse', `${outputs.release_tag}^{commit}`), sha);
    assert.equal(f.git(f.remote, 'rev-parse', `${sha}^{tree}`), outputs.tree_sha);
    const advance = f.clone('advance-after-publication');
    fs.appendFileSync(path.join(advance, 'README.md'), '\nLater source change\n');
    f.git(advance, 'add', 'README.md');
    f.git(advance, '-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Later source');
    f.git(advance, 'push', '-q', 'origin', 'main');
    const advancedSha = f.git(f.remote, 'rev-parse', 'main');
    const retry = f.clone('retry');
    const reused = f.prepare(retry);
    assert.equal(reused.release_tag, outputs.release_tag);
    assert.equal(reused.reused_sha, sha);
    assert.equal(reused.tree_sha, outputs.tree_sha);
    const retryBuild = f.clone('retry-build', reused.base_sha);
    f.git(retryBuild, 'apply', '--index', path.join(f.dir, 'release-source/release-source.patch'));
    assert.equal(f.git(retryBuild, 'write-tree'), outputs.tree_sha);
    const retryPublish = f.clone('retry-publish', reused.base_sha);
    const retried = f.publish(retryPublish, reused);
    success(retried.result);
    assert.equal(retried.outputs.source_sha, sha);
    assert.equal(f.git(f.remote, 'rev-parse', 'main'), advancedSha, 'Retry must not rewind main');
    const next = f.clone('next-run');
    assert.equal(f.prepare(next, { GITHUB_RUN_ID: '12346' }).release_tag, 'v1.14.2');
});

test('rejected publication fails without publishing branch or tag', t => {
    const f = fixture(t);
    const outputs = f.prepare();
    const before = f.git(f.remote, 'show-ref');
    const hook = path.join(f.remote, 'hooks/pre-receive');
    fs.writeFileSync(hook, '#!/bin/sh\necho "test rejection" >&2\nexit 1\n', { mode: 0o755 });
    const published = f.publish(f.clone('publish', outputs.base_sha), outputs);
    assert.notEqual(published.result.status, 0);
    assert.match(published.result.stderr, /test rejection/);
    assert.deepEqual(published.outputs, {});
    assert.equal(f.git(f.remote, 'show-ref'), before);
});

test('advancing main during a build rejects publication without creating a tag', t => {
    const f = fixture(t);
    const outputs = f.prepare();
    const advance = f.clone('advance');
    fs.appendFileSync(path.join(advance, 'README.md'), '\nAnother change\n');
    f.git(advance, 'add', 'README.md');
    f.git(advance, '-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Concurrent change');
    f.git(advance, 'push', '-q', 'origin', 'main');
    const before = f.git(f.remote, 'show-ref');
    const published = f.publish(f.clone('publish', outputs.base_sha), outputs);
    assert.notEqual(published.result.status, 0);
    assert.deepEqual(published.outputs, {});
    assert.equal(f.git(f.remote, 'show-ref'), before);
});

test('tampering with prepared source prevents source publication', t => {
    const f = fixture(t);
    const outputs = f.prepare();
    fs.appendFileSync(path.join(f.dir, 'release-source/release-source.patch'), '\n');
    outputs.tree_sha = '0000000000000000000000000000000000000000';
    const before = f.git(f.remote, 'show-ref');
    const published = f.publish(f.clone('publish', outputs.base_sha), outputs);
    assert.notEqual(published.result.status, 0);
    assert.match(published.result.stderr, /differs from the tree tested/);
    assert.equal(f.git(f.remote, 'show-ref'), before);
});

test('invalid manifests fail preparation without publishing anything', t => {
    const f = fixture(t);
    fs.appendFileSync(path.join(f.repo, 'Cargo.toml'), '\n<<<<<<< HEAD\n');
    f.git(f.repo, 'add', 'Cargo.toml');
    f.git(f.repo, '-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Invalid fixture');
    const before = f.git(f.remote, 'show-ref');
    const prepared = f.run(f.repo, 'prepare_release.sh');
    assert.notEqual(prepared.result.status, 0);
    assert.match(prepared.result.stderr, /invalid key|expected|invalid/);
    assert.deepEqual(prepared.outputs, {});
    assert.equal(f.git(f.remote, 'show-ref'), before);
});

test('preparation-only mode rejects publication flags', t => {
    const f = fixture(t);
    const result = command(f.repo, 'bash', [
        'scripts/update_versions.sh', '--prepare-only', '--git',
    ]);
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /cannot be combined/);
});

test('release preparation requires a committed lockfile', t => {
    const f = fixture(t);
    f.git(f.repo, 'rm', 'Cargo.lock');
    f.git(f.repo, '-c', 'core.hooksPath=/dev/null', 'commit', '-qm', 'Unpinned fixture');
    const prepared = f.run(f.repo, 'prepare_release.sh');
    assert.notEqual(prepared.result.status, 0);
    assert.match(prepared.result.stderr, /Cargo.lock must be committed/);
    assert.deepEqual(prepared.outputs, {});
});

test('a tag created by another run is never overwritten', t => {
    const f = fixture(t);
    const outputs = f.prepare();
    f.git(f.repo, 'tag', outputs.release_tag);
    f.git(f.repo, 'push', '-q', 'origin', outputs.release_tag);
    const before = f.git(f.remote, 'show-ref');
    const published = f.publish(f.clone('publish', outputs.base_sha), outputs);
    assert.notEqual(published.result.status, 0);
    assert.match(published.result.stderr, /does not belong to this prepared release/);
    assert.equal(f.git(f.remote, 'show-ref'), before);
});

const upload = workflow.split('      - name: Upload assets and publish release (retry safe)')[1]
    ?.split('        run: |\n')[1]?.replace(/^          /gm, '');
assert.ok(upload, 'Release asset publication must exist');

for (const scenario of [
    { status: '404', draft: 'true', commands: ['create', 'upload', 'edit'] },
    { status: '200', draft: 'true', commands: ['upload', 'edit'] },
    { status: '200', draft: 'false', commands: [] },
    { status: '403', draft: 'false', commands: [], failure: true },
    { status: '404', draft: 'true', commands: ['create', 'upload'], failCommand: 'upload', failure: true },
    { status: '200', draft: 'true', commands: ['upload', 'edit'], failCommand: 'edit', failure: true },
]) {
    test(`release assets: HTTP ${scenario.status}, draft ${scenario.draft}, failure ${scenario.failCommand}`, t => {
        const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'release-assets-'));
        t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
        const log = path.join(dir, 'commands');
        fs.writeFileSync(log, '');
        const result = command(dir, 'bash', ['-e', '-c', `
            curl() {
                printf '{"draft": %s}' "$TEST_DRAFT" > "$RUNNER_TEMP/release.json"
                printf '%s' "$TEST_STATUS"
            }
            gh() {
                printf '%s\\n' "$2" >> "$TEST_LOG"
                if [ "$2" = "$TEST_FAIL_COMMAND" ]; then
                    echo "simulated release API failure" >&2
                    return 1
                fi
            }
            ${upload}
        `], {
            RUNNER_TEMP: dir, TAG: 'v1.14.1', GH_TOKEN: 'test-placeholder',
            GITHUB_API_URL: 'https://example.invalid', GITHUB_REPOSITORY: 'fixture/release',
            GITHUB_STEP_SUMMARY: path.join(dir, 'summary'),
            TEST_LOG: log, TEST_STATUS: scenario.status, TEST_DRAFT: scenario.draft,
            TEST_FAIL_COMMAND: scenario.failCommand ?? '',
        });
        if (scenario.failure) {
            assert.notEqual(result.status, 0);
        } else {
            success(result);
        }
        assert.deepEqual(fs.readFileSync(log, 'utf8').trim().split('\n').filter(Boolean),
            scenario.commands);
    });
}
