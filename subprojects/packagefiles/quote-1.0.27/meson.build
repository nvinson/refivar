project(
  'quote-rs',
  'rust',
  version: '4.3.0',
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
  '--cfg', 'feature="default"',
  '--cfg', 'feature="proc-macro"',
]

proc_macro2_rs_dep = dependency('proc-macro2-rs', version: '>=1.0.42')

lib_quote = static_library('quote', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    proc_macro2_rs_dep
  ],
  pic: true,
)

quote_rs_dep = declare_dependency(
  link_with: lib_quote,
  dependencies: [
    proc_macro2_rs_dep
  ]
)
