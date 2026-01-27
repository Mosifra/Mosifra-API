FROM rust:1.90

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Rocket.toml ./

RUN cargo install --path . && rm -rf target Cargo.toml Cargo.lock src 

CMD ["Mosifra-API"]

ARG USERNAME=user-name-goes-here
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid 1000 nonroot \
    && useradd --uid 1000 --gid 1000 -m nonroot
    
USER nonroot

EXPOSE 8000
