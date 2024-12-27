# Use the official Rust image as the base
FROM rust:1.83 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a minimal image to run the application
FROM debian:bookworm-slim

# Install required dependencies for Rust executables
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/tokio-shared-state-monitor /app/

# Set the default command to run the application
CMD ["./tokio-shared-state-monitor"]