#!/usr/bin/env python3
"""
scripts/cargo_json_to_junit.py

Minimal converter from cargo --message-format=json output to a simple JUnit XML file.
This is a best-effort fallback for CI environments where cargo2junit is unavailable or doesn't detect results.

Usage:
  cargo test --workspace --message-format=json 2>&1 | tee target/cargo-json.log
  python3 scripts/cargo_json_to_junit.py target/cargo-json.log target/junit-report.xml

"""
import sys
import json
import xml.etree.ElementTree as ET
from collections import defaultdict

def parse_cargo_json_lines(path):
    tests = defaultdict(lambda: {"status": None, "output": []})
    import re
    try:
        with open(path, 'r', errors='ignore') as fh:
            in_failure_block = None
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                try:
                    obj = json.loads(line)
                except Exception:
                    # not JSON — try to parse plain-text test output lines
                    # Example lines: "test test_name ... ok" or "test test_name ... FAILED"
                    m = re.match(r'^test\s+(.+?)\s+\.\.\.\s+(ok|FAILED|ignored)$', line)
                    if m:
                        name = m.group(1)
                        ev = m.group(2)
                        if ev == 'ok' or ev == 'ignored':
                            tests[name]['status'] = 'ok'
                        else:
                            tests[name]['status'] = 'failed'
                        continue

                    # Capture failure stdout blocks like:
                    # "---- test_name stdout ----" followed by lines, ending at a blank line or next separator
                    fm = re.match(r'^----\s+(.+?)\s+stdout\s+----$', line)
                    if fm:
                        in_failure_block = fm.group(1)
                        # ensure test exists
                        _ = tests[in_failure_block]
                        continue

                    if in_failure_block:
                        # end block on empty line or a line of dashes
                        if line == '' or re.match(r'^-+$', line):
                            in_failure_block = None
                        else:
                            tests[in_failure_block]['output'].append(line + '\n')
                        continue
                    # otherwise skip non-JSON line
                    continue

                reason = obj.get('reason')
                if reason == 'test':
                    name = obj.get('name')
                    event = obj.get('event')
                    if event == 'ok':
                        tests[name]['status'] = 'ok'
                    elif event == 'failed':
                        tests[name]['status'] = 'failed'
                    elif event == 'started':
                        # ensure exists
                        _ = tests[name]
                elif reason == 'stdout' and 'rendered' in obj:
                    # appended output from test harness; try to attribute to last open tests
                    text = obj.get('rendered') or obj.get('text') or ''
                    # best-effort: append to all tests that have started but not finished
                    for name, info in tests.items():
                        if info['status'] is None:
                            info['output'].append(text)
                # Some cargo versions emit test messages using 'message' field
                if 'message' in obj and isinstance(obj['message'], str):
                    for name, info in tests.items():
                        if info['status'] is None:
                            info['output'].append(obj['message'])
    except FileNotFoundError:
        print(f"Input file not found: {path}", file=sys.stderr)
        return {}
    return tests

def write_junit(tests, outpath):
    testsuite = ET.Element('testsuite', name='cargo-tests', tests=str(len(tests)))
    for name, info in sorted(tests.items()):
        testcase = ET.SubElement(testsuite, 'testcase', classname='cargo', name=name)
        status = info.get('status') or 'unknown'
        out = ''.join(info.get('output') or [])
        if status != 'ok':
            failure = ET.SubElement(testcase, 'failure', message='test failed')
            failure.text = out or 'failure'
        if out:
            system_out = ET.SubElement(testcase, 'system-out')
            system_out.text = out

    tree = ET.ElementTree(testsuite)
    tree.write(outpath, encoding='utf-8', xml_declaration=True)
    print(f'Wrote JUnit XML to {outpath}')


def main():
    if len(sys.argv) != 3:
        print('Usage: cargo_json_to_junit.py <cargo-json-log> <output-junit-xml>', file=sys.stderr)
        sys.exit(2)
    infile = sys.argv[1]
    outfile = sys.argv[2]
    tests = parse_cargo_json_lines(infile)
    if not tests:
        print('No tests found in cargo JSON output.', file=sys.stderr)
    write_junit(tests, outfile)

if __name__ == '__main__':
    main()
