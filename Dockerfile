FROM rust:1.53 as builder

WORKDIR /usr/src/elections

ENV SQLX_OFFLINE="true"

COPY Cargo.toml Cargo.lock sqlx-data.json ./
COPY migrations ./migrations
COPY src ./src

RUN cargo build --release

RUN cargo install --version=0.5.2 sqlx-cli --no-default-features --features postgres

FROM debian:buster-slim

WORKDIR /usr/src/elections

# curl is used for docker-compose health checks
RUN apt-get update && \
    apt-get install curl ca-certificates -y --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY ./migrations ./migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/bin/sqlx
COPY --from=builder /usr/src/elections/target/release/jkzomaar-elections /usr/bin/jkzomaar-elections
COPY ./entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

EXPOSE 8080/tcp

ENTRYPOINT [ "/entrypoint.sh" ]

CMD [ "jkzomaar-elections" ]