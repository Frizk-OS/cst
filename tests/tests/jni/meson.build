# Meson build for JNI test cdylib rewritten in Rust

cargo = find_program('cargo', required: false)

if cargo.found()
  jnitest_manifest = meson.current_source_dir() / 'libjnitest_rs' / 'Cargo.toml'
  
  jnitest_so = custom_target(
    'libjnitest',
    build_by_default: true,
    input: [
      'libjnitest_rs/Cargo.toml',
      'libjnitest_rs/src/lib.rs',
      'libjnitest_rs/src/jni_sys.rs',
    ],
    output: ['libjnitest.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      jnitest_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libjnitest-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', jnitest_manifest],
    workdir: meson.current_source_dir(),
  )

  cts_jni_apk = custom_target(
    'CtsJniTestCases-apk',
    output: 'CtsJniTestCases.apk',
    input: [jnitest_so, ctsdeviceutil_jar, ctscommonutil_jar],
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
