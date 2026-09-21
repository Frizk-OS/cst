cpp = meson.get_compiler('cpp')

common_includes = include_directories(
  'compat/include',
  'lib/include',
  'lib/src',
  'executable/src',
)

project_cpp_args = []
# Modern host port uses standard C++ exceptions and std::thread. Keep the compiler
# defaults unless a project-specific flag is required.
add_project_arguments(project_cpp_args, language: 'cpp')

if host_machine.cpu_family() == 'arm'
  add_project_arguments('-marm', language: ['c', 'cpp'])
endif

lib_sources = files(
  'lib/src/Adb.cpp',
  'lib/src/BuiltinProcessing.cpp',
  'lib/src/ClientImpl.cpp',
  'lib/src/ClientSocket.cpp',
  'lib/src/FileUtil.cpp',
  'lib/src/GenericFactory.cpp',
  'lib/src/Log.cpp',
  'lib/src/Report.cpp',
  'lib/src/Semaphore.cpp',
  'lib/src/Settings.cpp',
  'lib/src/SignalProcessingImpl.cpp',
  'lib/src/SimpleScriptExec.cpp',
  'lib/src/StringUtil.cpp',
  'lib/src/task/ModelBuilder.cpp',
  'lib/src/task/TaskAsync.cpp',
  'lib/src/task/TaskBatch.cpp',
  'lib/src/task/TaskCase.cpp',
  'lib/src/task/TaskDownload.cpp',
  'lib/src/task/TaskGeneric.cpp',
  'lib/src/task/TaskInput.cpp',
  'lib/src/task/TaskMessage.cpp',
  'lib/src/task/TaskOutput.cpp',
  'lib/src/task/TaskProcess.cpp',
  'lib/src/task/TaskSave.cpp',
  'lib/src/task/TaskSequential.cpp',
  'lib/src/task/TaskSound.cpp',
  'lib/src/audio/AudioHardware.cpp',
  'lib/src/audio/AudioLocal.cpp',
  'lib/src/audio/AudioPlaybackLocal.cpp',
  'lib/src/audio/AudioProtocol.cpp',
  'lib/src/audio/AudioRecordingLocal.cpp',
  'lib/src/audio/AudioRemote.cpp',
  'lib/src/audio/AudioSignalFactory.cpp',
  'lib/src/audio/Buffer.cpp',
  'lib/src/audio/RemoteAudio.cpp',
)

lib_deps = [
  dependency('threads', required: true),
  cpp.find_library('dl', required: false),
  cpp.find_library('rt', required: false),
  cpp.find_library('m', required: false),
  dependency('tinyalsa', required: false),
  dependency('tinyxml2', fallback: ['tinyxml2', 'tinyxml2_dep']),
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

if get_option('audio_quality_tests')
  test_sources = files(
    'test/LogTest.cpp',
    'test/FileUtilTest.cpp',
    'test/ClientInterfaceTest.cpp',
    'test/BufferTest.cpp',
    'test/AudioSignalFactoryTest.cpp',
    'test/AudioRecordingLocalTest.cpp',
    'test/AudioRecordPlayLocalTest.cpp',
    'test/AudioPlaybackLocalTest.cpp',
    'test/AudioLocalTest.cpp',
    'test/AudioHardwareTest.cpp',
    'test/SimpleScriptExecTest.cpp',
    'test/SignalProcessingInterfaceTest.cpp',
    'test/RemoteAudioFakeTcpTest.cpp',
    'test/ModelBuilderTest.cpp',
    'test/MixerTest.cpp',
    'test/TaskProcessTest.cpp',
    'test/TaskCaseTest.cpp',
    'test/StringUtilTest.cpp',
    'test/TaskSequentialTest.cpp',
    'test/TaskTest.cpp',
  )

  gtest_dep = dependency('gtest_main', required: false)
  if not gtest_dep.found()
    gtest_dep = dependency('gtest', required: false)
  endif

  if gtest_dep.found()
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
  install_dir: get_option('datadir') / 'android-cts-audio-quality',
)
