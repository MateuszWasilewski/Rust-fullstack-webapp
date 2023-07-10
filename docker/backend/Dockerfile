FROM rust:1.70.0 as build_base

RUN apt update && apt upgrade -y
RUN apt-get install \
    -y openssl libssl-dev 

RUN rustup target add wasm32-unknown-unknown 

RUN cargo install \
    sqlx-cli \
    trunk \
    wasm-bindgen-cli \
    cargo-chef

WORKDIR /usr/build
ENV SQLX_OFFLINE=true

from build_base as backend_planner 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM build_base as build_backend
COPY --from=backend_planner /usr/build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

RUN \
    cargo build --release && \
    cp target/release/backend backend-app

FROM build_base as build_frontend
COPY . .

# build frontend
RUN \
    cd frontend && \
    trunk build --release

FROM ubuntu:22.04 as release

RUN apt update && apt upgrade -y
RUN apt-get install -y libfontconfig1-dev

WORKDIR /usr/bin
COPY --from=build_backend /usr/build/backend-app .
COPY --from=build_frontend /usr/build/frontend/dist frontend/dist

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT [ "backend-app" ]
