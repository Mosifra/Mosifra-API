FROM alpine:latest as security_provider
RUN addgroup -S nonroot \
    && adduser -S nonroot -G nonroot

FROM rust:1.90

COPY --from=security_provider /etc/passwd /etc/passwd

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Rocket.toml ./

RUN cargo install --path . && rm -rf target Cargo.toml Cargo.lock src 

USER nonroot

CMD ["Mosifra-API"]

EXPOSE 8000
