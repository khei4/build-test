FROM rust:1.60 AS builder

WORKDIR /app

COPY Cargo.toml .

RUN apt-get update && apt-get install -y libclang-dev clang


RUN mkdir -p src/bin
RUN touch src/lib.rs
RUN echo "fn main(){}" > src/bin/main.rs

RUN mkdir ./subcrate
COPY ./subcrate ./subcrate
RUN touch subcrate/lib.rs

COPY ./Cargo.lock .
RUN cargo build --release

COPY ./src ./src
RUN touch src/lib.rs

RUN cargo build --release

FROM alpine:latest AS prod
COPY --from=builder /app/target/release/build-test /usr/bin/build-test

CMD [ "build-test" ]