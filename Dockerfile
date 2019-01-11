FROM rustlang/rust:nightly as builder

WORKDIR /app
RUN mkdir -p bot/src && echo "fn main() {}" >> bot/src/main.rs
RUN mkdir -p todoist/src && echo "fn main() {}" >> todoist/src/main.rs
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY todoist/Cargo.toml ./todoist/Cargo.toml
COPY bot/Cargo.toml ./bot/Cargo.toml

RUN cargo build --release
# RUN cargo build
RUN rm bot/src/main.rs
RUN rm todoist/src/main.rs

COPY ./ ./

RUN cargo build --release -Z unstable-options  --out-dir release --all
# RUN cargo build -Z unstable-options  --out-dir release --all

FROM debian:stable-slim

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/release/* /

ENTRYPOINT [ "/shoppinglist" ]