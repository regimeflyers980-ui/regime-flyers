# Build Stage
FROM rust:1.85-alpine AS builder

# Permanent Fix: Combine apk commands into one layer to prevent SSL timeout issues
RUN apk add --no-cache ca-certificates musl-dev

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Run Stage
FROM alpine:3.20

# Install necessary runtime certificates
RUN apk add --no-cache ca-certificates

# Copy the compiled binary
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 3001
CMD ["backend"]
