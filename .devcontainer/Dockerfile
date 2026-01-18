FROM mcr.microsoft.com/devcontainers/rust:1

# Install common build dependencies used by many Rust crates
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Add rustfmt and clippy so they are available in the container
RUN rustup component add rustfmt clippy || true

CMD ["sleep", "infinity"]
