# tests/tests/security/meson.build

cargo = find_program('cargo', required: false)

if get_option('enable_rust_jni') and cargo.found()
  ctssecurity_manifest = meson.current_source_dir() / 'jni' / 'Cargo.toml'

  ctssecurity_so = custom_target(
    'libctssecurity_jni',
    build_by_default: true,
    input: [
      'jni/Cargo.toml',
      'jni/src/lib.rs',
    ],
    output: ['libctssecurity_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      ctssecurity_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libctssecurity_jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', ctssecurity_manifest],
    workdir: meson.current_source_dir(),
  )
endif
