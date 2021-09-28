FROM rust:1.55 as build

RUN USER=root cargo new --bin api-rust
WORKDIR /api-rust

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/api_rust*
RUN cargo build --release

FROM rust:1.55-slim-buster

COPY --from=build /api-rust/target/release/api-rust .

CMD [ "./api-rust" ]