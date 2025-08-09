# Base image: Rust + Cargo + standard build tools
FROM rust:1.89-slim-bookworm

# Install system dependencies your Rust crates may need
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for VS Code
ARG USERNAME=vscode
RUN useradd -ms /bin/bash $USERNAME \
    && echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER $USERNAME

# Set the working directory inside the container
WORKDIR /workspace
