#!/usr/bin/env python3
"""Report Android.mk modules that still need a native Meson definition."""

from __future__ import annotations

import sys
from pathlib import Path


def classify(text: str) -> str:
    if 'BUILD_CTS_PACKAGE' in text:
        return 'android-package'
    if 'BUILD_HOST_JAVA_LIBRARY' in text:
        return 'host-java'
    if 'BUILD_SHARED_LIBRARY' in text:
        return 'android-shared-library'
    if 'BUILD_STATIC_LIBRARY' in text:
        return 'android-static-library'
    if 'BUILD_HOST_EXECUTABLE' in text:
        return 'host-executable'
    if 'BUILD_EXECUTABLE' in text:
        return 'android-executable'
    return 'other'


def main() -> int:
    root = Path(sys.argv[1]).resolve()
    counts: dict[str, int] = {}
    modules: list[tuple[str, str]] = []

    for makefile in sorted(root.rglob('Android.mk')):
        relative_path = makefile.relative_to(root)
        if relative_path.parts[:2] == ('cst-kitkat', 'cts'):
            continue
        kind = classify(makefile.read_text(errors='replace'))
        counts[kind] = counts.get(kind, 0) + 1
        modules.append((kind, str(relative_path)))

    print('Android.mk migration audit')
    print(f'total: {len(modules)}')
    for kind in sorted(counts):
        print(f'{kind}: {counts[kind]}')
    print('')
    for kind, path in modules:
        print(f'{kind}\t{path}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
