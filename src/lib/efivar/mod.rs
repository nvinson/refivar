pub mod efi_guids;
pub mod efi_variable_attributes;
pub mod efivar;
pub mod efivarfs;
pub mod print_mode;
pub mod types;

mod efi_guids_list_path;

// variable file names have 1 or more characters, a dash, then a UUID (36 characters)
const MIN_VAR_FILE_NAME_LEN: usize = 38;
