project(
  'ignore-result-rs',
  'rust',
  version: '0.2.0',
  meson_version: '>=1.1.0',
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

lib_ignore_result = static_library('ignore_result', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [],
  pic: true,
)

ignore_result_rs_dep = declare_dependency(
  link_with: lib_ignore_result,
  dependencies: []
)
