# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.94

FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /usr/src/telers
COPY Cargo.toml ./
COPY telers telers
COPY telers-codegen telers-codegen
COPY telers-dialog telers-dialog
COPY telers-macros telers-macros
COPY examples/dialogs_mega examples/dialogs_mega
RUN cargo build --release --package dialogs_mega

FROM alpine:3.22 AS final
RUN apk add --no-cache ca-certificates && rm -rf /var/cache/apk/*
RUN adduser -D -u 10001 app
COPY --from=build /usr/src/telers/target/release/dialogs_mega /usr/local/bin/dialogs_mega
USER app
ENV RUST_BACKTRACE=1
ENTRYPOINT ["/usr/local/bin/dialogs_mega"]
