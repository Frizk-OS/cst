# tests/tests/accessibilityservice/meson.build

cts_accessibilityservice_apk = custom_target(
  'CtsAccessibilityServiceTestCases-apk',
  output: 'CtsAccessibilityServiceTestCases.apk',
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
