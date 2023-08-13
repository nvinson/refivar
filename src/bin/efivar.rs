use crate::efivar::print_mode::{Decimal, Verbose};
use crate::efivar::types::PrintMode;
use clap;
use efivar;
use ignore_result::Ignore;
use std::io;
use std::process::ExitCode;

fn create_parser() -> clap::Command {
    return clap::Command::new("efivar")
        .args_override_self(true)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .max_term_width(80)
        .term_width(80)
        .arg(clap::Arg::new("attributes")
            .short('A')
            .long("attributes")
            .help("attributes to use on append")
            .action(clap::ArgAction::Set)
        )
        .arg(clap::Arg::new("list")
            .short('l')
            .long("list")
            .help("list current variables")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("print")
            .short('p')
            .long("print")
            .help("Print variable specified by --name")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("dmpstore")
            .short('D')
            .long("dmpstore")
            .help("use DMPSTORE format when exporting")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("print-decimal")
            .short('d')
            .long("print-decimal")
            .help("print variable in decimal values specified by --name")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("name")
            .short('n')
            .long("name")
            .value_name("guid-name")
            .help("variable to manipulate, in the form 8be4df61-93ca-11d2-aa0d-00e098032b8c-Boot0000")
            .action(clap::ArgAction::Set)
        )
        .arg(clap::Arg::new("append")
            .short('a')
            .long("append")
            .help("append to variable specified by --name")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("datafile")
            .short('f')
            .long("datafile")
            .value_name("file")
            .help("load or save variable contents from or to <file>")
            .action(clap::ArgAction::Set)
        )
        .arg(clap::Arg::new("export")
            .short('e')
            .long("export")
            .value_name("file")
            .help("export variable to <file>")
            .action(clap::ArgAction::Set)
        )
        .arg(clap::Arg::new("import")
            .short('i')
            .long("import")
            .value_name("file")
            .help("import variable from <file>")
            .action(clap::ArgAction::Set)
        )
        .arg(clap::Arg::new("guids-list-path")
            .short('g')
            .long("guids-list-path")
            .value_name("guids-list-path")
            .default_value(efivar::efi_guids::DEFAULT_GUIDS_LIST_PATH)
            .help(format!("specify path to GUIDs list file."))
            .action(clap::ArgAction::Set)
        )
        .arg(clap::Arg::new("list-guids")
            .short('L')
            .long("list-guids")
            .help("show GUID list")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("write")
            .short('w')
            .long("write")
            .help("Write to variable specified by --name")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("help")
            .short('?')
            .long("help")
            .help("Show this help message")
            .action(clap::ArgAction::Help)
        )
        .arg(clap::Arg::new("usage")
            .short(None)
            .long("usage")
            .help("ignored for compatibility")
            .action(clap::ArgAction::Help)
        );
}

fn list_variables(_parser_args: clap::ArgMatches) -> ExitCode {
    let mut efivar_fs_variables: efivar::efivarfs::EfiVariables =
        efivar::efivarfs::EfiVariables::new();

    match efivar_fs_variables.list() {
        Ok(variables) => {
            for v in variables {
                println!("{}", v);
            }
            return std::process::ExitCode::from(0);
        }
        Err(_) => {
            let efivar_variables: efivar::efivar::EfiVariables =
                efivar::efivar::EfiVariables::new();
            match efivar_variables.list() {
                Ok(variables) => {
                    for v in variables {
                        println!("{}", v);
                    }
                    return std::process::ExitCode::from(0);
                }
                Err(e) => {
                    eprintln!("Failed to access EFI variables: {}", e);
                    return std::process::ExitCode::from(1);
                }
            }
        }
    }
}

fn print_variable(parser_args: clap::ArgMatches, print_mode: efivar::types::PrintMode) -> ExitCode {
    match parser_args.get_one::<String>("name") {
        Some(name) => {
            let efivar_fs_variables: efivar::efivarfs::EfiVariables =
                efivar::efivarfs::EfiVariables::new();
            match efivar_fs_variables.get_variable(name) {
                Ok(var) => {
                    match print_mode {
                        PrintMode::VERBOSE => println!("{}", Verbose(&var)),
                        PrintMode::DECIMAL => println!("{}", Decimal(&var)),
                    }
                    return std::process::ExitCode::from(0);
                }
                Err(_) => {
                    let efivar_variables: efivar::efivar::EfiVariables =
                        efivar::efivar::EfiVariables::new();
                    match efivar_variables.get_variable(name) {
                        Ok(var) => {
                            match print_mode {
                                PrintMode::VERBOSE => println!("{}", Verbose(&var)),
                                PrintMode::DECIMAL => println!("{}", Decimal(&var)),
                            }
                            return std::process::ExitCode::from(0);
                        }
                        Err(e) => {
                            eprintln!("Failed to read variable: {}", e);
                            return std::process::ExitCode::from(1);
                        }
                    }
                }
            };
        }
        None => {
            eprintln!("No variable name given");
            return std::process::ExitCode::from(1);
        }
    };
}

fn append_attributes(parser_args: clap::ArgMatches) -> ExitCode {
    return std::process::ExitCode::from(0);
}

fn list_guids(parser_args: clap::ArgMatches) -> ExitCode {
    let mut guid_list: efivar::efi_guids::EfiGuidList = Default::default();
    match guid_list.load(parser_args.get_one("guids-list-path").unwrap()) {
        Ok(()) => {
            for g in guid_list.guids(efivar::efi_guids::GuidListSortField::Guid) {
                println!("{}", g);
            }
        }
        Err(e) => {
            eprintln!("Failed to read GUIDs list file: {}", e);
            return std::process::ExitCode::from(e.raw_os_error().unwrap_or(1) as u8);
        }
    }
    return std::process::ExitCode::from(0);
}

fn write_variable(parser_args: clap::ArgMatches) -> ExitCode {
    return std::process::ExitCode::from(0);
}

fn import_variable(parser_args: clap::ArgMatches) -> ExitCode {
    return std::process::ExitCode::from(0);
}

fn export_variable(parser_args: clap::ArgMatches) -> ExitCode {
    return std::process::ExitCode::from(0);
}

fn main() -> ExitCode {
    let mut parser = create_parser();
    let matches = parser.get_matches_mut();
    if matches.get_flag("list") {
        return list_variables(matches);
    } else if matches.get_flag("print") {
        return print_variable(matches, efivar::types::PrintMode::VERBOSE);
    } else if matches.get_flag("append") {
        return append_attributes(matches);
    } else if matches.get_flag("list-guids") {
        return list_guids(matches);
    } else if matches.get_flag("write") {
        return write_variable(matches);
    } else if matches.get_flag("print-decimal") {
        return print_variable(matches, efivar::types::PrintMode::DECIMAL);
    } else if matches.get_one::<&str>("import").is_some() {
        return import_variable(matches);
    } else if matches.get_one::<&str>("export").is_some() {
        return export_variable(matches);
    } else {
        if matches.get_one::<String>("name").is_some() {
            return print_variable(matches, efivar::types::PrintMode::VERBOSE);
        } else {
            parser.write_help(&mut io::stderr()).ignore();
            return std::process::ExitCode::from(1);
        }
    }
}
