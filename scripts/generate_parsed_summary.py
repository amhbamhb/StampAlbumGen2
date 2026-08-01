#!/usr/bin/env python3
import re,glob,statistics,os,sys
PT_TO_MM = 0.352777778
out_path = "/tmp/wiki_parsed_vspace_summary.txt"
files = sorted(glob.glob('/tmp/Switzerland-1972-*.txt'))
with open(out_path,'w',encoding='utf-8') as outf:
    if not files:
        outf.write('No generated .txt files found under /tmp\n')
        print('No generated .txt files found under /tmp')
        sys.exit(0)
    outf.write('Parsed generated .txt files for vspace and heuristic percent-used\n\n')
    for f in files:
        txt = open(f,'r',encoding='utf-8',errors='ignore').read()
        # parse header values
        page_h = 279.4
        top = 10.0
        bottom = 10.0
        title_pt = 10
        foot_pt = 8
        m = re.search(r'^ALBUM_PAGES_SIZE\s*\(([^)]+)\)', txt, re.M)
        if m:
            parts = m.group(1).split()
            try:
                page_h = float(parts[1])
            except Exception:
                pass
        m = re.search(r'^ALBUM_PAGES_MARGINS\s*\(([^)]+)\)', txt, re.M)
        if m:
            parts = m.group(1).split()
            try:
                top = float(parts[2]); bottom = float(parts[3])
            except Exception:
                pass
        m = re.search(r'^ALBUM_PAGES_HEADER\s*\(([^)]+)\)', txt, re.M)
        if m:
            parts = m.group(1).split()
            try:
                title_pt = int(float(parts[1]))
            except Exception:
                pass
        header_h = title_pt * PT_TO_MM * 1.4
        footer_h = foot_pt * PT_TO_MM * 1.2
        usable = page_h - top - bottom - header_h - footer_h

        pages = re.split(r"^PAGE_START\s*$", txt, flags=re.M)
        page_infos = []
        for pi, blk in enumerate(pages[1:], start=1):
            content_h = 0.0
            vspaces = [float(m.group(1)) for m in re.finditer(r'^PAGE_VSPACE\s*\(\s*([0-9.]+)\)', blk, re.M)]
            # stamp heights
            for m in re.finditer(r'^STAMP_ADD_IMG\s*\(([^)]+)\)', blk, re.M):
                parts = m.group(1).split()
                if len(parts) >= 2:
                    try:
                        h = float(parts[1])
                        content_h += h
                    except Exception:
                        pass
            # text lines
            for m in re.finditer(r'PAGE_TEXT[_A-Z]*\s*\(([^)]+)\)', blk, re.M):
                parts = m.group(1).strip().split()
                pt = None
                if len(parts) >= 2:
                    try:
                        pt = int(float(parts[1]))
                    except Exception:
                        pt = None
                if pt is None:
                    pt = title_pt
                content_h += pt * PT_TO_MM * 1.2
            content_h += sum(vspaces)
            percent = (content_h / usable * 100.0) if usable > 0 else 0.0
            page_infos.append({'idx':pi,'content_h':content_h,'percent':percent,'vspaces':vspaces})

        if page_infos:
            percents = [p['percent'] for p in page_infos]
            vspace_counts = sum(1 for p in page_infos if any(v>0 for v in p['vspaces']))
            pages_first_gt12 = [(p['idx'], p['vspaces'][0]) for p in page_infos if p['vspaces'] and p['vspaces'][0] > 12.0]
            pages_any_gt12 = [(p['idx'], max(p['vspaces'])) for p in page_infos if any(v>12.0 for v in p['vspaces'])]
            ranked = sorted([(p['idx'], max(p['vspaces']) if p['vspaces'] else 0.0) for p in page_infos], key=lambda x: -x[1])[:10]
            low = sorted(page_infos, key=lambda x: x['percent'])[:10]
            outf.write(f"File: {os.path.basename(f)}\n")
            outf.write(f" pages={len(page_infos)} usable_height={usable:.1f}mm mean%={statistics.mean(percents):.1f} median%={statistics.median(percents):.1f} min%={min(percents):.1f} max%={max(percents):.1f}\n")
            outf.write(f" pages_with_any_vspace={vspace_counts} pages_with_first_vspace_gt12={len(pages_first_gt12)} pages_with_any_vspace_gt12={len(pages_any_gt12)}\n")
            outf.write(" Top pages by vspace (page:val_mm): " + ', '.join([f"{i}:{v:.1f}" for i,v in ranked]) + "\n")
            outf.write(" Lowest percent-used pages (page:percent): " + ', '.join([f"{p['idx']}:{p['percent']:.1f}" for p in low]) + "\n\n")
        else:
            outf.write(f"File: {os.path.basename(f)}: no page infos found\n\n")

    # extract WIKI_PAGE_METRICS from forcemetrics runlogs
    outf.write('\nExtracting WIKI_PAGE_METRICS from forcemetrics runlogs:\n')
    logs = sorted(glob.glob('/tmp/Switzerland-1972-*.forcemetrics.runlog'))
    if not logs:
        outf.write(' No forcemetrics runlogs found under /tmp\n')
    else:
        for lg in logs:
            lines = open(lg,'r',encoding='utf-8',errors='ignore').read().splitlines()
            mets = []
            for L in lines:
                m = re.search(r'WIKI_PAGE_METRICS.*percent_used=([0-9.]+)', L)
                if m:
                    mets.append(float(m.group(1)))
            if not mets:
                outf.write(f" {os.path.basename(lg)} -> no WIKI_PAGE_METRICS lines found\n")
            else:
                outf.write(f" {os.path.basename(lg)}: pages={len(mets)} mean%={statistics.mean(mets):.1f} median%={statistics.median(mets):.1f} min%={min(mets):.1f} max%={max(mets):.1f} under60={sum(1 for v in mets if v<60)}\n")
                low_idx = sorted(range(len(mets)), key=lambda i: mets[i])[:5]
                outf.write('  5 lowest percent_used (page_idx,percent): ' + ', '.join([f"{i+1}:{mets[i]:.1f}" for i in low_idx]) + "\n")

print('Wrote', out_path)
