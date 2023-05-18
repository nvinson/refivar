project(
  'anstyle-rs',
  'rust',
  version: '1.0.0',
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
  '--cfg', 'feature="std"'
]

lib_anstyle = static_library('anstyle', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  pic: true,
)

anstyle_rs_dep = declare_dependency(link_with: lib_anstyle)
