FROM rust:1.67 AS build
WORKDIR /usr/src/not_in_the_bible
COPY Cargo.toml Cargo.lock diesel.toml ./
COPY ./src/ ./src/
COPY ./locales/ ./locales/
COPY ./migrations/ ./migrations/
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/ /usr/local/bin/
COPY example/ /config/
VOLUME [ "/config" ]
CMD [ "not_in_the_bible", "-c", "/config/config.yaml" ]
