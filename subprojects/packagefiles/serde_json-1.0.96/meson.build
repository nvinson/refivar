project(
  'serde_json-rs',
  'rust',
  version: '1.0.96',
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

if (['aarch64', 'mips64', 'ppc64', 'x86_64']).contains(host_machine.cpu_family())
  fpu_limb_width = 64
else
  fpu_limb_width = 32
endif
message('FPU limb width set to @0@'.format(fpu_limb_width))

features = [
  '--cfg', 'limb_width_@0@'.format(fpu_limb_width),
  '--cfg', 'feature="std"'
]

serde_rs_dep = dependency('serde-rs', version: '>=1.0.160')
itoa_rs_dep = dependency('itoa-rs', version: '>=1.0.0')
ryu_rs_dep = dependency('ryu-rs', version: '>=1.0.0')

lib_serde_json = static_library('serde_json', 'src/lib.rs',
  rust_args: features,
  rust_crate_type: 'rlib',
  dependencies: [
    serde_rs_dep,
    itoa_rs_dep,
    ryu_rs_dep
  ],
  pic: true
)

serde_json_rs_dep = declare_dependency(
  link_with: lib_serde_json,
  dependencies: [
    serde_rs_dep,
    itoa_rs_dep,
    ryu_rs_dep
  ]
)
