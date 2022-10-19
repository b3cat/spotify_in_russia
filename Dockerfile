FROM rust:1.64.0 as build

RUN cargo new app

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=build /app/target/release/spotify_in_russia /app/config.toml ./
CMD ["/app/spotify_in_russia", "--config", "/app/config.toml"]
