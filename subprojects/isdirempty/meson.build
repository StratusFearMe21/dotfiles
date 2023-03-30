project('isdirempty', 'rust', version : '0.1.0')
cat_sock = executable(
  'isdirempty',
  sources: 'src/main.rs',
  rust_args: ['--edition=2021',
    '-C', 'panic=abort',
    '-C', 'lto',
    '-C', 'codegen-units=1',
    '-C', 'strip=symbols'
  ],
  install : true
)
