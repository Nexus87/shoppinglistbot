FROM rustlang/rust as builder

WORKDIR /app

COPY Cargo.lock ./
COPY Cargo.toml ./
COPY todoist/Cargo.toml ./todoist/Cargo.toml
COPY bot/Cargo.toml ./bot/Cargo.toml

RUN cargo fetch --locked

COPY ./ ./
RUN cargo build --release -Z unstable-options  --out-dir release

FROM debian:jessie-slim
COPY --from=builder /app/target/release/* .

CMD /shoppinglist