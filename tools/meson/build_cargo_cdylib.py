#!/usr/bin/env python3
"""
Meson helper script for building Cargo cdylib shared libraries targeting Android ARM (armeabi-v7a).
Cross-compiles for Android 4.4.4 KitKat (GT-I9300 / Cortex-A9) using NDK Clang and LLD.
"""

from __future__ import annotations
import os
import sys
import subprocess
import shutil
from pathlib import Path


def main() -> int:
    if len(sys.argv) < 5:
        print("Usage: build_cargo_cdylib.py <cargo_exe> <manifest_path> <target_dir> <output_so> [--target <target>]", file=sys.stderr)
        return 1

    cargo = sys.argv[1]
    manifest = Path(sys.argv[2]).resolve()
    target_dir = Path(sys.argv[3]).resolve()
    output_so = Path(sys.argv[4]).resolve()

    target = "armv7-linux-androideabi"
    if len(sys.argv) > 6 and sys.argv[5] == "--target":
        target = sys.argv[6]
    elif "CARGO_BUILD_TARGET" in os.environ:
        target = os.environ["CARGO_BUILD_TARGET"]

    env = os.environ.copy()

    # Locate Android NDK Clang linker for armv7
    ndk_clang = Path("/home/vptr/Android/Sdk/ndk/30.0.16248370/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang")
    if ndk_clang.is_file():
        env["CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER"] = str(ndk_clang)

    # Compile bionic_compat stub if needed for KitKat (dl_iterate_phdr)
    compat_o = target_dir / "bionic_compat.o"
    if not compat_o.is_file() and ndk_clang.is_file():
        target_dir.mkdir(parents=True, exist_ok=True)
        compat_c = target_dir / "bionic_compat.c"
        compat_c.write_text("""/* Copyright (C) 2026 The AOSP and FrizkOS */
#include <stddef.h>

__attribute__((weak, visibility("default")))
int dl_iterate_phdr(int (*callback)(void*, size_t, void*), void* data) {
    (void)callback;
    (void)data;
    return 0;
}
""")
        subprocess.run([str(ndk_clang), "-c", str(compat_c), "-o", str(compat_o), "-fPIC", "-O2"], check=True)

    rustflags = env.get("RUSTFLAGS", "")
    if compat_o.is_file() and target == "armv7-linux-androideabi":
        rustflags += f" -C link-arg={compat_o} -C link-arg=-Wl,-u,dl_iterate_phdr"
    env["RUSTFLAGS"] = rustflags.strip()

    cmd = [
        cargo, "build", "--release",
        "--manifest-path", str(manifest),
        "--target-dir", str(target_dir),
    ]
    if target:
        cmd.extend(["--target", target])
        built_so = target_dir / target / "release" / output_so.name
    else:
        built_so = target_dir / "release" / output_so.name

    res = subprocess.run(cmd, env=env)
    if res.returncode != 0:
        return res.returncode

    if not built_so.exists():
        print(f"Error: expected built library at {built_so} not found", file=sys.stderr)
        return 2

    output_so.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(built_so, output_so)
    return 0


if __name__ == "__main__":
    sys.exit(main())
