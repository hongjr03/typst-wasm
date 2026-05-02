// Hand-written wrapper for the Rust exports in `src/lib.rs`.
// It mirrors the wrapper this crate should generate automatically in the future.

#let wasm = plugin("examples.wasm")

// Plain mode exports pass text-like arguments as bytes and receive bytes back.
#let to-uppercase(input) = str(wasm.to_uppercase(bytes(input)))
#let count-chars(input) = str(wasm.count_chars(bytes(input)))
#let divide-numbers(a, b) = str(wasm.divide_numbers(bytes(str(a)), bytes(str(b))))
#let validate-email(input) = str(wasm.validate_email(bytes(input)))
#let add(left, right) = int(str(wasm.add(bytes(str(left)), bytes(str(right)))))

// CBOR mode exports pass structured Typst values through cbor.encode and decode
// the returned CBOR payload back into native Typst values.
#let greet-person(person) = cbor(wasm.greet_person(cbor.encode(person)))
#let sum-list(values) = cbor(wasm.sum-list(cbor.encode(values)))
