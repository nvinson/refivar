project(
  'clap_builder-rs',
  'rust',
  version: '4.3.0',
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
  '--cfg', 'feature="std"',
  '--cfg', 'feature="help"',
  '--cfg', 'feature="wrap_help"',
  '--cfg', 'feature="usage"'
]

anstyle_rs_dep = dependency('anstyle-rs', version: '>=1.0.0')
bitflgs_rs_dep = dependency('bitflags-rs', version: ['>=1.2.0', '<2.0.0'])
clap_lex_rs_dep = dependency('clap_lex-rs', version: '>=0.5.0')
terminal_size_rs_dep = dependency('terminal_size-rs', version: '>=0.2.1')

lib_clap_builder = static_library('clap_builder', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    anstyle_rs_dep,
    bitflgs_rs_dep,
    clap_lex_rs_dep,
    terminal_size_rs_dep
  ],
  pic: true,
)

clap_builder_rs_dep = declare_dependency(
  link_with: lib_clap_builder,
  dependencies: [
    anstyle_rs_dep,
    bitflgs_rs_dep,
    clap_lex_rs_dep,
    terminal_size_rs_dep
  ]
)

