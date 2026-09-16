# syntax=docker/dockerfile:1

FROM rust:1.98-bookworm AS builder

# xmp_toolkit compiles the bundled Adobe XMP Toolkit C/C++ sources.
RUN apt-get update \
    && apt-get install --yes --no-install-recommends cmake g++ \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY openapi_generated ./openapi_generated
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --locked --release \
    && cp target/release/backend /backend

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder --chown=nonroot:nonroot /backend /backend

ENV GALERIE_LISTEN_ADDRESS=0.0.0.0 \
    GALERIE_LISTEN_PORT=3000 \
    GALERIE_CONTENT_ACCESS__MODE=Nop

EXPOSE 3000

ENTRYPOINT ["/backend"]
