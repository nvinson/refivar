project(
  'serde_derive-rs',
  'rust',
  version: '1.0.163',
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
  '--cfg', 'feature="default"'
]

proc_macro2_rs_dep = dependency('proc-macro2-rs', version: '>=1.0')
quote_rs_dep = dependency('quote-rs', version: '>=1.0')
syn_rs_dep = dependency('syn-rs', version: '>=2.0.3')

lib_serde_derive = library('serde_derive', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'proc-macro',
  dependencies: [
    proc_macro2_rs_dep,
    quote_rs_dep,
    syn_rs_dep
  ],
  pic: true,
)

serde_derive_rs_dep = declare_dependency(
  link_with: lib_serde_derive,
  dependencies: [
    proc_macro2_rs_dep,
    quote_rs_dep,
    syn_rs_dep
  ]
)
