#!/usr/bin/env python3
"""Configure and build a standalone Meson project from the CTS tree."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--build', type=Path, required=True)
    parser.add_argument('--tests', choices=('true', 'false'))
    args = parser.parse_args()

    command = ['meson', 'setup']
    if (args.build / 'meson-private').is_dir():
        command.append('--reconfigure')
    command.extend([str(args.build), str(args.source)])
    if args.tests is not None:
        command.append(f'-Dtests={args.tests}')
    subprocess.run(command, check=True)
    subprocess.run(['meson', 'compile', '-C', str(args.build)], check=True)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
