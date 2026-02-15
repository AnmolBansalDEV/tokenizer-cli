use clap::Parser;

use crate::{
    cli::{Cli, Method},
    utils::{decode, encode, tokenize},
};

mod cli;
mod utils;

fn main() {
    let cli = Cli::parse();

    match cli.method {
        Method::Tokenize {
            input_path,
            max_dictionary_size,
            output_path,
        } => {
            tokenize(input_path, max_dictionary_size, output_path);
        }
        Method::Encode {
            input_path,
            dictionary_path,
            output_path,
        } => {
            encode(input_path, dictionary_path, output_path);
        }
        Method::Decode {
            input_path,
            dictionary_path,
            output_path,
        } => {
            decode(input_path, dictionary_path, output_path);
        }
    }
}
