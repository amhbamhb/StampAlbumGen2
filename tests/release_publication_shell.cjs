const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { test } = require('node:test');

const workflow = fs.readFileSync(
    path.join(__dirname, '..', '.github', 'workflows', 'release-combined.yml'), 'utf8');
const publication = workflow.match(/^\s+git tag -a.*\n(?:[ ]{10}.+\n)*/m)?.[0];
assert.ok(publication, 'Release publication commands must exist');

for (const rejected of [false, true]) {
    test(`release publication ${rejected ? 'fails on rejection' : 'pushes branch and tag atomically'}`, () => {
        const workspace = fs.mkdtempSync(path.join(os.tmpdir(), 'release publication '));
        try {
            const log = path.join(workspace, 'git.log');
            const result = spawnSync('bash', ['-e', '-c', `
                git() {
                    printf '%s\\n' "$*" >> "$PUBLICATION_LOG"
                    if [ "$1" = push ] && [ "$REJECT_PUSH" = true ]; then
                        echo 'remote rejected: missing necessary objects' >&2
                        return 1
                    fi
                }
                NEW_VERSION=1.2.4
                ${publication}
            `], {
                encoding: 'utf8',
                env: {
                    ...process.env,
                    PUBLICATION_LOG: log,
                    REJECT_PUSH: String(rejected),
                },
            });
            const commands = fs.readFileSync(log, 'utf8').trim().split('\n');
            assert.deepEqual(commands, [
                'tag -a v1.2.4 -m Release v1.2.4',
                'push --atomic origin HEAD refs/tags/v1.2.4',
            ]);
            if (rejected) {
                assert.notEqual(result.status, 0, 'Rejected publication must fail preparation');
                assert.match(result.stderr, /missing necessary objects/);
            } else {
                assert.equal(result.status, 0, result.stderr);
            }
        } finally {
            fs.rmSync(workspace, { recursive: true, force: true });
        }
    });
}
