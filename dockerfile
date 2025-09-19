FROM rust:1.85

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo install --path .

CMD ["mosifra-api"]