FROM scratch
COPY ./target/wasm32-wasi/release/wasm_locality_attribute.wasm /wasm_locality_attribute.wam
ENTRYPOINT [ "wasm_locality_attribute.wasm" ]
