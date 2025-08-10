# Build stage
FROM rust:1.89-slim-bookworm AS builder
# Install CA certificates for HTTPS calls
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release


# Runtime stage
FROM debian:bookworm-slim AS runtime
# Install CA certificates for HTTPS calls
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sandybox1 /usr/local/bin/sandybox1
EXPOSE 5100
CMD ["sandybox1"]
