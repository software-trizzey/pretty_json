use std::{fs, path::Path};

use serde_json::Value;

fn read_file(filepath: &Path) -> std::io::Result<String> {
    fs::read_to_string(filepath)
}

fn parse_contents(contents: String) -> serde_json::Result<Value> {
    let v: Value = serde_json::from_str(&contents)?;
    Ok(v)
}

pub fn prettify_json(filepath: &Path) -> std::result::Result<String, String> {
    let file = read_file(filepath).unwrap();
    let v: Value = parse_contents(file).unwrap();
    let prettified = format!("{:#}", v);

    Ok(prettified)
}

pub fn write_file(filepath: &Path, json: String) {
    fs::write(filepath, json).unwrap()
}
