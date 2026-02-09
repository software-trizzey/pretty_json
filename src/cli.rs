use std::{fmt, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Format {
    Pretty,
    Compact,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Format::Pretty => "pretty",
            Format::Compact => "compact",
        })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub filepath: PathBuf,

    #[arg(short, long, default_value_t = Format::Pretty)]
    pub format: Format,

    #[arg(short, long)]
    pub output_path: Option<PathBuf>,
}
