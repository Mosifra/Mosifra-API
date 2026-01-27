FROM rust:1.90.0

ARG USER=default
ENV HOME=/home/$USER

RUN adduser --disabled-password --gecos "" $USER

RUN mkdir -p /usr/src/myapp && chown -R $USER /usr/src/myapp

USER $USER
WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Rocket.toml ./


RUN cargo install --path . 

EXPOSE 8000

CMD ["Mosifra-API"]
