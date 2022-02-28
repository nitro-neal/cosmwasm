rustup default stable;
cargo wasm;
RUST_BACKTRACE=1 cargo unit-test;
cargo schema;

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.4

ls;
