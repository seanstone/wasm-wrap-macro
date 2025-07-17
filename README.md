# wasm-wrap-macro

`wasm-wrap-macro` provides a procedural macro attribute named `wasm_wrap` for generating `wasm_bindgen` wrappers around the public methods of an `impl` block. The generated wrappers expose the methods to JavaScript when the `wasm` feature is enabled and convert their results to `JsValue` using `serde_wasm_bindgen`.

## Usage

Add the crate and its dependencies to your `Cargo.toml`:

```toml
[dependencies]
wasm-bindgen = "*"
serde_wasm_bindgen = "*"
wasm-wrap-macro = { path = "path/to/wasm-wrap-macro" }
```

Use `#[wasm_wrap]` on an `impl` block whose public methods return `Result<T, Box<dyn std::error::Error>>`:

```rust
use wasm_bindgen::prelude::*;
use wasm_wrap_macro::wasm_wrap;

struct Example;

#[wasm_wrap]
impl Example {
    pub async fn compute(&self) -> Result<u32, Box<dyn std::error::Error>> {
        Ok(42)
    }
}
```

When compiled with the `wasm` feature, this expands to an `impl` block where each public method is annotated with `#[wasm_bindgen]` and returns `Result<JsValue, JsValue>`. Without the feature, the original implementation is emitted unchanged.

## Feature flags

- **`wasm`** – Enable generation of `wasm_bindgen` wrappers. This feature should be enabled when targeting WebAssembly.

## License

This project is licensed under the MIT license.
