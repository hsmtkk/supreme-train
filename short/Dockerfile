FROM golang:1.17 AS builder

WORKDIR /opt

COPY . .

WORKDIR /opt/short

RUN go build

FROM gcr.io/distroless/cc-debian11 AS runtime

COPY --from=builder /opt/short/short /usr/local/bin/short

ENTRYPOINT ["/usr/local/bin/short"]
