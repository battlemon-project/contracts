FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y cmake pkg-config libssl-dev git clang curl gnupg \
    && curl -sL https://deb.nodesource.com/setup_17.x  | bash - \
    && apt-get -y install nodejs \
    && npm install -g near-cli \
    && rustup target add wasm32-unknown-unknown

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --target wasm32-unknown-unknown \
    && chmod +x ./scripts/localnet_deploy.sh
#   && mv battlemon.testnet.json /root/.near-credentials/testnet/
#ENV NEAR_ENV=local
#ENV NEAR_CLI_LOCALNET_RPC_SERVER_URL=http://indexer:3030
#ENV NEAR_CLI_LOCALNET_KEY_PATH=/near-config/validator_key.json

ENTRYPOINT ["./scripts/testnet_deploy.sh"]