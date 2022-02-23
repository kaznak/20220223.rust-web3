# 開発環境
FROM rust:1.58.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.58.1-slim
COPY --from=build-stage /app/target/release/web3-server /usr/local/bin/web3-server
EXPOSE 8080
CMD ["/usr/local/bin/web3-server"]
