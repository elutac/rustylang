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
./rustylang [OPTIONS] --tolang <target language> <text|--paste>
```

### Options

To view all options execute:

```bash
./rustylang --help
```

### Examples

Translate text directly with specified target language:

```bash
./rustylang -t DE "Hello, world!"
```

Translate text from the clipboard and copy the result back to the clipboard:

```bash
./rustylang -t DE -c -p
```

Enable verbose output:

```bash
./rustylang -t DE -v "How are you?"
```

## Environment Variables

This tool requires you to set the following environment variable:

- `DEEPL_AUTH_KEY`: Your DeepL API authentication key. You can get this key by [signing up on DeepL](https://www.deepl.com/de/pro-api?cta=header-pro-api).

You can set this environment variable by creating a .env file in the project directory with the following content:

```bash
DEEPL_AUTH_KEY=your_auth_key_here
```

If you add the tool to your path, you can also export the environment variable in your shell.

## Supported Languages

To view all supported languages execute:

```bash
./rustylang --help
```

## Contributing
Contributions are welcome! Please feel free to submit a pull request.
