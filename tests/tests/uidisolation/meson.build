# tests/tests/uidisolation/meson.build

cts_uidisolation_apk = custom_target(
  'CtsUidIsolationTestCases-apk',
  output: 'CtsUidIsolationTestCases.apk',
  input: [ctsdeviceutil_jar, ctstestserver_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'src',
    '--dependency-jar', '@INPUT0@',
    '--dependency-jar', '@INPUT1@',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)
