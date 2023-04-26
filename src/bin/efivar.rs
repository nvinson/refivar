use clap;
use efivar::efi_guids;
use std::process::ExitCode;

fn create_parser() -> clap::Command {
    return clap::command!()
        .args_override_self(true)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .max_term_width(90)
        .term_width(90)
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
            .default_value(efi_guids::DEFAULT_GUIDS_LIST_PATH)
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

fn main() -> ExitCode {
    let matches = create_parser().get_matches();
    if matches.get_flag("list-guids") {
        let mut guid_list: efi_guids::EfiGuidList = Default::default();
        guid_list.load(matches.get_one("guids-list-path").unwrap());
        for g in guid_list.guids(efi_guids::GuidListSortField::Guid) {
            println!("{}", g);
        }
        println!("");
        for g in guid_list.guids(efi_guids::GuidListSortField::Id) {
            println!("{}", g);
        }
        println!("");
        for g in guid_list.guids(efi_guids::GuidListSortField::None) {
            println!("{}", g);
        }
    }
    return std::process::ExitCode::from(0);
}
