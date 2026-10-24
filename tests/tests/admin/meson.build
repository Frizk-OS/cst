# tests/tests/admin/meson.build

cts_admin_apk = custom_target(
  'CtsDeviceAdminTestCases-apk',
  output: 'CtsDeviceAdminTestCases.apk',
  input: [ctsdeviceutil_jar, ctstestserver_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'src',
    '--source-root', meson.project_source_root() / 'tests/deviceadmin/src',
    '--dependency-jar', '@INPUT0@',
    '--dependency-jar', '@INPUT1@',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
    '--dependency-jar', mockito_jar,
    '--dependency-jar', guava_jar,
  ],
  build_by_default: true,
  install: false,
)
