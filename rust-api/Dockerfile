# set up rust runtime enviroment
FROM rust:1.54.0-slim-buster

# set up working directory
WORKDIR /usr/src/app

# copy source code
COPY . .

# build the project
RUN cargo build --release

# set up runtime enviroment
FROM debian:buster-slim

# run aptget update
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=0 /usr/src/app/target/release/rust-api /usr/local/bin/rust-api
CMD ["rust-api"]