#!/usr/bin/env python3
"""Generate a Meson cross-file for an Android Studio NDK installation."""

from __future__ import annotations

import argparse
import os
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--ndk', type=Path, default=None)
    parser.add_argument('--api', type=int, default=21)
    parser.add_argument('--abi', choices=('armv7', 'arm64', 'x86_64'), default='armv7')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()

    ndk = args.ndk or Path(os.environ.get('ANDROID_NDK_HOME', ''))
    if not ndk.is_dir():
        raise SystemExit('Set ANDROID_NDK_HOME or pass --ndk=/path/to/android-ndk')

    prebuilt = ndk / 'toolchains/llvm/prebuilt/linux-x86_64'
    bin_dir = prebuilt / 'bin'
    targets = {
        'armv7': ('armv7a-linux-androideabi', 'arm'),
        'arm64': ('aarch64-linux-android', 'aarch64'),
        'x86_64': ('x86_64-linux-android', 'x86_64'),
    }
    target, cpu = targets[args.abi]
    prefix = f'{target}{args.api}'

    for tool in (f'{prefix}-clang', f'{prefix}-clang++', 'llvm-ar', 'llvm-strip'):
        if not (bin_dir / tool).is_file():
            raise SystemExit(f'Missing NDK tool: {bin_dir / tool}')

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        f'''[binaries]\n'''
        f'''c = '{bin_dir / (prefix + '-clang')}'\n'''
        f'''cpp = '{bin_dir / (prefix + '-clang++')}'\n'''
        f'''ar = '{bin_dir / 'llvm-ar'}'\n'''
        f'''strip = '{bin_dir / 'llvm-strip'}'\n'''
        f'''[host_machine]\n'''
        f'''system = 'android'\n'''
        f'''cpu_family = '{cpu}'\n'''
        f'''cpu = '{args.abi}'\n'''
        f'''endian = 'little'\n'''
        f'''[properties]\n'''
        f'''sys_root = '{prebuilt / 'sysroot'}'\n'''
        f'''needs_exe_wrapper = true\n'''
        f'''[built-in options]\n'''
        f'''c_std = 'c23'\n'''
        f'''cpp_std = 'c++26'\n'''
    )
    print(f'Generated {args.output} for {args.abi}, API {args.api}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
