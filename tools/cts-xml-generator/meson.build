custom_target(
  'cts-xml-generator-jar',
  output: 'cts-xml-generator.jar',
  command: [
    find_program('python3'),
    meson.project_source_root() / 'tools/meson/build_java_jar.py',
    '--output', '@OUTPUT@',
    '--main-class', 'com.android.cts.xmlgenerator.CtsXmlGenerator',
    '--source-root', meson.project_source_root() / 'tools/cts-xml-generator/src',
    '--source-root', meson.project_source_root() / 'libs/vogar-expect/src',
    '--source-root', meson.project_source_root() / 'libs/json/src',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/devtools/tools/lib/guava-13.0.1.jar',
  ],
  build_by_default: true,
  install: false,
)
