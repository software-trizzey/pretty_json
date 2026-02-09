use std::ops::Deref;
use std::{fs, path::Path};

use clap::Parser;
use serde_json::Value;

pub mod cli;

fn read_file(filepath: &Path) -> std::io::Result<String> {
    fs::read_to_string(filepath)
}

fn parse_contents(contents: String) -> serde_json::Result<Value> {
    let v: Value = serde_json::from_str(&contents)?;
    Ok(v)
}

pub fn prettify_json(file: &Path, format: cli::Format) -> std::result::Result<String, String> {
    let file = read_file(file).unwrap();
    let v: Value = parse_contents(file).unwrap();
    let prettified = match format {
        cli::Format::Pretty => format!("{:#}", v),
        cli::Format::Compact => format!("{}", v.to_string()),
    };

    Ok(prettified)
}

pub fn write_file(filepath: &Path, json: String) {
    fs::write(filepath, json).unwrap()
}

pub fn run() -> Result<(), String> {
    let args = cli::Args::parse();

    if cfg!(debug_assertions) {
        println!("{:?}", args);
    }

    let p_json = prettify_json(args.filepath.deref(), args.format);
    match args.output_path {
        Some(out) => {
            if cfg!(debug_assertions) {
                println!("adding results to {:?}", out);
            }
            write_file(&out, p_json.unwrap());
        }
        _ => {
            if cfg!(debug_assertions) {
                println!("Overwriting input file {:?}", args.filepath);
            }
            write_file(&args.filepath, p_json.unwrap());
        }
    }
    Ok(())
}
