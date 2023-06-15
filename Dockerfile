# Multistage docker image building
# build -> prod

FROM rust:1.70.0-slim-bullseye as build
WORKDIR /build
RUN apt-get update && \
    apt-get install -y apt-utils pkg-config libssl-dev --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
COPY . .
RUN cargo build --release

FROM debian:12.0-slim as prod
RUN apt-get update  && \
    apt-get install -y ca-certificates httpie --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
WORKDIR /app
COPY --from=build /build/target/release/telegrambot_ukranian .
RUN adduser --disabled-password appuser
USER appuser
ENV RUST_LOG=info
ENTRYPOINT [ "./telegrambot_ukranian" ]
