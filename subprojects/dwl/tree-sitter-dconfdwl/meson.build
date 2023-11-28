project('tree-sitter-dconfdwl', 'c')

dconfdwl_lib = library(
  'tree-sitter-dconfdwl',
  'src/parser.c',
  include_directories: ['src']
)

dconfdwl = declare_dependency(
  link_with: [dconfdwl_lib],
  include_directories: ['src']
)
