FROM alpine AS rust
RUN apk update
RUN apk add --no-cache rustup build-base
RUN rustup-init -y
RUN mkdir -p /build
WORKDIR /build
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src
RUN source /root/.cargo/env && cargo build --release

FROM scratch as runtime
COPY --from=rust /build/target/release/mazer /mazer
EXPOSE 8080
CMD ["/mazer"]
