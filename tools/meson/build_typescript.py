#!/usr/bin/env python3
"""Compile TypeScript sources to JavaScript for Meson targets."""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path


def find_tsc() -> list[str]:
    tsc = shutil.which('tsc')
    if tsc:
        return [tsc]
    npx = shutil.which('npx')
    if npx:
        return [npx, '-p', 'typescript', 'tsc']
    return []


def main() -> int:
    parser = argparse.ArgumentParser(description='Compile TypeScript sources to JavaScript')
    parser.add_argument('--root-dir', type=Path, default=None, help='Project or asset root for sources')
    parser.add_argument('--out-dir', type=Path, required=True, help='Output directory for compiled JS')
    parser.add_argument('sources', type=Path, nargs='+', help='TypeScript source files')
    args = parser.parse_args()

    tsc_cmd = find_tsc()
    if not tsc_cmd:
        raise SystemExit('TypeScript compiler (tsc or npx) not found in PATH')

    args.out_dir.mkdir(parents=True, exist_ok=True)

    cmd = list(tsc_cmd)
    cmd.extend(['--target', 'es2020', '--module', 'preserve', '--skipLibCheck'])
    if args.root_dir:
        cmd.extend(['--rootDir', str(args.root_dir.resolve())])
    cmd.extend(['--outDir', str(args.out_dir.resolve())])
    cmd.extend([str(s.resolve()) for s in args.sources])

    subprocess.run(cmd, check=True)

    # Post-process only the compiled output files for browser/device compatibility
    for source in args.sources:
        js_file = args.out_dir / f'{source.stem}.js'
        if js_file.is_file():
            try:
                content = js_file.read_text(encoding='utf-8')
                cleaned = content.replace('export {};', '').strip() + '\n'
                if cleaned != content:
                    js_file.write_text(cleaned, encoding='utf-8')
            except OSError:
                pass

    print(f'TypeScript: compiled {len(args.sources)} file(s) into {args.out_dir}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
