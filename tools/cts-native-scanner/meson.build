custom_target(
  'cts-native-scanner-jar',
  output: 'cts-native-scanner.jar',
  command: [
    find_program('python3'),
    meson.project_source_root() / 'tools/meson/build_java_jar.py',
    '--output', '@OUTPUT@',
    '--main-class', 'com.android.cts.nativescanner.CtsNativeScanner',
    '--source-root', meson.project_source_root() / 'tools/cts-native-scanner/src',
  ],
  build_by_default: true,
  install: false,
)
