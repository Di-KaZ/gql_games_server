FROM rust:1.74 as builder
WORKDIR /usr/src/game_gql_server
EXPOSE 8080
COPY . .
RUN cargo install --path .
CMD ["game_gql_server"]

# unable to get it working because of statically shared lib
#FROM debian:bullseye-slim
#EXPOSE 8080
#RUN apt-get update && apt-get install -y libsqlite3-dev libc2  && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/local/cargo/bin/game_gql_server /usr/local/bin/game_gql_server
#CMD ["game_gql_server"]
