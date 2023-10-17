project(
  'refivar',
  'rust',
  version: '0.1',
  meson_version: '>= 1.4.0',
  default_options: {
    'warning_level': '3',
    'rust_std': '2021'
  }
)

fs = import('fs')
rust = import('rust')

rustc = meson.get_compiler('rust')
if rustc.version().version_compare('<1.70')
  error('rustc 1.70 required. Found ' + rustc.version())
endif

clap_rs_dep = dependency('clap-4-rs', version: '>=4.5.8')
ignore_result_rs_dep = dependency('ignore-result-0.2-rs', version: '>=0.2.0')
indoc_rs_dep = dependency('indoc-2-rs', version: '>=2.0.5')
serde_json_rs_dep = dependency('serde_json-1-rs', version: '>=1.0.119')
serde_rs_dep = dependency('serde-1-rs', version: '>=1.0.203')

fqDataDir = get_option('prefix') / get_option('datadir') / 'refivar'

lib_refivar_guids_json = (
  meson.project_source_root() / 'src' / 'lib' / 'efivar' / 'guids.json'
)
lib_refivar_guids_json_install_path = (
  get_option('prefix') / get_option('datadir') / 'refivar' / fs.name(lib_refivar_guids_json)
)

lib_refivar_guids_list_path_rs_conf_data = configuration_data()
lib_refivar_guids_list_path_rs_conf_data.set(
  'GUIDS_LIST_PATH',
  lib_refivar_guids_json_install_path
)

lib_refivar_efi_guids_list_path_rs = configure_file(
  output: 'efi_guids_list_path.rs',
  input: 'src/lib/efivar/efi_guids_list_path.rs.in',
  configuration: lib_refivar_guids_list_path_rs_conf_data
)

install_data(
  lib_refivar_guids_json,
  install_dir : fs.parent(lib_refivar_guids_json_install_path)
)

lib_refivar = static_library(
  'efivar',
  structured_sources(
    [
      'src/lib/efivar/mod.rs',
      'src/lib/efivar/efi_guids.rs',
      'src/lib/efivar/efi_variable_attributes.rs',
      lib_refivar_efi_guids_list_path_rs
    ],
    {
      'efivarfs': [
        'src/lib/efivar/efivarfs/efi_variables.rs',
        'src/lib/efivar/efivarfs/mod.rs',
      ],
      'efivar': [
        'src/lib/efivar/efivar/efi_variables.rs',
        'src/lib/efivar/efivar/mod.rs',
      ],
      'print_mode': [
        'src/lib/efivar/print_mode/decimal.rs',
        'src/lib/efivar/print_mode/mod.rs',
        'src/lib/efivar/print_mode/verbose.rs',
      ],
      'types': [
        'src/lib/efivar/types/efi_guid_error.rs',
        'src/lib/efivar/types/efi_guid_list_entry.rs',
        'src/lib/efivar/types/efi_guid.rs',
        'src/lib/efivar/types/efi_variable_attribute.rs',
        'src/lib/efivar/types/efi_variable.rs',
        'src/lib/efivar/types/mod.rs',
        'src/lib/efivar/types/print_mode.rs',
      ],
    }
  ),
  rust_abi: 'rust',
  dependencies: [
    indoc_rs_dep,
    serde_json_rs_dep,
    serde_rs_dep,
  ],
  install: false,
)

# static_library('efivar', 'efivar.rs', rust_crate_type: 'staticlib',
#                install : true)
# shared_library('efivar', 'efivar.rs', rust_crate_type: 'cdylib',
#                install : true)

executable(
  'efivar',
  'src/bin/efivar.rs',
  link_with: lib_refivar,
  dependencies: [
    clap_rs_dep,
    ignore_result_rs_dep,
    serde_json_rs_dep,
    serde_rs_dep,
  ],
  install: true
)

rust.test('unit tests', lib_refivar, rust_args:[])
