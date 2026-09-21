project(
  'android-cts-modern',
  ['c', 'cpp'],
  version: '1.0.0',
  meson_version: '>=1.3.0',
  default_options: [
    'c_std=c23',
    'cpp_std=c++26',
    'warning_level=2',
    'buildtype=debugoptimized',
  ],
)

python = import('python').find_installation(required: true)

if get_option('audio_quality')
  subdir('suite/audio_quality')
endif

if get_option('tools')
  subdir('tools')
endif

run_target(
  'audio-quality-configure',
  command: [
    python,
    files('tools/meson/build_standalone.py'),
    '--source', meson.project_source_root() / 'suite/audio_quality',
    '--build', meson.project_build_root() / 'audio-quality',
    '--tests', get_option('audio_quality_tests').to_string(),
  ],
)

run_target(
  'cts-usb-accessory-configure',
  command: [
    python,
    files('tools/meson/build_standalone.py'),
    '--source', meson.project_source_root() / 'apps/cts-usb-accessory',
    '--build', meson.project_build_root() / 'cts-usb-accessory',
  ],
)

run_target(
  'android-mk-audit',
  command: [
    python,
    files('tools/meson/audit_android_mk.py'),
    meson.project_source_root(),
  ],
)

if get_option('browserbench_ts')
  custom_target(
    'browserbench-octane-js',
    input: [
      'suite/cts/deviceTests/browserbench/assets/octane/base.ts',
      'suite/cts/deviceTests/browserbench/assets/octane/cts_report.ts',
      'suite/cts/deviceTests/browserbench/assets/octane/navier-stokes.ts',
      'suite/cts/deviceTests/browserbench/assets/octane/octane.ts',
      'suite/cts/deviceTests/browserbench/assets/octane/run.ts',
      'suite/cts/deviceTests/browserbench/assets/octane/splay.ts',
    ],
    output: [
      'base.js',
      'cts_report.js',
      'navier-stokes.js',
      'octane.js',
      'run.js',
      'splay.js',
    ],
    command: [
      python,
      files('tools/meson/build_typescript.py'),
      '--root-dir', meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane',
      '--out-dir', '@OUTDIR@',
      '@INPUT@',
    ],
    build_by_default: true,
    install: false,
  )

  run_target(
    'browserbench-octane-js-update',
    command: [
      python,
      files('tools/meson/build_typescript.py'),
      '--root-dir', meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane',
      '--out-dir', meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane',
      meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane/base.ts',
      meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane/cts_report.ts',
      meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane/navier-stokes.ts',
      meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane/octane.ts',
      meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane/run.ts',
      meson.project_source_root() / 'suite/cts/deviceTests/browserbench/assets/octane/splay.ts',
    ],
  )
endif

if get_option('android_build')
  custom_target(
    'cts-audio-client-apk',
    output: 'cts-audio-client-debug.apk',
    command: [
      python,
      files('tools/meson/build_android_apk.py'),
      '--source', meson.project_source_root() / 'suite/audio_quality/client',
      '--output', '@OUTPUT@',
    ],
    build_by_default: true,
    install: false,
  )
  message('Android APK targets enabled through the Gradle wrapper and Android SDK.')
else
  message('Android APK/JNI targets are disabled; configure with -Dandroid_build=true to build the audio client APK.')
endif
