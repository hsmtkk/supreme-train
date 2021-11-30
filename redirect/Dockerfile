FROM golang:1.17 AS builder

WORKDIR /opt

COPY . .

WORKDIR /opt/redirect

RUN go build

FROM gcr.io/distroless/cc-debian11 AS runtime

COPY --from=builder /opt/redirect/redirect /usr/local/bin/redirect

ENTRYPOINT ["/usr/local/bin/redirect"]
