project(
  'terminal_size-rs',
  'rust',
  version: '0.2.6',
  meson_version: '>= 1.1.0',
  default_options: [
    'buildtype=debugoptimized',
    'rust_std=2018'
  ]
)

rustc = meson.get_compiler('rust')
if rustc.version().version_compare('<1.69')
  error('rustc 1.69 required. Found ' + rustc.version())
endif

features = [
  '--cfg', 'feature="default"'
]

rustix_rs_dep = dependency('rustix-rs', version: '>=0.37.0')

lib_terminal_size = static_library('terminal_size', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    rustix_rs_dep
  ],
  pic: true,
)

terminal_size_rs_dep = declare_dependency(
  link_with: lib_terminal_size,
  dependencies: [
    rustix_rs_dep
  ]
)
