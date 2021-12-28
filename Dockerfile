FROM rust:1.57 as builder
WORKDIR /usr/src/fish_api
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /fish_api
COPY --from=builder /usr/local/cargo/bin/fish_api ./
COPY ./dataset.csv .
CMD ["./fish_api"]
