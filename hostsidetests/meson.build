# hostsidetests/meson.build

cts_usb_tests_jar = custom_target(
  'cts-usb-tests-jar',
  output: 'CtsUsbTests.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'usb/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_monkey_test_cases_jar = custom_target(
  'cts-monkey-test-cases-jar',
  output: 'CtsMonkeyTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'monkey/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_aadb_test_cases_jar = custom_target(
  'cts-aadb-test-cases-jar',
  output: 'CtsAadbTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'aadb/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_sample_host_test_cases_jar = custom_target(
  'cts-sample-host-test-cases-jar',
  output: 'CtsSampleHostTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'sample/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_app_security_tests_jar = custom_target(
  'cts-app-security-tests-jar',
  output: 'CtsAppSecurityTests.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'appsecurity/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_holo_test_cases_jar = custom_target(
  'cts-holo-test-cases-jar',
  output: 'CtsHoloTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'holo/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

# Test APKs for hostsidetests
cts_usb_serial_test_app_apk = custom_target(
  'CtsUsbSerialTestApp-apk',
  output: 'CtsUsbSerialTestApp.apk',
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'usb/SerialTestApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'usb/SerialTestApp/src',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_sample_device_app_apk = custom_target(
  'CtsSampleDeviceApp-apk',
  output: 'CtsSampleDeviceApp.apk',
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'sample/app/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'sample/app/src',
  ],
  build_by_default: true,
  install: false,
)

cts_monkey_app_apk = custom_target(
  'CtsMonkeyApp-apk',
  output: 'CtsMonkeyApp.apk',
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'monkey/test-apps/CtsMonkeyApp/AndroidManifest.xml',
    '--res-dir', meson.current_source_dir() / 'monkey/test-apps/CtsMonkeyApp/res',
    '--source-root', meson.current_source_dir() / 'monkey/test-apps/CtsMonkeyApp/src',
  ],
  build_by_default: true,
  install: false,
)

cts_monkey_app2_apk = custom_target(
  'CtsMonkeyApp2-apk',
  output: 'CtsMonkeyApp2.apk',
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'monkey/test-apps/CtsMonkeyApp2/AndroidManifest.xml',
    '--res-dir', meson.current_source_dir() / 'monkey/test-apps/CtsMonkeyApp2/res',
    '--source-root', meson.current_source_dir() / 'monkey/test-apps/CtsMonkeyApp2/src',
  ],
  build_by_default: true,
  install: false,
)

cts_holo_device_app_apk = custom_target(
  'CtsHoloDeviceApp-apk',
  output: 'CtsHoloDeviceApp.apk',
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'holo/app/AndroidManifest.xml',
    '--res-dir', meson.current_source_dir() / 'holo/app/res',
    '--source-root', meson.current_source_dir() / 'holo/app/src',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

# AppSecurity Test APKs
subdir('appsecurity')
