project(
  'proc-macro2-rs',
  'rust',
  version: '1.0.58',
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
  '--cfg', 'feature="proc-macro"',
  '--cfg', 'use_proc_macro',
  '--cfg', 'wrap_proc_macro'
]

unicode_ident_rs_dep = dependency('unicode-ident-rs', version: '>=1.0')

lib_proc_macro2 = static_library('proc_macro2', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    unicode_ident_rs_dep
  ],
  pic: true,
)

proc_macro2_rs_dep = declare_dependency(
  link_with: lib_proc_macro2,
  dependencies: [
    unicode_ident_rs_dep
  ]
)
