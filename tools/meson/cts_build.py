#!/usr/bin/env python3
# Copyright (C) 2026 The Android Open Source Project
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

"""Helper script to package and collect built CTS artifacts into a distribution directory."""

import argparse
import os
import shutil
import sys
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(description="Package CTS artifacts from Meson build directory.")
    parser.add_argument("--build-dir", type=Path, default=Path("build"), help="Path to meson build directory")
    parser.add_argument("--dest-dir", type=Path, required=True, help="Destination directory for CTS package")
    args = parser.parse_args()

    build_dir = args.build_dir.resolve()
    dest_dir = args.dest_dir.resolve()

    if not build_dir.exists():
        sys.exit(f"Error: Build directory '{build_dir}' does not exist.")

    dest_dir.mkdir(parents=True, exist_ok=True)
    print(f"=== CTS Testcases Directory: {dest_dir} ===")

    # Defined in CtsBuild.mk:
    # CTS_HOST_LIBRARY_JARS, CTS_TEST_CASES, CTS_TARGET_TEST_COVERAGE_XML
    artifacts = [
        # Host JARs
        build_dir / "tools/cts-xml-generator/cts-xml-generator.jar",
        build_dir / "tools/cts-native-scanner/cts-native-scanner.jar",
        build_dir / "tools/cts-java-scanner/cts-java-scanner.jar",
        build_dir / "tools/cts-api-coverage/cts-api-coverage.jar",
        build_dir / "tools/tradefed-host/cts-tradefed.jar",
        build_dir / "tools/device-setup/ctsdeviceinfolib.jar",
        build_dir / "tools/dex-tools/dex-tools.jar",
        build_dir / "libs/jsonlib.jar",
        build_dir / "libs/ctscommonutil.jar",
        build_dir / "libs/vogarexpectlib.jar",
        build_dir / "libs/ctsdeviceutil.jar",
        build_dir / "libs/ctstestserver.jar",
        build_dir / "libs/wrappedgtest.jar",
        # Host side test JARs
        build_dir / "hostsidetests/CtsUsbTests.jar",
        build_dir / "hostsidetests/CtsMonkeyTestCases.jar",
        build_dir / "hostsidetests/CtsAadbTestCases.jar",
        build_dir / "hostsidetests/CtsSampleHostTestCases.jar",
        build_dir / "hostsidetests/CtsAppSecurityTests.jar",
        build_dir / "hostsidetests/CtsHoloTestCases.jar",
        # Binaries
        build_dir / "tools/dasm-rs/dasm-rs",
        build_dir / "subprojects/dasm-rs/dasm-rs",
        build_dir / "apps/cts-usb-accessory/cts-usb-accessory",
        build_dir / "suite/audio_quality/cts_audio_quality",
        # JNI Libraries
        build_dir / "tests/jni/libcts_jni.so",
        build_dir / "tests/tests/jni/libjnitest.so",
        build_dir / "apps/CtsVerifier/jni/libctsverifier_jni.so",
        build_dir / "suite/cts/deviceTests/dram/jni/libctsdram_jni.so",
        build_dir / "suite/cts/deviceTests/simplecpu/jni/libctscpu_jni.so",
        build_dir / "tests/tests/drm/libctsdrm_jni.so",
        build_dir / "tests/tests/net/libnativedns_jni.so",
        build_dir / "tests/tests/permission/libctspermission_jni.so",
        build_dir / "tests/tests/security/libctssecurity_jni.so",
        build_dir / "tests/tests/openglperf/libctsopenglperf_jni.so",
        build_dir / "tests/tests/mediastress/libctsmediastress_jni.so",
        build_dir / "tests/tests/opengl/libopengltest_jni.so",
    ]

    # Automatically collect all generated APKs across the build tree (excluding dest_dir)
    for apk_path in build_dir.rglob("*.apk"):
        if dest_dir not in apk_path.parents and apk_path != dest_dir:
            artifacts.append(apk_path)

    copied = 0
    for art in artifacts:
        if art.is_file():
            target = dest_dir / art.name
            shutil.copy2(art, target)
            print(f"  [COLLECT] {art.name} -> {target}")
            copied += 1

    print(f"Collected {copied} artifacts into {dest_dir}")

if __name__ == "__main__":
    main()
