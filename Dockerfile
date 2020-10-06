FROM rust:1.42 as builder
WORKDIR /usr/src/mama-server
COPY . .

RUN echo $(ls -1 /usr/src/mama-server)
RUN echo $(ls -1 /usr/src/mama-server/src)

RUN apt-get update && \
    apt-get install -y libpq-dev libmariadb-dev-compat libmariadb-dev sqlite3

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libpq-dev libmariadb-dev-compat libmariadb-dev sqlite3

COPY --from=builder /usr/local/cargo/bin/mama-server /usr/local/bin/mama-server

RUN echo $(ls -1 /usr/local/bin/mama-server)

CMD ["mama-server"]






