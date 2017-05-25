use std::io::{Read, Write};
use errors::*;
use std::fs;
use regex::RegexBuilder;

fn replace(path: &str, old: &str, new: &str) -> Result<()> {
    let file_old = load_file(path)?;

    let mut reb = RegexBuilder::new(old);
    let _ = reb.case_insensitive(true);
    let re = reb.build()?;
    let file_new = re.replace_all(&file_old, new);

    save_file(path, &file_new)
}

fn save_file(path: &str, data: &str) -> Result<()> {
    fs::File::create(path)?.write_all(data.as_bytes())?;
    Ok(())
}

fn load_file(path: &str) -> Result<String> {
    let mut buf = String::new();
    fs::File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf.trim().to_owned())
}

/**********************/
/*******  TESTS *******/
/**********************/

#[test]
fn test_load_file() {
    let output = load_file("./test/load_file").unwrap();

    assert_eq!(output, "test");
}

#[test]
fn test_save_file() {
    let data = "test";
    let _ = save_file("./test/save_file", data);
    let output = load_file("./test/save_file");

    fs::remove_file("./test/save_file").unwrap();
    assert_eq!(output.unwrap(), data);
}

#[test]
fn test_replace() {
    let _ = save_file("./test/replace", "#FF00FF");
    let _ = replace("./test/replace", "#FF00FF", "#00FF00");
    let output = load_file("./test/replace");

    fs::remove_file("./test/replace").unwrap();
    assert_eq!(output.unwrap(), "#00FF00");
}

#[test]
fn test_replace_case_insensitive() {
    let _ = save_file("./test/replace_case_insensitive", "#FF00FF");
    let _ = replace("./test/replace_case_insensitive", "#ff00ff", "#00ff00");
    let output = load_file("./test/replace_case_insensitive");

    fs::remove_file("./test/replace_case_insensitive").unwrap();
    assert_eq!(output.unwrap(), "#00ff00");
}
