project(
  'unicode-ident-rs',
  'rust',
  version: '1.0.8',
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
  '--cfg', 'feature="default"',
]

lib_unicode_ident = static_library('unicode_ident', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [],
  pic: true,
)

unicode_ident_rs_dep = declare_dependency(link_with: lib_unicode_ident)
