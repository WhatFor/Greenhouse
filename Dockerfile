FROM rust:1.83 as builder

WORKDIR /usr/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/app/target/release/greenhouse /usr/local/bin/
CMD ["greenhouse"]