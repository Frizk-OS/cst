# tests/tests/rscpp/meson.build

rscpp_sources = files(
  'librscpptest/neon_kernels.cpp',
  'librscpptest/rs_jni.cpp',
)

librscpptest_jni = custom_target(
  'librscpptest_jni',
  build_by_default: true,
  input: rscpp_sources,
  output: ['librscpptest_jni.so'],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_cpp_shlib.py',
    '--output', '@OUTPUT@',
    '--sources', '@INPUT@',
    '--include-dirs', meson.current_source_dir() / 'librscpptest',
  ],
)

rscpp_test_exe = executable(
  'rscpp_neon_tests',
  files(
    'librscpptest/neon_kernels.cpp',
    'librscpptest/test_main.cpp',
  ),
  include_directories: include_directories('librscpptest'),
  cpp_args: ['-std=c++23'],
  build_by_default: true,
)

test('rscpp-neon-tests', rscpp_test_exe)

cts_rscpp_apk = custom_target(
  'CtsRsCppTestCases-apk',
  output: 'CtsRsCppTestCases.apk',
  input: [librscpptest_jni, ctsdeviceutil_jar, ctscommonutil_jar],
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
