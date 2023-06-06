project('dwl', ['c', 'cpp'],
  version : '0.4'
)

subproject('libliftoff', default_options: 'default_library=static')
subproject('wlroots', default_options : ['examples=false', 'default_library=static'])
subdir('protocols')

cc = meson.get_compiler('c')
libm = cc.find_library('m')
xcb = dependency('xcb', required : get_option('xwayland'))
xlibs = dependency('xcb-icccm', required : get_option('xwayland'))
wayland_server_dep = dependency('wayland-server')
wlroots_dep = dependency('wlroots')
xkbcommon_dep = dependency('xkbcommon')
libinput_dep = dependency('libinput')
dconf_dep = dependency('dconf')
dbus_dep = dependency('dbus-1')

dbus_dwl = subproject('rust-workspace')
dbus_dwl_dep = dbus_dwl.get_variable('dbus_dwl')

c_args = [
  '-DWLR_USE_UNSTABLE',
  '-D_POSIX_C_SOURCE=200809L',
  '-DVERSION="@0@"'.format(meson.project_version())
]

if xcb.found() and xlibs.found()
  c_args += '-DXWAYLAND'
 endif

executable('dwl', 
  'dwl.c',
  'util.c',
  wayland_sources,
  dependencies : [
    libm,
    xcb,
    xlibs,
    dconf_dep,
    dbus_dep,
    dbus_dwl_dep,
    wayland_server_dep,
    wlroots_dep,
    xkbcommon_dep,
    libinput_dep,
  ],
  install : true,
  c_args : c_args
)

install_man('dwl.1')
