#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate walkdir;
extern crate regex;
extern crate clap;
#[cfg(test)]
extern crate tempdir;

mod replace;
mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Regex(::regex::Error);
        }
    }
}

use replace::{replace, restore_backup};
use errors::*;

quick_main!(run);

const BACKUP_DIR: &str = "./backup/";

fn run() -> Result<()> {
    let args = clap::App::new("Recholera")
        .version("0.1.0")
        .author("Christian Dürr <contact@christianduerr.com>")
        .about("A simple and safe way to change colors")
        .arg(clap::Arg::with_name("FILE")
                 .help("File which will be changed")
                 .required_unless("revert")
                 .index(1))
        .arg(clap::Arg::with_name("CURRENT_COLOR")
                 .help("The current color")
                 .required_unless("revert")
                 .index(2))
        .arg(clap::Arg::with_name("NEW_COLOR")
                 .help("The new color")
                 .required_unless("revert")
                 .index(3))
        .arg(clap::Arg::with_name("revert")
                 .long("revert")
                 .conflicts_with("FILE")
                 .short("r")
                 .help("Revert all changes from backup"))
        .get_matches();

    if let Some(file) = args.value_of("FILE") {
        let current_color = args.value_of("CURRENT_COLOR").unwrap();
        let new_color = args.value_of("NEW_COLOR").unwrap();
        replace(file, current_color, new_color, BACKUP_DIR)?;
    } else {
        restore_backup(BACKUP_DIR)?;
    }

    Ok(())
}
