FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1001 appuser
WORKDIR /app

# The binary is named after the package (finnlang), not server
COPY --from=builder /app/target/release/finnlang ./server
RUN chmod +x ./server
RUN chown appuser:appuser /app/server

USER appuser
EXPOSE 3000
CMD ["./server"]