# Multistage docker image building
# build -> prod

FROM rust:1.64-slim-bullseye as build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:11.5-slim as prod
RUN apt-get update  && \
    apt-get install -y ca-certificates httpie --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
WORKDIR /app
COPY --from=build /build/target/release/deficientebot_telegram .
RUN adduser --disabled-password appuser
USER appuser
ENV RUST_LOG=info
ENTRYPOINT [ "./deficientebot_telegram" ]