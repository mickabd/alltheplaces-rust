# Use Alpine-based Rust image for the builder stage
FROM rust:1.85-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    curl \
    build-base

# Create new cargo project
RUN cargo new --bin alltheplaces
WORKDIR /alltheplaces

# Build for release
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    # Use the RUSTFLAGS to statically link OpenSSL
    RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

# Use a clean Alpine image for the final stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    openssl \
    libgcc

# Create non-root user
ARG UID=10001
RUN adduser \
    -D \
    -g "" \
    -h "/nonexistent" \
    -s "/sbin/nologin" \
    -u "${UID}" \
    appuser
USER appuser

WORKDIR /app

# Copy the build artifact from the builder stage
COPY --from=builder /alltheplaces/target/release/alltheplaces .

# Set the startup command
CMD ["./alltheplaces"]
