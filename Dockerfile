FROM rust:alpine AS builder

RUN apk add build-base cmake linux-headers

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY libs ./libs
COPY apps ./apps

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=global-registry \
    --mount=type=cache,target=/usr/local/cargo/git,id=global-git \
    RUST_TARGET="$(uname -m)-unknown-linux-musl" && \
    RUSTFLAGS="-C target-feature=+crt-static" \
    cargo build --release --target "${RUST_TARGET}" && \
    cp "target/${RUST_TARGET}/release/dat-cms" /app/dat-cms-bin

FROM scratch

WORKDIR /app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/dat-cms-bin /dat-cms

ENV PORT=80

EXPOSE 80
ENTRYPOINT ["/dat-cms"]
