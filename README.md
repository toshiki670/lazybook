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

### Development in a container (Cursor Agent)

To run the development environment and **Cursor Agent** inside a container:

1. **Prerequisites**: Docker (or Docker Desktop) and the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension (Cursor includes this).
2. Open this repository in Cursor.
3. Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`) → **Dev Containers: Reopen in Container**.
4. Wait for the container to build (first time) and for `postCreateCommand` (`mise install`, `cargo fetch`) to finish.
5. The workspace runs inside the container; Cursor Agent uses the container’s terminal and tools (`cargo`, `rustc`, etc.) for development.

The container is defined in `.devcontainer/` (Rust + mise, matching `.mise.toml`). All edits and commands from Agent apply inside the container.

## Security

This project handles private personal information. All data is encrypted at rest, and security best practices are followed throughout the codebase.

## License

See [LICENSE](LICENSE) for details.