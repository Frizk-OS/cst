# suite/cts/deviceTests/dram/meson.build

subdir('jni')

if get_option('enable_rust_jni') and cargo.found()
  cts_device_dram_apk = custom_target(
    'CtsDeviceDram-apk',
    output: 'CtsDeviceDram.apk',
    input: [ctsdram_jni_so, ctsdeviceutil_jar, ctscommonutil_jar],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--source-root', meson.current_source_dir() / 'src',
      '--dependency-jar', '@INPUT1@',
      '--dependency-jar', '@INPUT2@',
      '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
      '--jni-lib', '@INPUT0@',
    ],
    build_by_default: true,
    install: false,
  )
endif
