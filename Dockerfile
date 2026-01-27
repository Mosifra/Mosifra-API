FROM rust:1.90.0-alpine3.22

ARG USER=default
ENV HOME /home/$USER

RUN adduser -D $USER

USER $USER
WORKDIR $HOME

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Rocket.toml ./

RUN cargo install --path . && rm -rf target Cargo.toml Cargo.lock src 

USER nonroot

CMD ["Mosifra-API"]

EXPOSE 8000
