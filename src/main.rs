use clap::{Parser, Subcommand};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Tokenizer Cli based on Byte pair encoding"
)]
struct Cli {
    // #[arg(short, long)]
    // input_path: PathBuf,
    // #[arg(short = 'n', long)]
    // max_dictionary_size: Option<u128>,
    #[command(subcommand)]
    method: Method,
    // #[arg(short, long)]
    // dictionary_path: PathBuf,
    // #[arg(short, long, default_value = "./out.txt")]
    // output_path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Method {
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

fn get_pair_stats(char_codes: &Vec<usize>) -> Vec<(usize, (usize, usize))> {
    let mut stats: HashMap<String, usize> = HashMap::new();

    for window in char_codes.windows(2) {
        let first = window[0];
        let second = window[1];
        let pair = format!("{:?}-{:?}", first, second);

        *stats.entry(pair).or_insert(1) += 1;
    }

    let mut final_value: Vec<(usize, (usize, usize))> = Vec::new();

    for (key, count) in stats {
        let codes: Result<Vec<usize>, _> = key
            .split("-")
            .map(|x| x.parse().map_err(|e| (key.clone(), e)))
            .collect();
        match codes {
            Ok(nums) => final_value.push((count, (nums[0], nums[1]))),
            Err((k, e)) => eprintln!("failed to parse the key: {}, err: {}", k, e),
        }
    }

    final_value.sort_by(|a, b| b.0.cmp(&a.0));
    final_value
}

fn token_swap(
    tokens: &Vec<usize>,
    merge_pair: (usize, usize),
    new_token_id: usize, // has to > 255 (utf-8)
) -> Vec<usize> {
    let mut new_tokens: Vec<Option<usize>> = tokens.iter().map(|&v| Some(v.into())).collect();
    for i in 0..new_tokens.len() - 1 {
        if new_tokens[i] == Some(merge_pair.0.into())
            && new_tokens[i + 1] == Some(merge_pair.1.into())
        {
            new_tokens[i] = Some(new_token_id);
            new_tokens[i + 1] = None;
        }
    }

    let new_tokens: Vec<usize> = new_tokens.into_iter().filter_map(|v| v).collect();
    new_tokens
}

fn tokenize(input_path: PathBuf, max_dictionary_size: u128, output_path: Option<PathBuf>) {
    let text = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "unable to read the file at path: {:?}, error occurred: {}",
                input_path, e
            );
            return;
        }
    };

    let char_codes = text.as_bytes();

    let mut tokens_for_operation: Vec<usize> =
        char_codes.into_iter().map(|&c| c as usize).collect();

    let mut merge_dict_ordered: HashMap<String, usize> = HashMap::new();

    let iterations_allowed = max_dictionary_size - 256;
    for i in 0..iterations_allowed {
        let sorted_pair_stats = get_pair_stats(&tokens_for_operation);

        let new_token_id = 256 + i; // has to be > utf-8 size
        tokens_for_operation = token_swap(
            &tokens_for_operation,
            sorted_pair_stats[0].1,
            new_token_id as usize,
        );

        merge_dict_ordered.insert(
            format!("{}-{}", sorted_pair_stats[0].1.0, sorted_pair_stats[0].1.1),
            new_token_id as usize,
        );
    }

    let mut contents = String::new();
    for (k, v) in merge_dict_ordered.iter() {
        contents.push_str(format!("{}: {}\n", k, v).as_str());
    }
    match fs::write(output_path.clone().unwrap_or_default(), contents) {
        Ok(_v) => println!("successfully created dictionary at: {:?}", output_path),
        Err(e) => eprintln!("failed to write dictionary: {}", e),
    }
}

fn encode(input_path: PathBuf, dictionary_path: PathBuf, output_path: Option<PathBuf>) {
    let dictionary = match fs::read_to_string(dictionary_path.clone()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "failed to read dictionary at: {:?}, err: {}",
                dictionary_path, e
            );
            return;
        }
    };

    let dictionary: HashMap<String, usize> = dictionary
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let key = String::from(parts.next().expect("invalid dictionary provided").trim());
            let value = parts
                .next()
                .expect("invalid dictionary provided")
                .trim()
                .parse::<usize>()
                .expect("failed parsing, invalid dictionary provided");
            (key, value)
        })
        .collect();

    let text = match fs::read_to_string(input_path.clone()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "failed to read the input file at: {:?}, err: {}",
                input_path, e
            );
            return;
        }
    };

    let text = Vec::from(text.as_bytes());

    let mut tokens: Vec<Option<usize>> = text.iter().map(|&v| Some(v as usize)).collect();

    for item in dictionary {
        let mut i = 0;
        while i < tokens.len() - 1 {
            let a = match tokens[i] {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let b = match tokens[i + 1] {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let sub_str = format!("{}-{}", a, b);

            if item.0 == sub_str {
                tokens[i] = Some(item.1);
                tokens[i + 1] = None;
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    let tokens: Vec<usize> = tokens.into_iter().flatten().collect();

    let content = tokens
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    match fs::write(output_path.clone().unwrap_or_default(), content) {
        Ok(_v) => println!("successfully created output at: {:?}", output_path),
        Err(e) => eprintln!(
            "failed to create the output at: {:?}, err: {}",
            output_path, e
        ),
    }
}

fn decode(input_path: PathBuf, dictionary_path: PathBuf, output_path: Option<PathBuf>) {
    let dictionary = match fs::read_to_string(dictionary_path.clone()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "failed to read the dictionary at: {:?}, err: {}",
                dictionary_path, e
            );
            return;
        }
    };

    let reverse_dictionary: HashMap<usize, (usize, usize)> = dictionary
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let pair = String::from(parts.next().expect("invaild dictionary provided").trim());
            let items = pair
                .split("-")
                .into_iter()
                .map(|k| {
                    k.trim()
                        .parse::<usize>()
                        .expect("parsing err, invalid dictionary provided")
                })
                .collect::<Vec<usize>>();
            let token = parts
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .expect("parsing err, invalid dictionary provided");
            (token, (items[0], items[1]))
        })
        .collect();

    let mut tokens = match fs::read_to_string(input_path.clone()) {
        Ok(content) => content
            .split(", ")
            .map(|v| v.parse::<usize>().expect("invalid tokens provided"))
            .collect::<Vec<usize>>(),
        Err(e) => {
            eprintln!("failed to read tokens at: {:?}, err: {}", input_path, e);
            return;
        }
    };

    let mut i = 0;
    while i < tokens.len() - 1 {
        if let Some(&new_token_pair) = reverse_dictionary.get(&tokens[i]) {
            tokens.splice(i..i + 1, [new_token_pair.0, new_token_pair.1]);
        } else {
            i += 1;
        }
    }

    let content = match String::from_utf8(
        tokens
            .iter()
            .filter_map(|&v| u8::try_from(v).ok())
            .collect::<Vec<u8>>(),
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to decode the tokens, err: {}", e);
            return;
        }
    };

    match fs::write(output_path.clone().unwrap_or_default(), content) {
        Ok(_v) => println!(
            "successfully created the decoded string at: {:?}",
            output_path
        ),
        Err(e) => eprintln!(
            "failed to create the decoded string at: {:?}, err: {}",
            output_path, e
        ),
    }
}
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
