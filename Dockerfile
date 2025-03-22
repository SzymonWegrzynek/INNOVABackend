FROM rust:1.76 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend

ENV DATABASE_URL=${DATABASE_URL}
ENV SQLX_OFFLINE=false

EXPOSE 8000
CMD ["/app/backend"]
