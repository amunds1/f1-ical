FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/f1-ical .
COPY templates templates
COPY static static

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENV PORT 8000

CMD ["./f1-ical"]