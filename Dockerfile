FROM rust:1.93.0-bookworm AS build
WORKDIR /build
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY Cargo.toml Cargo.lock* ./
COPY src src
RUN cargo build --release --locked

FROM debian:bookworm-slim AS prod
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* && \
    useradd -r -u 1000 appuser
WORKDIR /app
COPY --from=build /build/target/release/telegrambot_ukranian ./bot
RUN chown appuser:appuser /app
USER appuser
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["./bot"]
