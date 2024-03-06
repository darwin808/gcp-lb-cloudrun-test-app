

# 1. This tells docker to use the Rust official image
FROM rust:1.76

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Set the PORT environment variable
# Build your program for release
RUN cargo build --release

ENV PORT=8080
EXPOSE 8080

# Run the binary
CMD ["./target/release/hello-zesty"]
