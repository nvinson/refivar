project(
  'clap_lex-rs',
  'rust',
  version: '0.5.0',
  meson_version: '>= 1.1.0',
  default_options: [
    'buildtype=debugoptimized',
    'rust_std=2015'
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
  '--cfg', 'feature="std"',
]

lib_clap_lex = static_library('clap_lex', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [],
  pic: true,
)

clap_lex_rs_dep = declare_dependency(link_with: lib_clap_lex)
