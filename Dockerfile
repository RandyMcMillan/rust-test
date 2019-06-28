FROM rust as build

COPY ./ ./

RUN rustup target install x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-musl/release/rust-test /build-out/

# Ubuntu 18.04
FROM scratch

#ENV DEBIAN_FRONTEND=noninteractive
#RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/rust-test /rust

ENTRYPOINT ["/rust"]