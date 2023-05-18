project(
  'bitflags-rs',
  'rust',
  version: '1.3.2',
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

lib_bitflags = static_library('bitflags', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  pic: true,
)

bitflags_rs_dep = declare_dependency(link_with: lib_bitflags)
