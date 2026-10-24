# tests/tests/opengl/meson.build

cargo = find_program('cargo', required: false)

if get_option('enable_rust_jni') and cargo.found()
  opengltest_manifest = meson.current_source_dir() / 'jni' / 'Cargo.toml'

  opengltest_so = custom_target(
    'libopengltest_jni',
    build_by_default: true,
    input: [
      'jni/Cargo.toml',
      'jni/src/lib.rs',
    ],
    output: ['libopengltest_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      opengltest_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libopengltest_jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', opengltest_manifest],
    workdir: meson.current_source_dir(),
  )

  cts_opengl_apk = custom_target(
    'CtsOpenGlTestCases-apk',
    output: 'CtsOpenGlTestCases.apk',
    input: [opengltest_so, ctsdeviceutil_jar, ctscommonutil_jar],
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
  cts_opengl_apk = custom_target(
    'CtsOpenGlTestCases-apk',
    output: 'CtsOpenGlTestCases.apk',
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
