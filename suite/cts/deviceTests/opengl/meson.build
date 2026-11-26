# suite/cts/deviceTests/opengl/meson.build

opengl_sources = files(
  'jni/graphics/GLUtils.cpp',
  'jni/graphics/Matrix.cpp',
  'jni/graphics/Mesh.cpp',
  'jni/graphics/MeshNode.cpp',
  'jni/graphics/PerspectiveMeshNode.cpp',
  'jni/graphics/PerspectiveProgram.cpp',
  'jni/graphics/Program.cpp',
  'jni/graphics/ProgramNode.cpp',
  'jni/graphics/Renderer.cpp',
  'jni/graphics/SceneGraphNode.cpp',
  'jni/graphics/TexturedMeshNode.cpp',
  'jni/graphics/TransformationNode.cpp',
  'jni/graphics/Vector2D.cpp',
  'jni/primitive/GLPrimitive.cpp',
  'jni/primitive/contextswitch/ContextSwitchRenderer.cpp',
  'jni/primitive/fullpipeline/FullPipelineRenderer.cpp',
  'jni/primitive/pixeloutput/PixelOutputRenderer.cpp',
  'jni/primitive/shaderperf/ShaderPerfRenderer.cpp',
  'jni/reference/GLReference.cpp',
  'jni/reference/ReferenceRenderer.cpp',
  'jni/reference/scene/Scene.cpp',
  'jni/reference/scene/flocking/Boid.cpp',
  'jni/reference/scene/flocking/FlockingScene.cpp',
  'jni/reference/scene/flocking/WaterMeshNode.cpp',
  'jni/reference/scene/glowing/BlurMeshNode.cpp',
  'jni/reference/scene/glowing/GlowingScene.cpp',
)

libctsopengl_jni = custom_target(
  'libctsopengl_jni',
  build_by_default: true,
  input: opengl_sources,
  output: ['libctsopengl_jni.so'],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_cpp_shlib.py',
    '--output', '@OUTPUT@',
    '--sources', '@INPUT@',
    '--include-dirs',
    meson.current_source_dir() / 'jni',
    meson.current_source_dir() / 'jni/graphics',
    meson.current_source_dir() / 'jni/primitive',
    meson.current_source_dir() / 'jni/primitive/contextswitch',
    meson.current_source_dir() / 'jni/primitive/fullpipeline',
    meson.current_source_dir() / 'jni/primitive/pixeloutput',
    meson.current_source_dir() / 'jni/primitive/shaderperf',
    meson.current_source_dir() / 'jni/reference',
    meson.current_source_dir() / 'jni/reference/scene',
    meson.current_source_dir() / 'jni/reference/scene/flocking',
    meson.current_source_dir() / 'jni/reference/scene/glowing',
    '--cxxflag=-DEGL_EGLEXT_PROTOTYPES',
    '--cxxflag=-funsigned-char',
    '--ldflag=-lEGL',
    '--ldflag=-lGLESv2',
    '--ldflag=-landroid',
  ],
)

gtest_main_dep = dependency('gtest_main', required: false)
if not gtest_main_dep.found()
  gtest_main_dep = dependency('gtest', required: false)
endif

if gtest_main_dep.found()
  cts_device_opengl_test = executable(
    'cts_device_opengl_test',
    files(
      'test/MatrixTest.cpp',
      'jni/graphics/Matrix.cpp',
    ),
    include_directories: include_directories('jni/graphics', 'jni'),
    dependencies: [gtest_main_dep],
    cpp_args: ['-std=c++23'],
    build_by_default: true,
  )
  test('cts-device-opengl-matrix-test', cts_device_opengl_test)
endif

cts_device_opengl_apk = custom_target(
  'CtsDeviceOpenGl-apk',
  output: 'CtsDeviceOpenGl.apk',
  input: [libctsopengl_jni, ctsdeviceutil_jar, ctscommonutil_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
    '--assets-dir', meson.current_source_dir() / 'assets',
    '--source-root', meson.current_source_dir() / 'src',
    '--dependency-jar', '@INPUT1@',
    '--dependency-jar', '@INPUT2@',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
    '--jni-lib', '@INPUT0@',
  ],
  build_by_default: true,
  install: false,
)
