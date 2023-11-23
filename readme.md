# README.md for <Sweet Mardkown>

## Introduction

Sweet Markdown is a versatile command-line tool designed to convert GitHub-flavored Markdown files into PDF, HTML, or image formats while retaining the distinctive GitHub CSS styling. It's perfect for developers, content creators, and anyone looking to share their Markdown documents in various formats while maintaining the familiar GitHub aesthetic.

## Installation
Sweet Markdown can be either installed via the binaries on the [github](https://github.com/Flokkq/SweetMarkdown) or with a corresponding package manager.

### Linux
```bash
apt-get install smd
```

### MacOS
```bash
brew install smd
```

### Windows
```bash
choco install smd
```


## Usage

### Basic Command

```bash
# Convert a Markdown file to the desired format
smd <input_file.md> <output_format>
```

### Help Command

```bash
# Display help information
smd --help
```

### Supported Output Formats

- **PDF**: Portable Document Format
- **HTML**: HyperText Markup Language
- **IMG**: Image file (JPG, PNG, etc.)

## Features

- **GitHub CSS Styling**: Maintains the GitHub-flavored CSS styling in all output formats.
- **Multiple Output Formats**: Convert Markdown files to PDF, HTML, or various image formats.
- **Command-Line Interface**: Easy-to-use CLI for quick conversions.
- **Batch Conversion**: Support for converting multiple files at once.

## Examples

```bash
# Convert Markdown to PDF
smd example.md pdf

# Convert Markdown to HTML
smd example.md html

# Convert Markdown to an Image
smd example.md img jpg
```

## License

This tool is released under the [MIT License](LICENSE).