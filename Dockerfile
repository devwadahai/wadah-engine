# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release -p wadah-cli

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libgcc

# Create non-root user
RUN addgroup -g 1000 wadah && \
    adduser -D -u 1000 -G wadah wadah

# Copy binary from builder
COPY --from=builder /build/target/release/wadah /usr/local/bin/wadah

# Set permissions
RUN chmod +x /usr/local/bin/wadah

# Switch to non-root user
USER wadah
WORKDIR /workspace

# Set entrypoint
ENTRYPOINT ["wadah"]
CMD ["--help"]

# Labels
LABEL org.opencontainers.image.title="Wadah"
LABEL org.opencontainers.image.description="AI Agent Runtime - Contain Intelligence"
LABEL org.opencontainers.image.source="https://github.com/devwadahai/wadah-engine"
LABEL org.opencontainers.image.vendor="Wadah.ai"
LABEL org.opencontainers.image.licenses="Apache-2.0"

