project('cat_sock', 'rust', version : '0.1.0')
cat_sock = executable(
  'cat_sock',
  sources: 'src/main.rs',
  rust_args: ['--edition=2021',
    '-C', 'panic=abort',
    '-C', 'lto',
    '-C', 'codegen-units=1',
    '-C', 'strip=symbols'
  ],
  install : true
)
