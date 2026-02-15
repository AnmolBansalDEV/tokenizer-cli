use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Tokenizer Cli based on Byte pair encoding"
)]
pub struct Cli {
    #[command(subcommand)]
    pub method: Method,
}

#[derive(Subcommand)]
pub enum Method {
    Tokenize {
        #[arg(short, long)]
        input_path: PathBuf,
        #[arg(short = 'n', long)]
        max_dictionary_size: u128,
        #[arg(short, long, default_value = "./dictionary.txt")]
        output_path: Option<PathBuf>,
    },
    Encode {
        #[arg(short, long)]
        input_path: PathBuf,
        #[arg(short, long)]
        dictionary_path: PathBuf,
        #[arg(short, long, default_value = "./encoded.txt")]
        output_path: Option<PathBuf>,
    },
    Decode {
        #[arg(short, long)]
        input_path: PathBuf,
        #[arg(short, long)]
        dictionary_path: PathBuf,
        #[arg(short, long, default_value = "./decoded.txt")]
        output_path: Option<PathBuf>,
    },
}
