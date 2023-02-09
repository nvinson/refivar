extern crate efivar;
extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, StoreFalse};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut verbose = false;
    let mut parser = ArgumentParser::new();

    parser.refer(&mut verbose)
        .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose")
        .add_option(&["-q", "--quiet"], StoreFalse, "Be quiet");

    match parser.parse_args() {
        Ok(()) =>  {
            std::process::ExitCode::from(0)
        }
        Err(x) => {
            std::process::ExitCode::from(x as u8)
        }
    }
}
