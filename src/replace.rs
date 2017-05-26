use std::io::{Read, Write};
use regex::RegexBuilder;
use std::{fs, path};
use errors::*;

pub fn replace(path: &str, old: &str, new: &str, backup_dir: &str) -> Result<()> {
    let file_old = load_file(path)?;

    // Create backup
    let mut backup_file = String::from(backup_dir) + path;
    backup_file = backup_file.replace("//", "/");
    save_file(&backup_file, &file_old)?;

    // Replace and save
    let mut reb = RegexBuilder::new(old);
    let _ = reb.case_insensitive(true);
    let re = reb.build()?;
    let file_new = re.replace_all(&file_old, new);

    save_file(path, &file_new)
}

// Save a file and create required directories
fn save_file(path: &str, data: &str) -> Result<()> {
    let path = path::Path::new(path);
    let parent = path.parent().unwrap_or(path::Path::new(""));
    fs::create_dir_all(parent)?;
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

#[cfg(test)]
use tempdir;

#[cfg(test)]
fn get_tmp_path(tmp_dir: &tempdir::TempDir, file_name: &str) -> String {
    let file_path = tmp_dir.path().join(file_name);
    file_path.to_str().unwrap().to_string()
}

#[test]
fn test_load_file() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let file_path = get_tmp_path(&tmp_dir, "load_file");
    let _ = fs::File::create(&file_path).and_then(|mut f| f.write_all(b"test"));
    let output = load_file(&file_path).unwrap();

    assert_eq!(output, "test");
}

#[test]
fn test_save_file() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let file_path = get_tmp_path(&tmp_dir, "save_file");
    let _ = save_file(&file_path, "test");
    let output = load_file(&file_path);

    assert_eq!(output.unwrap(), "test");
}

#[test]
fn test_save_creating_dirs() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let file_path = get_tmp_path(&tmp_dir, "save_creating_dirs");
    let _ = save_file(&file_path, "test");
    let output = load_file(&file_path);

    assert_eq!(output.unwrap(), "test");
}

#[test]
fn test_replace() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let file_path = get_tmp_path(&tmp_dir, "replace");
    let backup_path = get_tmp_path(&tmp_dir, "backup/");
    let _ = save_file(&file_path, "#FF00FF");
    let _ = replace(&file_path, "#FF00FF", "#00FF00", &backup_path);
    let output = load_file(&file_path);

    assert_eq!(output.unwrap(), "#00FF00");
}

#[test]
fn test_replace_case_insensitive() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let file_path = get_tmp_path(&tmp_dir, "replace_case_insensitive");
    let backup_path = get_tmp_path(&tmp_dir, "backup/");
    let _ = save_file(&file_path, "#FF00FF");
    let _ = replace(&file_path, "#ff00ff", "#00ff00", &backup_path);
    let output = load_file(&file_path);

    assert_eq!(output.unwrap(), "#00ff00");
}

#[test]
fn test_replace_creating_backup() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let file_path = get_tmp_path(&tmp_dir, "replace_creating_backup");
    let backup_path = get_tmp_path(&tmp_dir, "backup/");
    let _ = save_file(&file_path, "#ff00ff");
    let _ = replace(&file_path, "#ff00ff", "#00ff00", &backup_path);
    let output = load_file(&(backup_path + &file_path));

    assert_eq!(output.unwrap(), "#ff00ff");
}
