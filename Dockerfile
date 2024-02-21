# syntax=docker/dockerfile:1
FROM rust:latest as builder
ARG RUST_CHANNEL=stable
ENV RUST_CHANNEL=$RUST_CHANNEL
ARG RUST_APP_NAME
ENV RUST_APP_NAME=$RUST_APP_NAME
WORKDIR /app/
RUN rustup default $RUST_CHANNEL
COPY . .
RUN cargo build --release --bin $RUST_APP_NAME
FROM gcr.io/distroless/cc-debian12:nonroot
ARG RUST_APP_NAME
ENV RUST_APP_NAME=$RUST_APP_NAME
WORKDIR /bin/
COPY --from=builder /app/target/release/$RUST_APP_NAME app
ENTRYPOINT [ "/bin/app" ]
CMD [ "--help" ]
