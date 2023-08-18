project(
  'libc-rs',
  'rust',
  version: '0.37.19',
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
  '--cfg', 'feature="default"',
  '--cfg', 'libc_priv_mod_use',
  '--cfg', 'libc_union',
  '--cfg', 'libc_const_size_of',
  '--cfg', 'libc_align',
  '--cfg', 'libc_core_cvoid',
  '--cfg', 'libc_packedN',
]

lib_libc = static_library('libc', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [],
  pic: true,
)

libc_rs_dep = declare_dependency(link_with: lib_libc)

