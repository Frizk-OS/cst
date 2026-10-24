# tests/tests/permission/meson.build

cargo = find_program('cargo', required: false)

if get_option('enable_rust_jni') and cargo.found()
  ctspermission_manifest = meson.current_source_dir() / 'jni' / 'Cargo.toml'

  ctspermission_so = custom_target(
    'libctspermission_jni',
    build_by_default: true,
    input: [
      'jni/Cargo.toml',
      'jni/src/lib.rs',
    ],
    output: ['libctspermission_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      ctspermission_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libctspermission_jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', ctspermission_manifest],
    workdir: meson.current_source_dir(),
  )

  cts_permission_apk = custom_target(
    'CtsPermissionTestCases-apk',
    output: 'CtsPermissionTestCases.apk',
    input: [ctspermission_so, ctsdeviceutil_jar, ctscommonutil_jar],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--source-root', meson.current_source_dir() / 'src',
      '--dependency-jar', '@INPUT1@',
      '--dependency-jar', '@INPUT2@',
      '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
      '--jni-lib', '@INPUT0@',
    ],
    build_by_default: true,
    install: false,
  )
endif
