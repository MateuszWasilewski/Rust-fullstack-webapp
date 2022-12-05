FROM rust:1.61.0 as build

RUN rustup target add wasm32-unknown-unknown 
RUN cargo install trunk

WORKDIR /usr/build
COPY . .
# build backend
RUN \
    --mount=type=cache,target=/usr/build/target \
    cargo build --release && \
    cp target/debug/backend backend-app

# build frontend
RUN \
    --mount=type=cache,target=/usr/build/target \
    cd frontend --release && \
    trunk build

FROM ubuntu:20.04 as release
WORKDIR /usr/bin
COPY --from=build /usr/build/backend-app .
COPY --from=build /usr/build/frontend/dist frontend/dist

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "backend-app" ]