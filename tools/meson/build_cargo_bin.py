#!/usr/bin/env python3
import os
import sys
import subprocess
import shutil
from pathlib import Path

def main():
    if len(sys.argv) < 5:
        print("Usage: build_cargo_bin.py <cargo_exe> <manifest_path> <target_dir> <output_bin> [--target <target>]", file=sys.stderr)
        return 1

    cargo = sys.argv[1]
    manifest = Path(sys.argv[2]).resolve()
    target_dir = Path(sys.argv[3]).resolve()
    output_bin = Path(sys.argv[4]).resolve()

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
        built_bin = target_dir / target / "release" / output_bin.name
    else:
        built_bin = target_dir / "release" / output_bin.name

    res = subprocess.run(cmd)
    if res.returncode != 0:
        return res.returncode

    if not built_bin.exists():
        print(f"Error: expected built binary at {built_bin} not found", file=sys.stderr)
        return 2

    output_bin.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(built_bin, output_bin)
    output_bin.chmod(0o755)
    return 0

if __name__ == "__main__":
    sys.exit(main())
