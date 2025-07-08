# ---------- Build Stage ----------
FROM rust:slim-bookworm AS builder
WORKDIR /build

# Install build tools and Node.js (for pnpm)
RUN apt-get update && \
    apt-get install -y --no-install-recommends build-essential npm

# Install pnpm globally
RUN npm install -g pnpm

# Copy only package files first (better Docker cache)
COPY package.json pnpm-lock.yaml ./
RUN pnpm install

# Copy the rest of your files
COPY . .

# Install cargo-leptos
COPY rust-toolchain.toml .
RUN rustup update && \
    cargo install --locked --version=0.2.33 cargo-leptos

# Build the app (release mode)
RUN cargo leptos build --release -vv

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim AS runner
WORKDIR /var/www/app

RUN apt-get update && apt-get upgrade -y

# Create a non-root user
RUN groupadd -r server && \
    useradd -r -g server -s /usr/sbin/nologin -c "Docker user" docker && \
    mkdir -p /var/www/app && \
    chown -R docker:server /var/www/app

# Copy built files and set correct owner
COPY --from=builder /build/target/release/portfolio ./portfolio
COPY --from=builder /build/target/site ./site
RUN chown -R docker:server /var/www/app

USER docker

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:${PORT}"

ENV LEPTOS_SITE_ROOT="/var/www/app/site"

EXPOSE 3000

CMD ["./portfolio"]
