# tests/tests/net/meson.build

cargo = find_program('cargo', required: false)

if get_option('enable_rust_jni') and cargo.found()
  nativedns_manifest = meson.current_source_dir() / 'jni' / 'Cargo.toml'

  nativedns_so = custom_target(
    'libnativedns_jni',
    build_by_default: true,
    input: [
      'jni/Cargo.toml',
      'jni/src/lib.rs',
    ],
    output: ['libnativedns_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      nativedns_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'libnativedns_jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', nativedns_manifest],
    workdir: meson.current_source_dir(),
  )
endif
