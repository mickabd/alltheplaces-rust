# Use Alpine-based Rust image for the builder stage
FROM rust:1.85-alpine AS builder

ENV RUST_LOG=info \
    RUST_BACKTRACE=1

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

COPY backend/Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build dependencies to reuse cached layers
# Use the RUSTFLAGS to statically link OpenSSL
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

# Build for release
RUN --mount=type=bind,source=backend/src,target=src \
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
COPY --from=builder /alltheplaces/target/release/backend .

EXPOSE 8080

# Set the startup entry point to the built binary
CMD ["./backend"]
