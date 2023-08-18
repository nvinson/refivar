project(
  'linux-raw-sys-rs',
  'rust',
  version: '0.3.8',
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
  '--cfg', 'feature="general"',
  '--cfg', 'feature="errno"',
  '--cfg', 'feature="ioctl"',
  '--cfg', 'feature="no_std"'
]

lib_linux_raw_sys = static_library('linux_raw_sys', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [],
  pic: true,
)

linux_raw_sys_rs_dep = declare_dependency(link_with: lib_linux_raw_sys)
