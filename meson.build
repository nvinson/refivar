project('refivar', 'rust',
  version : '0.1',
  default_options : ['warning_level=3'])


r_lib_argparse = static_library('argparse', 'src/lib/rust-argparse/src/lib.rs',
                                rust_crate_type: 'lib', install: false)

r_lib_efivar = static_library('efivar', 'src/lib/efivar/mod.rs',
                              rust_crate_type: 'lib', install: false)

# static_library('efivar', 'efivar.rs', rust_crate_type: 'staticlib',
#                install : true)
# shared_library('efivar', 'efivar.rs', rust_crate_type: 'cdylib',
#                install : true)

executable('efivar', 'src/bin/efivar.rs',
           link_with: [r_lib_efivar, r_lib_argparse], install: true)
