FROM --platform=linux/amd64 rust:latest AS builder
WORKDIR /usr/src

RUN apt-get update && apt-get install -y musl-tools

# Create blank project
RUN USER=root cargo new cairohttp

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/cairohttp/

# Set the working directory
WORKDIR /usr/src/cairohttp

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/cairohttp/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/cairohttp/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM --platform=linux/amd64 alpine:3.19@sha256:c5b1261d6d3e43071626931fc004f70149baeba2c8ec672bd4f27761f8e1ad6b

WORKDIR /usr/local/bin

EXPOSE 3000

COPY --from=builder /usr/src/cairohttp/target/x86_64-unknown-linux-musl/release/cairo-http ./

CMD ["cairo-http"]