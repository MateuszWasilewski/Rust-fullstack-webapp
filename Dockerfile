FROM rust:1.61.0 as build

RUN apt update && apt upgrade -y
RUN apt-get install \
    -y openssl libssl-dev 
    #gcc-aarch64-linux-gnu libc6-dev-arm64-cross

RUN rustup target add wasm32-unknown-unknown 

FROM build as build_backend

RUN cargo install sqlx-cli

WORKDIR /usr/build
COPY . .

ENV SQLX_OFFLINE=true

#--target=aarch64-unknown-linux-gnu
RUN \
    --mount=type=cache,target=/usr/build/target \
    cargo build --release && \
    cp target/release/backend backend-app

FROM build as build_frontend

RUN cargo install trunk
RUN cargo install --locked wasm-bindgen-cli
#RUN cargo install wasm-opt

WORKDIR /usr/build
COPY . .

# build frontend
RUN \
    --mount=type=cache,target=/usr/build/target \
    cd frontend && \
    trunk build --release

#RUN wasm-opt -Oz -o dist/frontend-9878cf296e84afe_bg.wasm dist/frontend-9878cf296e84afe_bg.wasm

#--platform=linux/aarch64
FROM ubuntu:22.04 as release

RUN apt update && apt upgrade -y
RUN apt-get install -y libfontconfig1-dev

WORKDIR /usr/bin
COPY --from=build_backend /usr/build/backend-app .
COPY --from=build_frontend /usr/build/frontend/dist frontend/dist

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT [ "backend-app" ]