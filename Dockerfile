FROM rust:1.76 as builder

WORKDIR /src

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /src

COPY --from=builder /src/target/release/backend /src/backend

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ENV SQLX_OFFLINE=false

EXPOSE 8000

CMD ["/src/backend"]
