FROM rust:1.90

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Rocket.toml ./

RUN cargo install --path . && rm -rf target Cargo.toml Cargo.lock src 

CMD ["Mosifra-API"]

USER nonroot

EXPOSE 8000
