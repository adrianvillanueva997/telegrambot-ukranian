FROM rust:1.72.1-slim-bullseye AS build
WORKDIR /build
RUN apt-get update && \
    apt-get install -y apt-utils pkg-config libssl-dev --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
COPY . .
RUN cargo build --release

FROM ubuntu:22.04 AS prod
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN echo "deb http://security.ubuntu.com/ubuntu focal-security main" | tee /etc/apt/sources.list.d/focal-security.list
RUN apt-get update && \
    apt-get install -y apt-utils ca-certificates pkg-config libssl-dev libssl1.1 --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
WORKDIR /app
COPY --from=build /build/target/release/telegrambot_ukranian .
RUN adduser --disabled-password appuser
USER appuser
ENV RUST_LOG=info
EXPOSE 80
ENTRYPOINT [ "./telegrambot_ukranian" ]
