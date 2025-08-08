# Multi-stage build for batless
# Produces a minimal Docker image (~8MB) for containerized usage

# Stage 1: Build
FROM rust:1.80-alpine AS builder

# Install build dependencies including oniguruma for syntect
RUN apk add --no-cache musl-dev gcc oniguruma-dev pkgconfig

# Set environment variables to use system oniguruma
ENV RUSTONIG_SYSTEM_LIBONIG=1

# Create app directory
WORKDIR /app

# Copy all source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches
COPY README.md ./

# Build the application using native target (musl is already the default in Alpine)
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.18 AS runtime

# Install CA certificates for HTTPS
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1001 -S batless && \
    adduser -S -D -H -u 1001 -G batless batless

# Copy the binary from builder stage
COPY --from=builder /app/target/release/batless /usr/local/bin/batless

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
