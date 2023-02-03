# WASM Locality attribute

This WASM plugin generates location attributes from metadata on nodes. The attributes can be used to extend telemetry and monitor traffic between locations (zones).

## Build

Install cargo-wasi which allows to build code for `wasm32-wasi` architecture.

```
cargo install cargo-wasi
```

Build a binary.

```
cargo wasi build --release
```
