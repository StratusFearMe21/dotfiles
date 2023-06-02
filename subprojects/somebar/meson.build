project('somebar', ['c', 'cpp'],
	version: '0.1.0',
	default_options: [
	    'cpp_std=c++17',
	    'cpp_args=-Wno-parentheses',
	])

cc = meson.get_compiler('c')
wayland_dep = dependency('wayland-client')
wayland_cursor_dep = dependency('wayland-cursor')
cairo_dep = dependency('cairo')
dbus = dependency('dbus-1')
pango_dep = dependency('pango')
pangocairo_dep = dependency('pangocairo')

subdir('protocols')

rustbar = subproject('rust-workspace')
rustbar_dep = rustbar.get_variable('rustbar')

executable('somebar',
	'src/main.cpp',
	'src/shm_buffer.cpp',
	'src/bar.cpp',
	wayland_sources,
	dependencies: [
	    wayland_dep,
			dbus,
	    wayland_cursor_dep,
	    cairo_dep,
	    pango_dep,
	    pangocairo_dep,
			rustbar_dep,
	],
	install: true,
	cpp_args: '-DSOMEBAR_VERSION="@0@"'.format(meson.project_version()))

install_man('somebar.1')
