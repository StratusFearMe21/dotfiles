project('wpmonitor', 'c', version : '0.1.0')

gobject_dep = dependency('gobject-2.0')
gio_dep = dependency('gio-2.0')
pipewire = dependency('libpipewire-0.3')
wireplumber = dependency('wireplumber-0.5')

gdbus = find_program('gdbus-codegen')

fs = import('fs')

wpmonitor = custom_target(
  'wpmonitor-generated.c',
  output : 'wpmonitor-generated.c',
  input : 'api.xml',
  command : [gdbus, '--c-namespace', 'WpMonitor', '--body', '--output', '@OUTPUT@', '@INPUT@']
)

wpmonitor_header = custom_target(
  'wpmonitor-generated.h',
  output : 'wpmonitor-generated.h',
  input : 'api.xml',
  command : [gdbus, '--c-namespace', 'WpMonitor', '--header', '--output', '@OUTPUT@', '@INPUT@']
)

wireplumber_module_dir = get_option('prefix') / get_option('libdir') / 'wireplumber-0.5'

shared_library('wireplumber-module-wpmonitor',
  'wpmonitor.c',
  wpmonitor,
  wpmonitor_header,
  c_args : ['-D_GNU_SOURCE', '-DG_LOG_USE_STRUCTURED', '-DG_LOG_DOMAIN="wpmonitor"'],
  install_dir : wireplumber_module_dir,
  dependencies : [gobject_dep, gio_dep, pipewire, wireplumber],
  install : true,
  install_tag : 'dotfiles'
)

