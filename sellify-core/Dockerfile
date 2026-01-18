# Multi-stage build for Sellify Core

# Stage 1: Builder
FROM rust:1.77-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --bin sellify-server --features http-server --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/sellify-server /usr/local/bin/sellify-server

# Create data directory
RUN mkdir -p /data && chmod 777 /data

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run as non-root user
RUN useradd -m -u 1000 sellify && chown -R sellify:sellify /data
USER sellify

# Start server
CMD ["sellify-server"]
