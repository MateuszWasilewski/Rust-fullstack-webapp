FROM rust:1 as build

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown 

WORKDIR /usr/build
COPY . .
WORKDIR /usr/build/frontend
RUN trunk build
WORKDIR /usr/build
RUN cargo build

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "target/debug/backend" ]