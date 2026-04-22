# syntax=docker/dockerfile:1

# ── Stage 1: Build ────────────────────────────────────────────────────────────
FROM rust:1-trixie AS builder

# Install cargo-binstall for fast binary installs
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin

# System deps: clang for linking, Node.js for Tailwind CSS / daisyui
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends clang curl ca-certificates \
    && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && apt-get autoremove -y && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# cargo-leptos via binstall (pre-built binary, no compile wait)
RUN cargo binstall cargo-leptos -y

# WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Install npm deps (Tailwind CSS + daisyui)
RUN npm install --prefix leptos_store

# Release build — outputs:
#   target/server/release/leptos_store  (server binary)
#   target/site/                        (WASM, CSS, JS assets)
RUN cargo leptos build --release --project leptos_store

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────
FROM debian:trixie-slim AS runtime

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Server binary
COPY --from=builder /app/target/release/leptos_store ./server

# Static assets (WASM, CSS, JS)
COPY --from=builder /app/target/site ./site

# get_configuration(Some("leptos_store/Cargo.toml")) reads these at startup
COPY --from=builder /app/Cargo.toml ./Cargo.toml
COPY --from=builder /app/leptos_store/Cargo.toml ./leptos_store/Cargo.toml

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 3000

CMD ["/app/server"]
