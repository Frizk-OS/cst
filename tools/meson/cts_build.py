#!/usr/bin/env python3
"""
Port of build/core/CtsBuild.mk into a standalone Python runner for Meson.
Collects and packages CTS test packages, host tools, and libraries into CTS distribution format.
"""

from __future__ import annotations

import argparse
import os
import shutil
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Package CTS distribution directory")
    parser.add_argument("--build-dir", type=Path, default=Path("build"), help="Meson build directory")
    parser.add_argument("--dest-dir", "--out-dir", dest="dest_dir", type=Path, default=None, help="Destination directory for CTS testcases")
    args = parser.parse_args()

    build_dir = args.build_dir.resolve()
    if not build_dir.is_dir():
        print(f"Error: Build directory not found: {build_dir}", file=sys.stderr)
        return 1

    dest_dir = args.dest_dir.resolve() if args.dest_dir else build_dir / "cts-testcases"
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
    ]

    # Automatically collect all generated APKs across the build tree (excluding dest_dir)
    for apk_path in build_dir.rglob("*.apk"):
        if dest_dir not in apk_path.parents and apk_path != dest_dir:
            artifacts.append(apk_path)

    copied = 0
    seen = set()
    for art in artifacts:
        if art.is_file() and art.name not in seen:
            target_path = dest_dir / art.name
            shutil.copy2(art, target_path)
            print(f"  [COLLECT] {art.name} -> {target_path}")
            copied += 1
            seen.add(art.name)

    print(f"Collected {copied} artifacts into {dest_dir}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
