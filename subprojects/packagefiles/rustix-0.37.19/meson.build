project(
  'rustix-rs',
  'rust',
  version: '0.37.19',
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
  '--cfg', 'feature="termios"',
  '--cfg', 'linux_raw',
  '--cfg', 'asm',
  '--cfg', 'linux_like',
]

bitflags_rs_dep = dependency('bitflags-rs', version: ['>=1.3.2', '<2.0.0'])
io_lifetimes_rs_dep = dependency('io-lifetimes-rs', version: '>=1.0.10')
libc_rs_dep = dependency('libc-rs', version: '>=0.2.144')
linux_raw_sys_rs_dep = dependency('linux-raw-sys-rs', version: '>=0.3.6')

lib_rustix = static_library('rustix', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    bitflags_rs_dep,
    io_lifetimes_rs_dep,
    libc_rs_dep,
    linux_raw_sys_rs_dep,
  ],
  pic: true,
)

rustix_rs_dep = declare_dependency(
  link_with: lib_rustix,
  dependencies: [
    bitflags_rs_dep,
    io_lifetimes_rs_dep,
    libc_rs_dep,
    linux_raw_sys_rs_dep
  ]
)
