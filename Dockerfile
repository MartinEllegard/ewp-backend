# Use the official Rust image as the base image
FROM rust:latest as build

# Create a new directory for the application
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY . .

# Create a dummy main.rs to build dependencies
RUN echo "fn main() {}" > src/main.rs

# Build dependencies in a separate step to cache them for faster builds
RUN cargo build --release
RUN rm -f target/release/deps/rust-api*

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo build --release

# Create a new lightweight image for deployment
FROM debian:buster-slim

# Install necessary dependencies for SSL support
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder image
COPY --from=build /usr/src/app/target/release/rust-api /usr/local/bin/

# Set the working directory
WORKDIR /usr/local/bin

# Expose the application port
EXPOSE 8080

# Start the application
CMD ["./rust-api"]
