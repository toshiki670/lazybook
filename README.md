# lazybook

A private information management tool with a Text-based User Interface (TUI).

## Overview

lazybook is a personal tool for managing private information through an efficient, keyboard-driven terminal interface. Built with Rust and following Domain-Driven Design principles, it provides secure storage and quick access to personal data.

## Technical Stack

- **Language**: Rust (stable)
- **Interface**: TUI (Text-based User Interface)
- **Architecture**: Domain-Driven Design (DDD)
- **Tool Management**: mise

## Development

This project follows the SpecKit development workflow. See `.specify/memory/constitution.md` for detailed development principles and constraints.

### Prerequisites

Install mise for tool management:

```bash
# Follow mise installation instructions for your platform
# https://mise.jdx.dev/getting-started.html
```

### Setup

```bash
# Install project tools via mise
mise install

# Build the project
cargo build

# Run tests
cargo test
```

## Security

This project handles private personal information. All data is encrypted at rest, and security best practices are followed throughout the codebase.

## License

See [LICENSE](LICENSE) for details.