# https://github.com/rust-lang/docker-rust/blob/1d112bc218d6b7a5479a05fa652130d8e086564f/1.31.1/stretch/Dockerfile
# From: https://docs.docker.com/samples/library/rust/#start-a-rust-instance-running-your-app
FROM rust:latest

RUN apt-get update \
    && apt-get install -yqq \
    libcurl4-openssl-dev \
    libelf-dev \
    libdw-dev \
    cmake \
    gcc \
    binutils-dev \
    libiberty-dev \
    zlib1g-dev

# Print stacktrace on error
# https://github.com/rust-lang/rust/pull/38165
RUN export RUST_BACKTRACE=1

# Specify working directory.
# You should ind mount crate root on host to this directory
WORKDIR /app

# Using
# RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc"
# since specifying linker in .cargo/config proved challenging
ENTRYPOINT cargo test

# https://sunjay.dev/2016/07/25/rust-code-coverage
# https://github.com/codecov/example-rust/blob/master/.travis.yml
# WGET kcov & build
RUN wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz \
    && tar xzf master.tar.gz \
    && cd kcov-master \
    && mkdir build \
    && cd build \
    && cmake .. \
    && make \ 
    && make install DESTDIR=../../kcov-build \
    && cd ../.. \
    && rm -rf kcov-master \
    && for file in target/debug/raspberry_web-*; do [ -x "${file}" ] || continue; mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done

