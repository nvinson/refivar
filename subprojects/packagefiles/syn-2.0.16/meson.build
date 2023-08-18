project(
  'syn-rs',
  'rust',
  version: '2.0.16',
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
  '--cfg', 'feature="default"',
  '--cfg', 'feature="derive"',
  '--cfg', 'feature="parsing"',
  '--cfg', 'feature="printing"',
  '--cfg', 'feature="clone-impls"',
  '--cfg', 'feature="proc-macro"'
]

proc_macro2_rs_dep = dependency('proc-macro2-rs', version: '>=1.0.55')
quote_rs_dep = dependency('quote-rs', version: '>=1.0.25')

lib_syn = static_library('syn', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    proc_macro2_rs_dep,
    quote_rs_dep
  ],
  pic: true,
)

syn_rs_dep = declare_dependency(
  link_with: lib_syn,
  dependencies: [
    proc_macro2_rs_dep,
    quote_rs_dep
  ]
)
