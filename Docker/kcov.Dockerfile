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
    && make install DESTDIR=/opt/kcov \
    && cd ../.. \
    && rm -rf kcov-master

# https://stackoverflow.com/questions/27093612
ENV PATH="/opt/kcov/usr/local/bin/:${PATH}"

# https://github.com/kennytm/cargo-kcov
RUN cargo install cargo-kcov

# Specify working directory.
# You should bind mount crate root on host to this directory
WORKDIR /app

# https://github.com/rust-lang/cargo/issues/6100
# 1) Copy crate root contents to /app
# 2) Get coverage
# 3) Copy coverage back to /shared/target, and thus to host if volume was mounted
ENTRYPOINT cp -R /shared/. /app/ \
    && cargo kcov --all \ 
    && cp -R target/cov /shared/target/