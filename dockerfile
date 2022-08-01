FROM rust:1.40 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

FROM alpine:3.14

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app

CMD ["myapp"]