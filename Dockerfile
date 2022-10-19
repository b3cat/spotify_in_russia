FROM rust:1.64.0 as build
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
RUN apt-get update \
	&& apt-get install -y ca-certificates openssl \
	# cleanup
	&& rm -rf /var/lib/apt/lists/*
COPY --from=build /target/release/spotify_in_russia /usr/bin/
COPY --from=build /config.toml /etc/spotify_in_russia/
ENTRYPOINT ["spotify_in_russia"]
CMD ["--config", "/etc/spotify_in_russia/config.toml"]
