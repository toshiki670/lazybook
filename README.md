# lazybook

A private information management tool with a Text-based User Interface (TUI).

## Overview

lazybook is a personal tool for managing private information through an efficient, keyboard-driven terminal interface. Built with Rust and following Domain-Driven Design principles, it provides secure storage and quick access to personal data.

## Technical Stack

- **Language**: Rust (stable)
- **Interface**: TUI (Text-based User Interface)
- **Architecture**: Domain-Driven Design (DDD)

## Development

This project follows the SpecKit development workflow. See `.specify/memory/constitution.md` for detailed development principles and constraints.

### Local install (use the app on your machine)

Install via a private Homebrew tap:

```bash
brew tap toshiki670/lazybook   # once
brew install lazybook
```

### Development in a container (Cursor CLI Agent)

Development uses **standard Docker** (no mise): Rust official image + zsh.

1. **Prerequisites**: Docker (or Docker Desktop) and Dev Containers support (Cursor includes this).
2. Open this repository in Cursor.
3. Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`) → **Dev Containers: Reopen in Container**.
4. Wait for the container to build (first time) and for `postStartCommand` (`cargo fetch`) to finish.
5. The workspace runs inside the container; Cursor Agent uses the container’s terminal and tools.

The container is defined in `.devcontainer/` (Rust official image + zsh). All edits and commands from Agent apply inside the container.

To get a shell inside the container without Cursor (from the repo root):

```bash
docker compose build
docker compose run --rm dev
```

### Build from source on host (optional)

If you want to build on the host instead of using the tap or container:

```bash
# Install Rust (e.g. via mise: https://mise.jdx.dev/)
mise install   # if using .mise.toml
cargo build --release
cargo test
```

## Security

This project handles private personal information. All data is encrypted at rest, and security best practices are followed throughout the codebase.

## License

See [LICENSE](LICENSE) for details.