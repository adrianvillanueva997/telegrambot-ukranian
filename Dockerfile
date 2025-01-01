# Build stage
FROM rust:1.82.0-slim-bookworm AS build
WORKDIR /build

# Install only necessary build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy only files needed for build first
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim AS prod
WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -r -s /bin/false appuser

COPY --from=build /build/target/release/telegrambot_ukranian .

USER appuser
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["./telegrambot_ukranian"]
