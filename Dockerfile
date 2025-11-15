FROM rust:1.91-slim-bullseye AS builder
WORKDIR /workdir
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/terminal-ray-tracer /
CMD ["/terminal-ray-tracer"]
