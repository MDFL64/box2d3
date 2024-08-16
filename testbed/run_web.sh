set -e
JS_LIB=$( realpath library.js )
# -sASSERTIONS=2
export EMCC_CFLAGS="-g -msimd128 -mbulk-memory --js-library=$JS_LIB -sUSE_SDL=2 -sUSE_SDL_GFX=2 -sALLOW_MEMORY_GROWTH=1 -sSTACK_SIZE=131072 -Wno-unused-command-line-argument"
export RUSTFLAGS="-Ctarget-feature=+simd128,+bulk-memory,+nontrapping-fptoint"
cargo build --release --target wasm32-unknown-emscripten
cp ../target/wasm32-unknown-emscripten/release/testbed.wasm web
cp ../target/wasm32-unknown-emscripten/release/testbed.js web
basic-http-server web
