FROM rust:bullseye as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build

FROM debian:bullseye
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/debug/backend /usr/local/bin/app
COPY --from=builder /usr/src/app/Rocket.toml .
EXPOSE 8000

ENV AWS_REGION=eu-central-1
ENV AWS_ENDPOINT_URL=http://192.168.0.10:9000
ENV AWS_ACCESS_KEY_ID=cloud
ENV AWS_SECRET_ACCESS_KEY=9NJfTCQ4V7Y3LNRQ
ENV AWS_BUCKET=cloud
ENV SURREAL_NAMESPACE=Cloud
ENV SURREAL_DATABASE=Cloud
ENV SURREAL_USERNAME=bot
ENV SURREAL_PASSWORD=bot123
CMD ["app"]
