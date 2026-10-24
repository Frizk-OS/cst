# suite/cts/deviceTests/browserbench/meson.build

cts_device_browserbench_apk = custom_target(
  'CtsDeviceBrowserBench-apk',
  output: 'CtsDeviceBrowserBench.apk',
  input: [ctstestserver_jar, ctsdeviceutil_jar, ctscommonutil_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
    '--assets-dir', meson.current_source_dir() / 'assets',
    '--source-root', meson.current_source_dir() / 'src',
    '--dependency-jar', '@INPUT0@',
    '--dependency-jar', '@INPUT1@',
    '--dependency-jar', '@INPUT2@',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)
