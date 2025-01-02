use std::env::args;
use colored::Colorize;
// use itertools::Itertools;

pub mod commands {
    macro_rules! cmd {
        ($name:ident, $full:expr, $short:expr) => {
            pub mod $name {
                pub const FULL: &str = $full;
                pub const SHORT: &str = $short;
            }
        };
    }

    cmd!(help, "--help", "-h");
}

#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("\tno such command: `{0}`\n\tView all available commands with `{full}` or `{shorthand}`\n", full = commands::help::FULL, shorthand = commands::help::SHORT)]
    UnknownCommand(&'a str),
}

impl Error<'_> {
    fn abort_app(self) {
        eprintln!("{} {}", "error:".red(), self);
        std::process::exit(1);
    }
}

fn main() {
    let mut args = args().skip(1);

    if let Some(cmd) = args.next() {
        println!("{}", cmd);
        use commands::*;
        match &cmd[..] {
            help::FULL | help::SHORT => { todo!() }
            _ => Error::UnknownCommand(&cmd).abort_app(),
        }
    } else {
        eprintln!("No arguments!");
    }
}
