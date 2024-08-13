# Password Generator

This is a simple password generator written in Rust. It allows you to generate passwords with various options such as length, inclusion of digits, special characters and uppercase letters.

## Usage

To run the password generator, use the following command:

```sh
cargo run -- [options]
```

The following options are available:

- `-l`, `--length`: The length of the password (default: 16)
- `-d`, `--digits`: Include digits in the password
- `-s`, `--special`: Include special characters in the password
- `-u`, `--uppercase`: Include uppercase letters in the password
- `-h`, `--help`: Print help message"

## Examples
To generate a password of length 16 with digits and special characters, you can use the following command:
```sh
cargo run -- -l 16 -d -s
```

## Testing

To run the tests, use the following command:

```sh
cargo test
```