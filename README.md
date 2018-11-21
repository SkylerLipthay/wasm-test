# `wasm-test`

This is just a playground I've been working in to test WASM and WebGL development patterns.

## Building and running

```sh
# Make sure you have the Rust nightly toolchain installed:
rustup install nightly
# Make sure you have the WASM32 target installed for Rust:
rustup target add wasm32-unknown-unknown --toolchain nightly
# Make sure you have the `wasm-bindgen` CLI tool installed:
cargo +nightly install wasm-bindgen-cli
# Install WebPack dependencies:
npm install
# Build the Rust library and wraps the generated WASM using `wasm-bindgen`:
npm run build
# Serve using `webpack-dev-server`:
npm run serve
# Visit `http://localhost:8080`!
```
