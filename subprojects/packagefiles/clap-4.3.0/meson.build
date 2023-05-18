project(
  'clap-rs',
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

features = [
  '--cfg', 'feature="std"',
  '--cfg', 'feature="help"',
  '--cfg', 'feature="wrap_help"',
  '--cfg', 'feature="usage"'
]

clap_builder_rs_dep = dependency('clap_builder-rs', version: '=4.3.0')

lib_clap = static_library('clap', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    clap_builder_rs_dep,
  ],
  pic: true,
)

clap_rs_dep = declare_dependency(
  link_with: lib_clap,
  dependencies: [
    clap_builder_rs_dep,
  ]
)
