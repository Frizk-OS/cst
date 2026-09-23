project(
  'cts-usb-accessory',
  'c',
  version: '1.0.0',
  meson_version: '>=1.3.0',
  default_options: [
    'c_std=c23',
    'warning_level=2',
    'buildtype=debugoptimized',
  ],
)

cc = meson.get_compiler('c')
usbhost_dep = cc.find_library('usbhost', required: false)
cutils_dep = cc.find_library('cutils', required: false)

if usbhost_dep.found() and cutils_dep.found()
  executable(
    'cts-usb-accessory',
    'cts-usb-accessory.c',
    dependencies: [usbhost_dep, cutils_dep],
    include_directories: include_directories('bionic/libc/kernel/common'),
    install: true,
  )
else
  message('Skipping cts-usb-accessory: Android libusbhost and libcutils are required')
endif