# rustylang : Translation CLI Tool using DeepL and Rust

A command line interface (CLI) tool to translate text using DeepL API, written in Rust. This tool allows you to translate text from one language to another and view the response directly in your terminal.

## Table of Contents

  - [Installation](#installation)
  - [Usage](#usage)
    - [Options](#options)
    - [Example](#example)
  - [Environment Variables](#environment-variables)
  - [Supported Languages](#supported-languages)
  - [Contributing](#contributing)

## Installation

To use this tool, you need to have Rust installed on your system. If you haven't installed Rust, you can install it from [the official website](https://www.rust-lang.org/learn/get-started).

Once Rust is installed, you can compile the project by navigating to the project directory and running:

```bash
cargo build --release
```

This command will create an executable file in the `target/release` directory.

## Usage

Run the RustyLang tool using the following syntax:

```bash
./rustylang -t <target_language> "Text to translate"
```

### Options

To view all options execute:

```bash
./rustylang --help
```

### Example

```bash
./rustylang -t DE "Hello, world!"
```

This command will translate "Hello, world!" to German and print the translated text.

## Environment Variables

This tool requires you to set the following environment variable:

- `DEEPL_AUTH_KEY`: Your DeepL API authentication key. You can get this key by [signing up on DeepL](https://www.deepl.com/de/pro-api?cta=header-pro-api).

You can set this environment variable by creating a .env file in the project directory with the following content:

```bash
DEEPL_AUTH_KEY=your_auth_key_here
```

## Supported Languages

To view all supported languages execute:

```bash
./rustylang --help
```

## Contributing
Contributions are welcome! Please feel free to submit a pull request.