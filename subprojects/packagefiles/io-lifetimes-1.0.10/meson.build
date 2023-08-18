project(
  'io-lifetimes-rs',
  'rust',
  version: '1.0.10',
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

features = []
if rustc.get_id().contains('clippy')
  features += [ '--allow=clippy::all' ]
endif
features += [
  '--cfg', 'feature="close"'
]

libc_rs_dep = dependency('libc-rs', version: '>=0.2.96')

lib_io_lifetimes = static_library('io_lifetimes', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    libc_rs_dep
  ],
  pic: true,
)

io_lifetimes_rs_dep = declare_dependency(
  link_with: lib_io_lifetimes,
  dependencies: [
    libc_rs_dep
  ]
)
