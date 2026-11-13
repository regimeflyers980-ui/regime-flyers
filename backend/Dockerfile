# Build Stage
FROM rust:1.94-alpine AS builder
RUN apk add --no-cache ca-certificates musl-dev
WORKDIR /app

# 1. Copy ONLY the manifests first
COPY Cargo.toml Cargo.lock ./

# 2. Create a dummy main.rs and build ONLY the dependencies
# This layer will be cached by Docker forever
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# 3. Copy the REAL source code and migrations
COPY src ./src
COPY migrations ./migrations

# 4. Build the real app (this will take seconds because deps are cached)
# We touch main.rs to force cargo to rebuild the final binary
RUN touch src/main.rs && cargo build --release

# Run Stage
FROM alpine:3.20
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 3001
CMD ["backend"]
