project('refivar', 'rust',
  version : '0.1',
  default_options : ['warning_level=3'])

#static_library('efivar', 'efivar.rs', rust_crate_type: 'staticlib', install : true)
#shared_library('efivar', 'efivar.rs', rust_crate_type: 'cdylib', install : true)

r_lib_efivar = static_library('efivar', 'src/lib/efivar/mod.rs', rust_crate_type: 'lib', install: false)
#executable('refivar_test', 'src/bin/test.rs', link_with: [r_lib_efivar], install: true)
