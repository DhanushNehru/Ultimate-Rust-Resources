# ROT13 + Base64 Decoder 🔐

## Usage

```shell
# Decrypt an encoded string
$ cargo run -- M3I6r2IbMzq9
```

This tool reverses a custom encoding scheme applied as:

```
plaintext → ROT13 → base64 → ROT13
```

The program takes an encrypted string and outputs the original plaintext by:
```
ROT13 → base64 decode → ROT13 → plaintext
```

This is a great beginner project to understand text transformation, base64 encoding and command-line argument parsing in Rust.