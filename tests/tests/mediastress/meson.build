# tests/tests/mediastress/meson.build

cargo = find_program('cargo', required: false)

if get_option('enable_rust_jni') and cargo.found()
  ctsmediastress_manifest = meson.current_source_dir() / 'jni' / 'Cargo.toml'

  ctsmediastress_so = custom_target(
    'libctsmediastress_jni',
    build_by_default: true,
    input: [
      'jni/Cargo.toml',
      'jni/src/lib.rs',
    ],
    output: ['libctsmediastress_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      ctsmediastress_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libctsmediastress_jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', ctsmediastress_manifest],
    workdir: meson.current_source_dir(),
  )

  cts_mediastress_apk = custom_target(
    'CtsMediaStressTestCases-apk',
    output: 'CtsMediaStressTestCases.apk',
    input: [ctsmediastress_so, ctsdeviceutil_jar, ctscommonutil_jar],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--res-dir', meson.current_source_dir() / 'res',
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
  cts_mediastress_apk = custom_target(
    'CtsMediaStressTestCases-apk',
    output: 'CtsMediaStressTestCases.apk',
    input: [ctsdeviceutil_jar, ctscommonutil_jar],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--res-dir', meson.current_source_dir() / 'res',
      '--source-root', meson.current_source_dir() / 'src',
      '--dependency-jar', '@INPUT0@',
      '--dependency-jar', '@INPUT1@',
      '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
    ],
    build_by_default: true,
    install: false,
  )
endif
