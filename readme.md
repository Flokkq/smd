# Sweet Markdown

## Introduction

Sweet Markdown is a versatile command-line tool designed to convert Markdown files into GitHub-flavored PDF, HTML, or image formats while retaining the distinctive GitHub CSS styling. It's perfect for developers, content creators, and anyone looking to share their Markdown documents in various formats while maintaining the familiar GitHub aesthetic.

## Requirements

### Windows

Before proceeding, ensure that [npm](https://www.npmjs.com/) is installed on your system.

### Linux & MacOS

Before proceeding, ensure that [npm](https://www.npmjs.com/) is installed on your system.

#### Changing npm Rights from Root to User

To change the ownership of npm directories to the current user, follow these steps:
1. First, confirm your username by running the following command in your terminal:

```bash
whoami
```

2. Once you have your username, use it to change the ownership of the npm directories. Replace `<username>` with your actual username in the command below:

```bash
sudo chown -R <username>: /usr/local/lib/node_modules
```

This command changes the ownership of the npm directory to your user, ensuring that you can run npm commands without needing root permissions.

## Installation

Sweet Markdown can be either installed via the binaries on the [GitHub](https://github.com/Flokkq/SweetMarkdown) or with a corresponding package manager.

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
