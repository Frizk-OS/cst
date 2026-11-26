#!/usr/bin/env python3
# Copyright (C) 2026 The AOSP and FrizkOS
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import os
import sys
import subprocess
import shutil
import argparse
from pathlib import Path

def find_ndk_clangpp():
    if "NDK_CXX" in os.environ:
        return os.environ["NDK_CXX"]
    ndk_dirs = []
    if "ANDROID_NDK_ROOT" in os.environ:
        ndk_dirs.append(Path(os.environ["ANDROID_NDK_ROOT"]))
    if "ANDROID_NDK_HOME" in os.environ:
        ndk_dirs.append(Path(os.environ["ANDROID_NDK_HOME"]))
    sdk_ndk = Path.home() / "Android/Sdk/ndk"
    if sdk_ndk.exists():
        ndk_dirs.extend(sorted(sdk_ndk.glob("*"), reverse=True))

    for ndk in ndk_dirs:
        toolchain = ndk / "toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang++"
        if toolchain.exists():
            return str(toolchain)
    return None

def main():
    parser = argparse.ArgumentParser(description="Build C++ shared library for ARM32 Android / CTS")
    parser.add_argument("--output", required=True, type=Path, help="Output .so file")
    parser.add_argument("--sources", nargs="+", required=True, type=Path, help="Source files")
    parser.add_argument("--include-dirs", nargs="*", default=[], type=Path, help="Include directories")
    parser.add_argument("--compiler", default=None, help="Explicit C++ compiler to use")
    parser.add_argument("--cxxflags", nargs="*", default=[], help="Extra compiler flags")
    parser.add_argument("--ldflags", nargs="*", default=[], help="Extra linker flags")
    args = parser.parse_args()

    cxx = args.compiler
    is_ndk = False
    if not cxx:
        cxx = find_ndk_clangpp()
        if cxx:
            is_ndk = True
        else:
            cxx = "clang++"

    cmd = [cxx, "-std=c++23", "-O3", "-fPIC", "-shared"]
    if is_ndk:
        cmd.extend(["-mfpu=neon", "-mfloat-abi=softfp", "-D__ARM_NEON__=1", "-D__ARM_NEON=1"])

    for inc in args.include_dirs:
        cmd.extend(["-I", str(inc.resolve())])

    cmd.extend(args.cxxflags)
    for src in args.sources:
        cmd.append(str(src.resolve()))

    cmd.extend(args.ldflags)
    if is_ndk:
        cmd.extend(["-llog", "-lm"])

    output_so = args.output.resolve()
    output_so.parent.mkdir(parents=True, exist_ok=True)
    cmd.extend(["-o", str(output_so)])

    res = subprocess.run(cmd)
    if res.returncode != 0:
        return res.returncode

    if not output_so.exists():
        print(f"Error: output {output_so} was not created", file=sys.stderr)
        return 1

    return 0

if __name__ == "__main__":
    sys.exit(main())
