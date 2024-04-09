rm -rf wasm_files
mkdir wasm_files

cargo build --target wasm32-unknown-unknown --release --package icrc7
candid-extractor target/wasm32-unknown-unknown/release/icrc7.wasm > icrc7/icrc7.did || true
mv target/wasm32-unknown-unknown/release/icrc7.wasm wasm_files
ic-wasm wasm_files/icrc7.wasm -o wasm_files/icrc7.wasm metadata candid:service -f icrc7/icrc7.did -v public
gzip wasm_files/icrc7.wasm

cargo build --target wasm32-unknown-unknown --release --package archive
candid-extractor target/wasm32-unknown-unknown/release/archive.wasm > archive/archive.did || true
mv target/wasm32-unknown-unknown/release/archive.wasm wasm_files
ic-wasm wasm_files/archive.wasm -o wasm_files/archive.wasm metadata candid:service -f archive/archive.did -v public
gzip wasm_files/archive.wasm
