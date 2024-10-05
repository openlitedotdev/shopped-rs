FROM rust:1.80 as build

RUN USER=root cargo new --bin shopped-rs
WORKDIR /shopped-rs

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx

RUN rm ./target/release/deps/shopped_rs*
RUN cargo build --release

COPY --from=build /shopped-rs/target/release/shopped_rs .

ENV RUST_BACKTRACE=1

ENTRYPOINT ["./shopped_rs"]
