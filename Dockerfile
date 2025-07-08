FROM rust:slim-bookworm AS builder
WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get upgrade && \
    apt-get install -y --no-install-recommends \
    build-essential npm

# Install pnpm
RUN npm install -g pnpm

# Copy Rust toolchain config
COPY rust-toolchain.toml .

# Update Rust toolchain and install cargo-leptos
RUN rustup update && \
    cargo install --locked --version=0.2.33 cargo-leptos

# Copy package manifests and install Node dependencies with pnpm
COPY package.json pnpm-lock.yaml ./
# Use pnpm cache for faster builds (requires BuildKit)
RUN --mount=type=cache,target=/root/.pnpm-store pnpm install

# Copy the rest of your source code
COPY . .

# Build your Rust/Leptos project
RUN cargo leptos build --release -vv

# Production runner stage
FROM debian:bookworm-slim AS runner
WORKDIR /var/www/app

RUN apt-get update && apt-get upgrade

# Set up non-root user
RUN groupadd -r server && \
    useradd -r -g server -s /usr/sbin/nologin -c "Docker user" docker && \
    chown -R docker:server /var/www/app

# Copy compiled binary and site files from builder stage
COPY --chown=docker:server --from=builder /build/target/release/portfolio ./portfolio
COPY --chown=docker:server --from=builder /build/target/site ./site

USER docker

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="/var/www/app/site"

EXPOSE 3000

CMD ["./portfolio"]
