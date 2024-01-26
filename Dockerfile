# Build stage
FROM rust:1.75 as builder

RUN USER=root cargo new --bin iprust
WORKDIR /iprust

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/iprust*
RUN cargo build --release

# Runtime stage
FROM gcr.io/distroless/cc-debian12

# RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /iprust/target/release/iprust /

ENTRYPOINT ["./iprust"]
