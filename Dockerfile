FROM scratch
COPY ./target/wasm32-wasi/release/wasm_locality_attribute.wasm /plugin.wasm
ENTRYPOINT [ "plugin.wasm" ]
