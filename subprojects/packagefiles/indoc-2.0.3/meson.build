project(
  'indoc-rs',
  'rust',
  version: '2.0.3',
  meson_version: '>= 1.1.0',
  default_options: [
    'buildtype=debugoptimized',
    'rust_std=2021'
  ]
)

rustc = meson.get_compiler('rust')
if rustc.version().version_compare('<1.69')
  error('rustc 1.69 required. Found ' + rustc.version())
endif

features = []
if rustc.get_id().contains('clippy')
  features += [ '--allow=clippy::all' ]
endif
features += [
  '--extern', 'proc_macro',
]

unindent_rs_dep = dependency('unindent-rs', version: '>=0.2.1')

lib_indoc = shared_library('indoc', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'proc-macro',
  dependencies: [
    unindent_rs_dep,
  ],
)

indoc_rs_dep = declare_dependency(
  link_with: lib_indoc,
  dependencies: [
    unindent_rs_dep
  ]
)
