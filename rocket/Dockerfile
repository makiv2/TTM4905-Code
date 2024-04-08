FROM rust:latest
LABEL authors="maki"

ENV ROCKET_ENV = production

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release

CMD ["cargo", "run"]
