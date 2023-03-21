FROM rust:1-buster

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
# RUN cargo check
ENV SQLX_OFFLINE true
# RUN cargo build
RUN cargo build --release
ENV SQLX_OFFLINE false

# Run the binary
EXPOSE 3000
# CMD ["./target/debug/service"]
CMD ["./target/release/service"]
# CMD cargo run
