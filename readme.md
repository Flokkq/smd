# Sweet Markdown

## Introduction

Sweet Markdown is a versatile command-line tool designed to convert Markdown files into GitHub-flavored PDF, HTML, or image formats while retaining the distinctive GitHub CSS styling. It's perfect for developers, content creators, and anyone looking to share their Markdown documents in various formats while maintaining the familiar GitHub aesthetic.

## Requirements

Before proceeding, ensure that the following are installed on your system:
- [Node.js and npm](https://nodejs.org/)

### Cross-Platform Installation Notes

- **Windows**: No additional configuration required.
- **Linux & MacOS**: You may need to change npm rights from root to user.

#### Changing npm Rights (Linux & MacOS)

1. Confirm your username: `whoami`
2. Change ownership of npm directories: `sudo chown -R $(whoami) /usr/local/lib/node_modules`

## Installation

Clone and install Sweet Markdown:

```bash
# Clone the repository
git clone https://github.com/Flokkq/SweetMarkdown --depth 1

# Change directory
cd SweetMarkdown

# Run the installation script
./scripts/install.sh
```

After the installation, the `smd` command should be available in your terminal.

## Usage

### Basic Command

```bash
# Convert a Markdown file to the desired format
smd --input <path/to/file.md> --output <format> --specifc <specifc-format>
```

### Help Command

```bash
# Display help information
smd --help
```

### Markdown flavour Command
```bash
# Set desired Markdown flavour
smd --flavour
```

### Supported Output Formats

- **PDF**: Portable Document Format
- **HTML**: HyperText Markup Language
- **IMG**: Image file (JPG, PNG, WEBP)

## Features

- **GitHub CSS Styling**: Maintains the GitHub-flavored CSS styling in all output formats.
- **Multiple Output Formats**: Convert Markdown files to PDF, HTML, or various image formats.
- **Command-Line Interface**: Easy-to-use CLI for quick conversions.

### Examples

```bash
# Convert Markdown to pdf
smd --input readme.md --output html

# Convert Markdown to png
smd --input readne.md --output img --specific png
```

## License

This tool is released under the [MIT License](LICENSE).
