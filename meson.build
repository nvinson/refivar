project('refivar', 'rust',
  version : '0.1',
  default_options : ['warning_level=3'])


# Third-party libraries
tp_r_lib_argparse = subproject('rust-argparse-v0.2.2_p20181221').get_variable('tp_r_lib_rust_argparse')

r_lib_efivar = static_library('efivar', 'src/lib/efivar/mod.rs', rust_crate_type: 'lib', install: false)
#a_lib_efivar = static_library('efivar', 'efivar.rs', rust_crate_type: 'staticlib', install : true)
#so_lib_efivar = shared_library('efivar', 'efivar.rs', rust_crate_type: 'cdylib', install : true)

executable('efivar', 'src/bin/efivar.rs', link_with: [r_lib_efivar, tp_r_lib_argparse], install: true)
