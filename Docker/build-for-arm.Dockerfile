# https://github.com/rust-lang/docker-rust/blob/1d112bc218d6b7a5479a05fa652130d8e086564f/1.31.1/stretch/Dockerfile
# From: https://docs.docker.com/samples/library/rust/#start-a-rust-instance-running-your-app
FROM rust:latest

RUN dpkg --add-architecture armhf \
    && apt-get update \
    && apt-get install -yqq \
    build-essential \
    gcc-arm-linux-gnueabihf \
    libsqlite3-dev:armhf

# Add RPi (ARM) as target
RUN rustup target add armv7-unknown-linux-gnueabihf

# Print stacktrace on error
# https://github.com/rust-lang/rust/pull/38165
RUN export RUST_BACKTRACE=1

# Specify working directory.
# You should ind mount crate root on host to this directory
WORKDIR /app

# Using
# RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc"
# since specifying linker in .cargo/config proved challenging
ENTRYPOINT RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc" \
    cargo build --target=armv7-unknown-linux-gnueabihf
