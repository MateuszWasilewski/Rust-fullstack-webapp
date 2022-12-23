FROM rust:1.61.0 as build

RUN rustup target add wasm32-unknown-unknown 
RUN cargo install trunk

RUN apt update && apt upgrade -y

RUN rustup target add aarch64-unknown-linux-gnu

RUN apt-get install -y openssl libssl-dev && \
    apt install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross

WORKDIR /usr/build
COPY . .
ENV SQLX_OFFLINE=true

# build backend
RUN \
    --mount=type=cache,target=/usr/build/target \
    cargo build --release --target=aarch64-unknown-linux-gnu && \
    cp target/aarch64-unknown-linux-gnu/release/backend backend-app

# build frontend
RUN \
    --mount=type=cache,target=/usr/build/target \
    cd frontend && \
    trunk build --release

FROM --platform=linux/aarch64 ubuntu:22.04 as release

WORKDIR /usr/bin
COPY --from=build /usr/build/backend-app .
COPY --from=build /usr/build/frontend/dist frontend/dist

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT [ "backend-app" ]