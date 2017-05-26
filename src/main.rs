#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
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

use replace::replace;
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let args = clap::App::new("Recholera")
        .version("0.1.0")
        .author("Christian Dürr <contact@christianduerr.com>")
        .about("A simple and safe way to change colors")
        .arg(clap::Arg::with_name("FILE")
                 .help("File which will be changed")
                 .required(true)
                 .index(1))
        .arg(clap::Arg::with_name("CURRENT_COLOR")
                 .help("The current color")
                 .required(true)
                 .index(2))
        .arg(clap::Arg::with_name("NEW_COLOR")
                 .help("The new color")
                 .required(true)
                 .index(3))
        .get_matches();

    let file = args.value_of("FILE").unwrap();
    let current_color = args.value_of("CURRENT_COLOR").unwrap();
    let new_color = args.value_of("NEW_COLOR").unwrap();
    replace(file, current_color, new_color, "./backup/")?;

    Ok(())
}
