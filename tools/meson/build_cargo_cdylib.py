#!/usr/bin/env python3
import os
import sys
import subprocess
import shutil
from pathlib import Path

def main():
    if len(sys.argv) < 5:
        print("Usage: build_cargo_cdylib.py <cargo_exe> <manifest_path> <target_dir> <output_so> [--target <target>]", file=sys.stderr)
        return 1

    cargo = sys.argv[1]
    manifest = Path(sys.argv[2]).resolve()
    target_dir = Path(sys.argv[3]).resolve()
    output_so = Path(sys.argv[4]).resolve()

    target = None
    if len(sys.argv) > 6 and sys.argv[5] == "--target":
        target = sys.argv[6]
    elif "CARGO_BUILD_TARGET" in os.environ:
        target = os.environ["CARGO_BUILD_TARGET"]
    elif os.environ.get("CTS_TARGET_ARCH", "").lower() in ("arm", "arm32", "armv7", "armeabi-v7a"):
        target = "armv7-linux-androideabi"

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

    res = subprocess.run(cmd)
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
