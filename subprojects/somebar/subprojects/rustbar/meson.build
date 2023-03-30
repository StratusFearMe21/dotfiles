project('rustbar', 'rust', meson_version : '>=0.56.0')

mod = import('unstable-external_project')

p = mod.add_project('sh', configure_options: ['-c', 'echo .PHONY: rustbar\\\\nrustbar:\\\\n\\\\tcargo build --release --lib --target-dir . --manifest-path $(dirname "$0")/Cargo.toml\\\\ninstall: rustbar\\\\n\\\\tmkdir -p \$\(DESTDIR\)@PREFIX@/@LIBDIR@\\\\n\\\\t install -m 755 release/librustbar.a \$\(DESTDIR\)@PREFIX@/@LIBDIR@ > Makefile'])

rustbar = p.dependency('rustbar')
