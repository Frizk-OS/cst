#!/usr/bin/env python3
"""Build an Android APK through its Gradle wrapper for a Meson target."""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--sdk', type=Path, default=None)
    args = parser.parse_args()

    sdk = args.sdk or Path(os.environ.get('ANDROID_HOME', os.environ.get('ANDROID_SDK_ROOT', '')))
    if not sdk.is_dir():
        fallback = Path.home() / 'Android/Sdk'
        sdk = fallback if fallback.is_dir() else sdk
    if not sdk.is_dir():
        raise SystemExit('Set ANDROID_HOME/ANDROID_SDK_ROOT or pass --sdk=/path/to/sdk')

    wrapper = args.source / 'gradlew'
    if not wrapper.is_file():
        raise SystemExit(f'Missing Gradle wrapper: {wrapper}')

    environment = os.environ.copy()
    environment['ANDROID_HOME'] = str(sdk)
    environment['ANDROID_SDK_ROOT'] = str(sdk)
    subprocess.run([str(wrapper), 'assembleDebug'], cwd=args.source, env=environment, check=True)

    apk = args.source / 'build/outputs/apk/debug/cts-audio-client-debug.apk'
    if not apk.is_file():
        raise SystemExit(f'Gradle did not produce expected APK: {apk}')
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(apk, args.output)
    print(f'APK: {args.output}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
