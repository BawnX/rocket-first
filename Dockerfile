FROM rust:1.54.0 as build

WORKDIR /usr/src/rest-api
COPY . .

RUN rustup default nightly

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/rocket /build-out/

# Ubuntu 18.04
FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/rocket /

EXPOSE 8080

CMD ./rocket
