# hostsidetests/appsecurity/meson.build

junit_jar = meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar'

# 1. AppAccessData
cts_appaccessdata_apk = custom_target(
  'CtsAppAccessData-apk',
  output: 'CtsAppAccessData.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/AppAccessData/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/AppAccessData/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 2. AppWithData
cts_appwithdata_apk = custom_target(
  'CtsAppWithData-apk',
  output: 'CtsAppWithData.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/AppWithData/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/AppWithData/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 3. ExternalStorageApp
cts_externalstorageapp_apk = custom_target(
  'CtsExternalStorageApp-apk',
  output: 'CtsExternalStorageApp.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/ExternalStorageApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/ExternalStorageApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 4. MultiUserStorageApp
cts_multiuserstorageapp_apk = custom_target(
  'CtsMultiUserStorageApp-apk',
  output: 'CtsMultiUserStorageApp.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/MultiUserStorageApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/MultiUserStorageApp/src',
    '--source-root', meson.current_source_dir() / 'test-apps/ExternalStorageApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 5. ReadExternalStorageApp
cts_readexternalstorageapp_apk = custom_target(
  'CtsReadExternalStorageApp-apk',
  output: 'CtsReadExternalStorageApp.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/ReadExternalStorageApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/ReadExternalStorageApp/src',
    '--source-root', meson.current_source_dir() / 'test-apps/ExternalStorageApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 6. WriteExternalStorageApp
cts_writeexternalstorageapp_apk = custom_target(
  'CtsWriteExternalStorageApp-apk',
  output: 'CtsWriteExternalStorageApp.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/WriteExternalStorageApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/WriteExternalStorageApp/src',
    '--source-root', meson.current_source_dir() / 'test-apps/ExternalStorageApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 7. InstrumentationAppDiffCert
cts_instrumentationappdiffcert_apk = custom_target(
  'CtsInstrumentationAppDiffCert-apk',
  output: 'CtsInstrumentationAppDiffCert.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/InstrumentationAppDiffCert/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/InstrumentationAppDiffCert/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 8. TargetInstrumentationApp
cts_targetinstrumentationapp_apk = custom_target(
  'CtsTargetInstrumentationApp-apk',
  output: 'CtsTargetInstrumentationApp.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/TargetInstrumentationApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/TargetInstrumentationApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 9. PermissionDeclareApp
cts_permissiondeclareapp_apk = custom_target(
  'CtsPermissionDeclareApp-apk',
  output: 'CtsPermissionDeclareApp.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/PermissionDeclareApp/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/PermissionDeclareApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 10. PermissionDeclareAppCompat
cts_permissiondeclareappcompat_apk = custom_target(
  'CtsPermissionDeclareAppCompat-apk',
  output: 'CtsPermissionDeclareAppCompat.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/PermissionDeclareAppCompat/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/PermissionDeclareAppCompat/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 11. UsePermissionDiffCert
cts_usepermissiondiffcert_apk = custom_target(
  'CtsUsePermissionDiffCert-apk',
  output: 'CtsUsePermissionDiffCert.apk',
  input: [junit_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/UsePermissionDiffCert/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'test-apps/UsePermissionDiffCert/src',
    '--source-root', meson.current_source_dir() / 'test-apps/PermissionDeclareApp/src',
    '--dependency-jar', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)

# 12. SharedUidInstall
cts_shareduidinstall_apk = custom_target(
  'CtsSharedUidInstall-apk',
  output: 'CtsSharedUidInstall.apk',
  input: [],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/SharedUidInstall/AndroidManifest.xml',
  ],
  build_by_default: true,
  install: false,
)

# 13. SharedUidInstallDiffCert
cts_shareduidinstalldiffcert_apk = custom_target(
  'CtsSharedUidInstallDiffCert-apk',
  output: 'CtsSharedUidInstallDiffCert.apk',
  input: [],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/SharedUidInstallDiffCert/AndroidManifest.xml',
  ],
  build_by_default: true,
  install: false,
)

# 14. SimpleAppInstall
cts_simpleappinstall_apk = custom_target(
  'CtsSimpleAppInstall-apk',
  output: 'CtsSimpleAppInstall.apk',
  input: [],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/SimpleAppInstall/AndroidManifest.xml',
  ],
  build_by_default: true,
  install: false,
)

# 15. SimpleAppInstallDiffCert
cts_simpleappinstalldiffcert_apk = custom_target(
  'CtsSimpleAppInstallDiffCert-apk',
  output: 'CtsSimpleAppInstallDiffCert.apk',
  input: [],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'test-apps/SimpleAppInstallDiffCert/AndroidManifest.xml',
  ],
  build_by_default: true,
  install: false,
)
