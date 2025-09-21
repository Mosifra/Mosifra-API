FROM rust:1.85

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .env ./
COPY Rocket.toml ./

RUN cargo install --path .
RUN rm -rf target Cargo.toml Cargo.lock src

CMD ["Mosifra-API"]

EXPOSE 8000