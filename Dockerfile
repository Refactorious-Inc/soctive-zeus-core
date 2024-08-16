FROM rust:1.59 as builder

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# This is a dummy build to get the dependencies cached
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy in the real source code
COPY . .

# Build for release
RUN cargo build --release

# Final stage
FROM debian:buster-slim

WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/soctive-zeus-core .

# Install SSL certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the startup command
CMD ["./soctive-zeus-core"]