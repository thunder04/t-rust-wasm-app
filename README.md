# Rust + WASM + vite Application Template

My template for creating a basic Rust application with WASM support, using the vite bundler.

It features logging using the [`tracing`](https://crates.io/crates/tracing) library,
[`color_eyre`](https://crates.io/crates/color_eyre) for pretty panic reports,
and [`thiserror`](https://crates.io/crates/thiserror) for specialized error handling.

## Usage

To use it, install [`cargo-generate`](https://cargo-generate.github.io/cargo-generate/installation.html)
and run:

```sh
cargo generate --git https://github.com/thunder04/t-rust-wasm-app
```

## Folder Structure

- `{{crate_name}}`: The back-end part of the application.
- `{{crate_name}}-lib`: The library for the back-end.
- `{{crate_name}}-web`: The front-end part of the application.
- `{{crate_name}}-web-lib`: The library for the front-end. *Note: It doesn't actually exist.*
- `{{crate_name}}-web-native-lib`: The native (Rust + WASM) library for the front-end.
