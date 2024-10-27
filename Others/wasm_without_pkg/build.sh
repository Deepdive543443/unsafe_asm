rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target=wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/release/wasm_init.wasm --out-dir ./target --target no-modules --no-typescript