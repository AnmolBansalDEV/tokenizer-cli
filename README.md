# Tokenizer CLI (Byte Pair Encoding)

This is a command-line tool for tokenizing, encoding, and decoding text using Byte Pair Encoding (BPE), implemented in Rust.
## Features

- **Tokenize**: Build a BPE dictionary from input text.
- **Encode**: Encode text into tokens using a BPE dictionary.
- **Decode**: Decode tokens back to text using a BPE dictionary.

## Installation
Build the project with Cargo:

```sh
cargo build --release
```

## Usage
Run the CLI with one of the subcommands:

### Tokenize
Generate a BPE dictionary from an input file:

```sh
./target/release/tokenizer tokenize --input-path <INPUT_FILE> --max-dictionary-size <SIZE> [--output-path <DICT_FILE>]
```

- `--input-path` (required): Path to the input text file.
- `--max-dictionary-size` (required): Maximum dictionary size (must be > 256).
- `--output-path` (optional): Output path for the dictionary (default: `./dictionary.txt`).

### Encode
Encode text using a BPE dictionary:

```sh
./target/release/tokenizer encode --input-path <INPUT_FILE> --dictionary-path <DICT_FILE> [--output-path <ENCODED_FILE>]
```

- `--input-path` (required): Path to the input text file.
- `--dictionary-path` (required): Path to the BPE dictionary file.
- `--output-path` (optional): Output path for the encoded tokens (default: `./encoded.txt`).

### Decode
Decode tokens back to text:

```sh
./target/release/tokenizer decode --input-path <ENCODED_FILE> --dictionary-path <DICT_FILE> [--output-path <DECODED_FILE>]
```

- `--input-path` (required): Path to the encoded tokens file.
- `--dictionary-path` (required): Path to the BPE dictionary file.
- `--output-path` (optional): Output path for the decoded text (default: `./decoded.txt`).

## Example
```sh
# Step 1: Tokenize
./target/release/tokenizer tokenize --input-path input.txt --max-dictionary-size 300 --output-path dictionary.txt

# Step 2: Encode
./target/release/tokenizer encode --input-path input.txt --dictionary-path dictionary.txt --output-path encoded.txt

# Step 3: Decode
./target/release/tokenizer decode --input-path encoded.txt --dictionary-path dictionary.txt --output-path decoded.txt
```

## License
MIT
