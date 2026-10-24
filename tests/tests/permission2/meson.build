# tests/tests/permission2/meson.build

cts_permission2_apk = custom_target(
  'CtsPermission2TestCases-apk',
  output: 'CtsPermission2TestCases.apk',
  input: [ctsdeviceutil_jar, ctscommonutil_jar],
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
