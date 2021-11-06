cargo build --release --target wasm32-unknown-unknown
wasm-opt -O target/wasm32-unknown-unknown/release/mq-columns.wasm -o target/wasm32-unknown-unknown/release/mq-columns.wasm