#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate regex;

mod replace;
mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Regex(::regex::Error);
        }
    }
}

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    Ok(())
}
