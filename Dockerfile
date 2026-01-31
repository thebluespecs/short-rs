# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies for Alpine
RUN apk add --no-cache musl-dev postgresql-dev

WORKDIR /app

# Copy manifests first (for better layer caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy src to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (cached unless Cargo.toml changes)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src

# Touch main.rs to force rebuild of our code (not dependencies)
RUN touch src/main.rs

# Build the actual application
RUN cargo build --release

# Runtime stage - minimal image
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libpq ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/short-it .

# Expose port
EXPOSE 8000

# Run the binary
CMD ["./short-it"]
