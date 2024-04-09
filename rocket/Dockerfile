FROM rust:latest
LABEL authors="maki"

ENV ROCKET_ENV = production

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release
RUN chmod +x ./run.sh
RUN cargo install diesel_cli --no-default-features --features postgres

CMD ["./run.sh"]