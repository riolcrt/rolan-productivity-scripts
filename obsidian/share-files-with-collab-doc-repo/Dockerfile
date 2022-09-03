FROM rust:1.61.0-slim-buster AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/ /usr/local/bin/
CMD ["app"]
