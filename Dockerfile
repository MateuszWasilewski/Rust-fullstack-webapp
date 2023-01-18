FROM rust:1.61.0 as build

RUN apt update && apt upgrade -y
RUN apt-get install -y openssl libssl-dev && \
    apt install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross

RUN rustup target add wasm32-unknown-unknown 
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo install trunk
RUN cargo install wasm-opt

WORKDIR /usr/build
COPY . .

FROM build as build_backend

ENV SQLX_OFFLINE=true
RUN \
    --mount=type=cache,target=/usr/build/target \
    cargo build --release --target=aarch64-unknown-linux-gnu && \
    cp target/aarch64-unknown-linux-gnu/release/backend backend-app

FROM build as build_frontend

# build frontend
RUN \
    --mount=type=cache,target=/usr/build/target \
    cd frontend && \
    trunk build --release

#RUN wasm-opt -Oz -o dist/frontend-9878cf296e84afe_bg.wasm dist/frontend-9878cf296e84afe_bg.wasm

FROM --platform=linux/aarch64 ubuntu:22.04 as release

WORKDIR /usr/bin
COPY --from=build_backend /usr/build/backend-app .
COPY --from=build_frontend /usr/build/frontend/dist frontend/dist

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT [ "backend-app" ]