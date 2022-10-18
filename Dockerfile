FROM rust:1.64.0

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/spotify_in_russia"]