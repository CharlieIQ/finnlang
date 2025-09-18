FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# List everything that was built for debugging
RUN echo "=== BUILT BINARIES ===" && ls -la target/release/ && echo "===================="

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary
COPY --from=builder /app/target/release/server .

# Verify it was copied
RUN echo "=== COPIED FILES ===" && ls -la && echo "===================="
RUN echo "=== SERVER FILE ===" && ls -la server && file server && echo "===================="

EXPOSE 3000
CMD ["./server"]