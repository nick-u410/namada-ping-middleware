FROM lukemathwalker/cargo-chef:latest-rust-1.76.0-bullseye as builder

# Set the working directory
WORKDIR /app

RUN apt-get update && apt-get install -y \
    build-essential \
    clang-tools-11 \
    git \
    libssl-dev \
    pkg-config \
    protobuf-compiler \
    libudev-dev \
    && apt-get clean

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo install --path .


# Use a smaller base image for the final image
FROM debian:bullseye

RUN apt-get update && apt-get install libcurl4-openssl-dev libudev-dev -y && apt-get clean

# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/namada-ping-middleware /usr/local/bin/namada-ping-middleware

RUN echo "RPC="http://node:26657"" | tee .env

EXPOSE 1317
# Set the command to run your application
CMD ["namada-ping-middleware"]