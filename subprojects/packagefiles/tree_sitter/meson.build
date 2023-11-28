project('tree_sitter', 'c', default_options: ['c_std=gnu99'])

tree_sitter_lib = library(
  'tree_sitter',
  'lib/src/lib.c',
  include_directories: ['lib/include']
)

tree_sitter_dep = declare_dependency(
  link_with: [tree_sitter_lib],
  include_directories: ['lib/include']
)
