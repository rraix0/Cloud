FROM rust:bullseye as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/app
COPY --from=builder /usr/src/app/Rocket.toml .
EXPOSE 8000

CMD ["app"]
