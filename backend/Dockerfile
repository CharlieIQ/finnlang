FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the server binary explicitly
RUN cargo build --release --bin server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary to /app/server
COPY --from=builder /app/target/release/server /app/server

# Make it executable
RUN chmod +x /app/server

# Test that the binary exists and is executable
RUN ls -la /app/server
RUN file /app/server

EXPOSE 3000

# Use absolute path to be 100% sure
CMD ["/app/server"]