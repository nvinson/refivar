project(
  'serde-rs',
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

serde_derive_rs_dep = dependency('serde_derive-rs', version: '=1.0.163')

features = [
  '--cfg', 'feature="std"',
  '--cfg', 'feature="derive"',
  '--cfg', 'feature="serde_derive"',
]

lib_serde = static_library('serde', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    serde_derive_rs_dep,
  ],
  pic: true,
)

serde_rs_dep = declare_dependency(
  link_with: lib_serde,
  dependencies: [
    serde_derive_rs_dep,
  ]
)
