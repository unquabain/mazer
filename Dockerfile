FROM rust:1.75 AS build

WORKDIR /usr/src/mazer
COPY . .
RUN cargo install --profile release --path .
EXPOSE 8080
CMD ["mazer"]
