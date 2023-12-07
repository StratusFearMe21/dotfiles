project('rust-workspace', 'rust', meson_version : '>=0.56.0')

mod = import('unstable-external_project')

p = mod.add_project(
  'sh',
  configure_options: ['-c', 'echo rust:\\\\n\\\\tcargo build --release --target-dir . --manifest-path $(dirname "$0")/Cargo.toml\\\\ninstall: rust\\\\n\\\\tmkdir -p \$\(DESTDIR\)@PREFIX@/@LIBDIR@\\\\n\\\\tmkdir -p \$\(DESTDIR\)@PREFIX@/bin\\\\n\\\\tinstall -m 755 release/rustbar \$\(DESTDIR\)@PREFIX@/bin\\\\n\\\\tinstall -m 755 release/liblistenbrainz_mpv.so \$\(DESTDIR\)@PREFIX@/@LIBDIR@\\\\n\\\\tinstall -m 755 release/libdbus_dwl.a \$\(DESTDIR\)@PREFIX@/@LIBDIR@\\\\n\\\\tinstall -m 755 release/tuigreet \$\(DESTDIR\)@PREFIX@/bin\\\\n\\\\tinstall -m 755 release/s6-manager \$\(DESTDIR\)@PREFIX@/bin\\\\n\\\\tinstall -m 755 release/chromiumpsd \$\(DESTDIR\)@PREFIX@/bin\\\\n\\\\ttouch $(dirname "$0")/../dwl/meson.build\\\\n\\\\ttouch $srcdir/rust-build > Makefile'],
  verbose: true
)

dbus_dwl = p.dependency('dbus_dwl')
