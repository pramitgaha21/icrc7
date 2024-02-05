rm -rf wasm_files
mkdir wasm_files

cargo build --target wasm32-unknown-unknown --release --package icrc7
candid-extractor target/wasm32-unknown-unknown/release/icrc7.wasm > icrc7/icrc7.did || true
mv target/wasm32-unknown-unknown/release/icrc7.wasm wasm_files
gzip wasm_files/icrc7.wasm

cargo build --target wasm32-unknown-unknown --release --package factory
candid-extractor target/wasm32-unknown-unknown/release/factory.wasm > factory/factory.did || true