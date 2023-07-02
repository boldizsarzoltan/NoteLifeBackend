FROM rust:latest

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . .

ENV ROCKET_ADDRESS=0.0.0.0

RUN cargo build --release

CMD bash -c "diesel setup && ./target/release/backend"