# Typst WASM Protocol

[![Crates.io](https://img.shields.io/crates/v/typst-wasm-protocol.svg)](https://crates.io/crates/typst-wasm-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/typst-wasm-protocol.svg)](https://crates.io/crates/typst-wasm-protocol)

> [!IMPORTANT]
> This project is WIP and in early development and may not be fully functional.

A toolkit for [Typst Plugins](https://typst.app/docs/reference/foundations/plugin/) that provides a macro and protocol for exporting typed Rust functions to WebAssembly.

## Installation

Add the following dependency to your Rust project:

```toml
[dependencies]
typst-wasm-protocol = "0.0.2"
```

## Usage

### Exporting Functions to WASM

Use the `wasm_export` macro to mark functions for export. The macro hides the raw wasm-minimal-protocol buffer handling and decodes parameters through `PluginArg`:

```rust
use typst_wasm_protocol::wasm_export;

#[wasm_export]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {name}!")
}

// Custom export name
#[wasm_export(export_rename = "greet")]
pub fn say_hello(name: &str) -> String {
    format!("Hello, {name}!")
}

#[wasm_export]
pub fn add(left: i64, right: i64) -> i64 {
    left + right
}
```

Built-in argument decoding currently supports `&[u8]`, `Vec<u8>`, `&str`, `String`, booleans, integers, and floats. Decode failures are returned to the host as protocol errors.

### Handling Results and Errors

The protocol provides `PluginOutput` and `PluginResult` traits that automatically handle result conversion:

```rust
use typst_wasm_protocol::PluginResult;

#[wasm_export]
pub fn process_data(input: &[u8]) -> Result<Vec<u8>, String> {
    Ok(input.to_vec())
}

// Works with different Result types without manual conversions
#[wasm_export]
pub fn validate_text(text: &str) -> Result<&'static str, String> {
    if text.len() > 10 {
        Ok("Text is valid".to_string())
    } else {
        Err("Text is too short".to_string())
    }
}
```

Built-in result encoding supports `Vec<u8>`, `&[u8]`, `String`, `&str`, `()`, booleans, integers, floats, and `Result<T, E>` where `T` is encodable and `E: ToString`.

### Structured Values with CBOR

Use `#[wasm_export(cbor)]` when the Typst side passes each argument through `cbor.encode(...)` and decodes the result with `cbor(...)`. In this mode, arguments use `serde::Deserialize` and return values use `serde::Serialize`:

```rust
use serde::{Deserialize, Serialize};
use typst_wasm_protocol::wasm_export;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct Greeting {
    message: String,
    adult: bool,
}

#[wasm_export(cbor)]
fn greet_person(person: Person) -> Greeting {
    Greeting {
        message: format!("Hello, {}!", person.name),
        adult: person.age >= 18,
    }
}

#[wasm_export(cbor, export_rename = "sum-list")]
fn sum_list(values: Vec<i64>) -> i64 {
    values.into_iter().sum()
}
```

The equivalent Typst-side wrapper shape is:

```typst
#let wasm = plugin("plugin.wasm")
#let greet-person(person) = cbor(wasm.greet_person(cbor.encode(person)))
#let sum-list(values) = cbor(wasm.sum-list(cbor.encode(values)))
```

Automatic `.typ` wrapper generation, default arguments, varargs, existing-function export, and doc-comment export are still not implemented.

## Examples

See [typst-wasm-protocol/examples](typst-wasm-protocol/examples) for a basic example of using the `wasm_export` macro and handling results. Also [typst-relescope](https://github.com/sjfhsjfh/typst-relescope) as a real-world example of a Typst plugin using this protocol.

## Building WASM Modules

Compile to WebAssembly using:

```bash
cargo build --target wasm32-unknown-unknown --release
```
