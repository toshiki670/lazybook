# Dev container for lazybook: Rust + mise
# Cursor Agent runs inside this container and uses this environment for development.
FROM mcr.microsoft.com/devcontainers/base:bookworm

# Install build essentials (for rusqlite native deps if needed)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install mise for vscode user (postCreateCommand runs as vscode)
USER vscode
RUN curl -sSf https://mise.run | sh \
    && echo 'eval "$(/home/vscode/.local/bin/mise activate bash)"' >> ~/.bashrc
ENV PATH="/home/vscode/.local/bin:${PATH}"

WORKDIR /workspaces/lazybook
