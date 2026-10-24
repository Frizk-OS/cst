# tests/tests/openglperf/meson.build

cargo = find_program('cargo', required: false)

if get_option('enable_rust_jni') and cargo.found()
  ctsopenglperf_manifest = meson.current_source_dir() / 'jni' / 'Cargo.toml'

  ctsopenglperf_so = custom_target(
    'libctsopenglperf_jni',
    build_by_default: true,
    input: [
      'jni/Cargo.toml',
      'jni/src/lib.rs',
    ],
    output: ['libctsopenglperf_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      ctsopenglperf_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libctsopenglperf_jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', ctsopenglperf_manifest],
    workdir: meson.current_source_dir(),
  )

  cts_openglperf_apk = custom_target(
    'CtsOpenGlPerfTestCases-apk',
    output: 'CtsOpenGlPerfTestCases.apk',
    input: [ctsopenglperf_so, ctsdeviceutil_jar, ctscommonutil_jar],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--assets-dir', meson.current_source_dir() / 'assets',
      '--source-root', meson.current_source_dir() / 'src',
      '--dependency-jar', '@INPUT1@',
      '--dependency-jar', '@INPUT2@',
      '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
      '--jni-lib', '@INPUT0@',
    ],
    build_by_default: true,
    install: false,
  )
else
  cts_openglperf_apk = custom_target(
    'CtsOpenGlPerfTestCases-apk',
    output: 'CtsOpenGlPerfTestCases.apk',
    input: [ctsdeviceutil_jar, ctscommonutil_jar],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--assets-dir', meson.current_source_dir() / 'assets',
      '--source-root', meson.current_source_dir() / 'src',
      '--dependency-jar', '@INPUT0@',
      '--dependency-jar', '@INPUT1@',
      '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
    ],
    build_by_default: true,
    install: false,
  )
endif
