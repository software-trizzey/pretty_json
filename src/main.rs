use std::{ops::Deref, path::PathBuf};

use clap::Parser;
use pretty_json::{prettify_json, write_file};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filepath: PathBuf,

    #[arg(short, long)]
    output_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let p_json = prettify_json(args.filepath.deref());
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
}
