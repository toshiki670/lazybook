# Dev container for lazybook: standard Docker (Rust official image)
# For Cursor CLI Agent. No mise; local install via private tap.
FROM rust:1-bookworm

# Install zsh and set as default shell
RUN apt-get update && apt-get install -y --no-install-recommends zsh \
    && usermod -s /usr/bin/zsh root \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspaces/lazybook
