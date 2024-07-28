FROM --platform=linux/amd64 rust:1.80.0-bookworm AS builder-x86_64-unknown-linux-gnu

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo test --target=x86_64-unknown-linux-gnu
RUN cargo build --release --target=x86_64-unknown-linux-gnu

FROM --platform=linux/arm64 rust:1.80.0-bookworm AS builder-aarch64-unknown-linux-gnu
# OS dependencies
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo test --target=aarch64-unknown-linux-gnu
RUN cargo build --release --target=aarch64-unknown-linux-gnu

FROM crazymax/osxcross:14.5-r0-debian AS osxcross
FROM rust:1.80.0-bookworm AS apple-base

RUN apt-get update && apt-get install -y \
    libxdo-dev \
    clang \
    lld \
    musl-dev \
    && rm -rf /var/lib/apt/lists/*
ENV PATH="/osxcross/bin:$PATH"
ENV LD_LIBRARY_PATH="/osxcross/lib:$LD_LIBRARY_PATH"

WORKDIR /app
COPY . .

FROM apple-base AS builder-x86_64-apple-darwin

RUN rustup target add x86_64-apple-darwin

RUN --mount=type=bind,from=osxcross,source=/osxcross,target=/osxcross \
  cargo build --release --target=x86_64-apple-darwin

FROM apple-base AS builder-aarch64-apple-darwin

RUN rustup target add aarch64-apple-darwin

RUN --mount=type=bind,from=osxcross,source=/osxcross,target=/osxcross \
  cargo build --release --target=aarch64-apple-darwin

FROM busybox
COPY --from=builder-x86_64-unknown-linux-gnu /app/target/x86_64-unknown-linux-gnu/release/amm-rust /amm-rust-x86_64-unknown-linux-gnu
COPY --from=builder-aarch64-unknown-linux-gnu /app/target/aarch64-unknown-linux-gnu/release/amm-rust /amm-rust-aarch64-unknown-linux-gnu
COPY --from=builder-x86_64-apple-darwin /app/target/x86_64-apple-darwin/release/amm-rust /amm-rust-x86_64-apple-darwin
COPY --from=builder-aarch64-apple-darwin /app/target/aarch64-apple-darwin/release/amm-rust /amm-rust-aarch64-apple-darwin
WORKDIR /dist
CMD cp /amm-rust-* /dist/
