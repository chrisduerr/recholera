use std::io::{Read, Write, stdin};
use regex::RegexBuilder;
use walkdir::WalkDir;
use std::{fs, path};
use errors::*;

pub fn replace(path: &str, old: &str, new: &str, backup_dir: &str) -> Result<()> {
    let file_old = load_file(path)?;

    // Abort if old color doesn't exist
    // Abort if new color exists and user confirms abortion
    let uppercase_content = file_old.to_uppercase();
    if !uppercase_content.contains(&old.to_uppercase()) {
        println!("\"{}\" doesn't contain \"{}\".", path, old);
        println!("Exited without any change or backup.");
        return Ok(());
    } else if uppercase_content.contains(&new.to_uppercase()) {
        let mut buf = String::new();
        println!("\"{}\" already exists in \"{}\", do you want to keep going? [y/N]?",
                 new,
                 path);
        stdin().read_line(&mut buf)?;
        if buf.to_lowercase().trim() != "y" {
            println!("Exited without any change or backup.");
            return Ok(());
        }
    }

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

pub fn restore_backup(backup_dir: &str) -> Result<()> {
    // Get fixed base path independent of ending with / or not
    let path_base_len = if backup_dir.ends_with("/") {
        backup_dir.len() - 1
    } else {
        backup_dir.len()
    };

    // Go through every entry in the backup directory, restore it if it's a file
    for entry in WalkDir::new(backup_dir).into_iter().filter_map(|e| e.ok()) {
        let backup_path = entry.path().to_str().ok_or("Not a valid path.")?;
        let metadata = fs::metadata(backup_path)?;
        if metadata.is_file() {
            let target_path = &backup_path[path_base_len..];

            let content = load_file(backup_path)?;
            save_file(target_path, &content)?;
        }
    }

    fs::remove_dir_all(backup_dir)?;
    Ok(())
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

#[test]
fn test_restore_backup() {
    let tmp_dir = tempdir::TempDir::new("test").unwrap();
    let tmp_backup_dir = tempdir::TempDir::new("backup").unwrap();
    let target_file = get_tmp_path(&tmp_dir, "restore_backup");
    let backup_file = get_tmp_path(&tmp_backup_dir, &target_file[1..]);
    let _ = save_file(&backup_file, "#ff00ff");
    let _ = restore_backup(tmp_backup_dir.path().to_str().unwrap());
    let output = load_file(&target_file);

    assert_eq!(output.unwrap(), "#ff00ff");
}
