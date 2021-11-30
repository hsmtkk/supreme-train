FROM rust:1.56 AS builder

WORKDIR /opt

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11 AS runtime

COPY --from=builder /opt/target/release/redirect /usr/local/bin/redirect
COPY --from=builder /opt/target/release/short /usr/local/bin/short

EXPOSE 80
