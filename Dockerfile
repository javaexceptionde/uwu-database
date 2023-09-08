FROM rust:1.61.0 as builder
LABEL authors="jonathan" name="uwu-database" version="0.1.0"
EXPOSE 8080

ENTRYPOINT ["top", "-b"]
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
EXPOSE 8080
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/uwu-database /usr/local/bin/uwu-database
CMD ["uwu-database"]