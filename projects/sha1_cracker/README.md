# SHA-1 Cracker

This is a command-line tool written in Rust to crack SHA-1 hashes using a wordlist.

## Description

The program takes a wordlist file and a SHA-1 hash as input. It attempts to find the original string that corresponds to the given SHA-1 hash by comparing each word in the wordlist to the hash.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Installation

1. Clone the repository:
```
git clone https://github.com/DhanushNehru/sha1_cracker.git
```

2. Navigate to the project directory:

```
cd sha1_cracker
```

### Running the Code

```
cargo run -- wordlist.txt 7c222fb2927d828af22f592134e8932480637c0d
```

You will get tbe output for commonly found password from the word list

```
Password found: 12345678
```