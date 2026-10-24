# tests/tests/preference2/meson.build

cts_preference2_apk = custom_target(
  'CtsPreference2TestCases-apk',
  output: 'CtsPreference2TestCases.apk',
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
