# Multistage docker image building
# build -> prod

# Build stage
FROM lukemathwalker/cargo-chef:0.1.59-rust-slim-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install pkg-config libssl-dev -y
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin telegrambot_ukranian

# Runtime stage
FROM alpine:3.17.3 AS runtime
WORKDIR /app
RUN apk --no-cache --no-progress --update add ca-certificates
COPY --from=builder --chown=nobody:nogroup /app/target/release/telegrambot_ukranian /usr/local/bin/app
EXPOSE 8080
USER nobody:nogroup
ENV RUST_LOG=info
ENTRYPOINT ["/usr/local/bin/app"]
