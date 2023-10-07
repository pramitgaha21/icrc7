rm -rf wasm_files
mkdir wasm_files

# building icrc7
cargo build --target wasm32-unknown-unknown --release --package icrc7
candid-extractor target/wasm32-unknown-unknown/release/icrc7.wasm > src/icrc7/icrc7.did || true

# ic-wasm target/wasm32-unknown-unknown/release/icrc7.wasm -o wasm_files/icrc7.wasm metadata candid:service -v public -f src/icrc7/icrc7.did
# ic-wasm wasm_files/icrc7.wasm -o wasm_files/icrc7.wasm shrink

# building factory
cargo build --target wasm32-unknown-unknown --release --package factory
candid-extractor target/wasm32-unknown-unknown/release/factory.wasm > src/factory/factory.did || true

# ic-wasm target/wasm32-unknown-unknown/release/factory.wasm -o wasm_files/factory.wasm metadata candid:service -v public -f src/factory/factory.did
# ic-wasm wasm_files/factory.wasm -o wasm_files/factory.wasm shrink