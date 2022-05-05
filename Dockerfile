FROM rust:1.60 AS deps

WORKDIR /app
RUN apt-get update && apt-get install -y libclang-dev clang

# dependencies
RUN mkdir -p src/bin
RUN touch src/lib.rs
RUN echo "fn main(){}" > src/bin/main.rs
COPY ./Cargo.toml .

RUN mkdir ./subcrate
COPY ./subcrate ./subcrate
RUN touch subcrate/lib.rs

COPY ./Cargo.lock .
RUN cargo build --release

# binary
COPY ./src ./src
RUN touch src/lib.rs

RUN cargo build --release

# production
FROM alpine:latest AS prod
COPY --from=deps /app/target/release/build-test /usr/bin/build-test

CMD [ "build-test" ]