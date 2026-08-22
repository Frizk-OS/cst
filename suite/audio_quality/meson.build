project(
  'android-cts-audio-quality',
  ['c', 'cpp'],
  version: '1.0.0',
  license: 'Apache-2.0',
  meson_version: '>=1.3.0',
  default_options: [
    'c_std=c17',
    'cpp_std=c++26',
    'warning_level=2',
    'buildtype=debugoptimized',
    'b_ndebug=if-release',
  ],
)

cpp = meson.get_compiler('cpp')

common_includes = include_directories(
  'compat/include',
  'lib/include',
  'lib/src',
  'executable/src',
)

project_cpp_args = []
if cpp.get_id() == 'gcc' or cpp.get_id() == 'clang'
  project_cpp_args += ['-fno-exceptions']
endif
add_project_arguments(project_cpp_args, language: 'cpp')

lib_sources = run_command(
  'find',
  meson.current_source_dir() / 'lib/src',
  '-type',
  'f',
  '-name',
  '*.cpp',
  check: true,
).stdout().strip().split('\n')
lib_sources = lib_sources[0] != '' ? lib_sources : []

lib_deps = [
  dependency('threads', required: true),
  cpp.find_library('dl', required: false),
  cpp.find_library('rt', required: false),
  cpp.find_library('m', required: false),
  dependency('tinyalsa', required: false),
  dependency('tinyxml2', required: false),
]

cts_audio_quality_lib = static_library(
  'cts_audio_quality',
  lib_sources,
  include_directories: common_includes,
  dependencies: lib_deps,
  install: false,
)

executable(
  'cts_audio_quality',
  'executable/src/main.cpp',
  include_directories: common_includes,
  link_with: cts_audio_quality_lib,
  dependencies: lib_deps,
  install: true,
)

if get_option('tests')
  test_sources = run_command(
    'find',
    meson.current_source_dir() / 'test',
    '-type',
    'f',
    '-name',
    '*.cpp',
    check: true,
  ).stdout().strip().split('\n')
  test_sources = test_sources[0] != '' ? test_sources : []

  gtest_dep = dependency('gtest', required: false)
  if not gtest_dep.found()
    gtest_dep = dependency('gtest_main', required: false)
  endif

  if gtest_dep.found() and test_sources.length() > 0
    executable(
      'cts_audio_quality_test',
      test_sources,
      include_directories: common_includes,
      link_with: cts_audio_quality_lib,
      dependencies: [lib_deps, gtest_dep],
      install: false,
    )
  endif
endif

install_subdir(
  'test_description',
  install_dir: join_paths(get_option('datadir'), 'android-cts-audio-quality'),
)
