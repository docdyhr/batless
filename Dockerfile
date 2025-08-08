# Multi-stage build for batless
# Produces a minimal Docker image (~8MB) for containerized usage

# Stage 1: Build
FROM rust:1.80-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Add musl target for cross-compilation
RUN rustup target add x86_64-unknown-linux-musl

# Create app directory
WORKDIR /app

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy directories and files to build dependencies
RUN mkdir src benches && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    echo "fn main() {}" > benches/performance.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src benches

# Copy source code
COPY src ./src
COPY benches ./benches
COPY README.md ./

# Build the actual application
RUN touch src/main.rs  # Force rebuild
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM alpine:3.18 AS runtime

# Install CA certificates for HTTPS
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1001 -S batless && \
    adduser -S -D -H -u 1001 -G batless batless

# Copy the binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/batless /usr/local/bin/batless

# Set ownership and permissions
RUN chown batless:batless /usr/local/bin/batless && \
    chmod +x /usr/local/bin/batless

# Switch to non-root user
USER batless

# Set working directory
WORKDIR /workspace

# Default entrypoint
ENTRYPOINT ["batless"]

# Default command (help)
CMD ["--help"]

# Metadata labels
LABEL org.opencontainers.image.title="batless" \
      org.opencontainers.image.description="Non-blocking code viewer for AI and automation" \
      org.opencontainers.image.source="https://github.com/docdyhr/batless" \
      org.opencontainers.image.documentation="https://github.com/docdyhr/batless/blob/main/README.md" \
      org.opencontainers.image.licenses="MIT"
