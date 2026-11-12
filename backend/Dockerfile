# Build Stage
FROM rust:1.94-alpine AS builder

RUN apk add --no-cache ca-certificates musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

# Run Stage
FROM alpine:3.20
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 3001
CMD ["backend"]
