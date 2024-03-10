# Sweet Markdown

## Table of Contents

- [Introduction](#introduction)
- [Requirements](#requirements)
  - [Cross-Platform Installation Notes](#cross-platform-installation-notes)
- [Installation](#installation)
  - [Installation Script](#installation-script)
  - [Manual Installation](#manual-installation)
- [Usage](#usage)
  - [Rendering Markdown](#rendering-markdown)
    - [Supported Output Formats](#supported-output-formats)
    - [Rendering Markdown with Sweet Markdown](#rendering-markdown-with-sweet-markdown)
  - [Flavors (Color Themes)](#flavors-color-themes)
    - [Changing Flavor](#changing-flavor)
    - [Creating Custom Flavors](#creating-custom-flavors)
    - [Editing Custom Flavors](#editing-custom-flavors)
    - [Updating Custom Flavors](#updating-custom-flavors)
- [License](#license)

## Introduction

Sweet Markdown is a command-line utility designed for converting Markdown files into various formats including PDF, HTML, or images, all while applying the familiar GitHub CSS styling. This tool is ideal for developers, content creators, and anyone interested in distributing their Markdown content in multiple formats without losing the GitHub look and feel.

## Requirements

Ensure the following prerequisites are met before installation:

- [Rust and Cargo](https://www.rust-lang.org/) (required for the installation script)

## Installation

Sweet Markdown can be installed via:

- Cloning the repository and executing the installation script.
- Downloading the latest release from [GitHub](https://github.com/Flokkq/smd/release).

### Installation Script

```bash
# Clone the repository
git clone https://github.com/Flokkq/SweetMarkdown/release --depth 1

# Navigate to the directory
cd smd

# Execute the installation script
./scripts/install
```

### Manual Installation

After downloading the latest release, execute the following commands based on your operating system:

- **Linux & MacOS**: `sudo mv path/to/smd /usr/local/bin/smd`
- **Windows**: `copy path\to\smd.exe C:\Windows\System32\smd.exe`

The `smd` command will now be accessible from your terminal.

## Usage

### Rendering Markdown

#### Supported Output Formats

- **PDF**: Portable Document Format
- **HTML**: HyperText Markup Language
- **Image**: JPG, PNG, WEBP formats

#### Rendering Markdown with Sweet Markdown

```bash
# Convert a Markdown file to the specified format
smd --input <path/to/file.md> --output <format> --specific <specific-format>

# Example:
smd --input README.md --output pdf

# Or:
smd --input README.md --output img --specific jpg
```

### Flavors (Color Themes)

Sweet Markdown allows for easy customization of markdown rendering through the use of flavors, including the option to create custom ones.

#### Changing Flavor

Change the global flavor setting to apply to all markdown renderings:

```bash
smd --flavor --set

Select a flavor:
        1 > light
        2 > dark
        3 > auto
flavor:
```

Alternatively, specify a flavor for a single rendering:

```bash
smd --input README.md --output pdf --flavor light
```

#### Creating Custom Flavors

Generate your own custom flavors with `--flavor --create [flavor-name]`, starting from a template:

```yaml
# Flavor name
color-scheme: dark
syntax:
  comment: "#8b949e"
  constant: "#79c0ff"
  entity: "#d2a8ff"
[...]
```

After customization, add your flavor to the available options with `smd --flavor --add /path/to/file`. The `.yaml` file will be integrated into Sweet Markdown's configuration directory.

#### Editing Custom Flavors

To modify a custom flavor:

```bash
smd --flavor --edit [flavor-name]

# Or choose from a list of available flavours

smd --flavour --edit

Select a flavor:
        1 > light
        2 > dark
        3 > auto
flavor:
```

This command opens the `.yaml` configuration file for editing.

#### Updating Custom Flavors

After editing, apply the changes with:

```bash
smd --flavor --update /path/to/your/file
```

Note: Sweet Markdown will incorporate the updated `.yaml` file into its configuration directory.

## License

Sweet Markdown is available under the [MIT License](LICENSE)
