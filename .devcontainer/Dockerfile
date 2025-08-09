FROM rust:1.89-slim

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev git curl \
    && rm -rf /var/lib/apt/lists/*

ARG USERNAME=vscode
RUN useradd -ms /bin/bash $USERNAME \
    && echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER $USERNAME
WORKDIR /workspace
