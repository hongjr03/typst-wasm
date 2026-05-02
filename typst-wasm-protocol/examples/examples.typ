// Build first:
// cargo build -p examples --target wasm32-unknown-unknown --release
// Then copy or reference the generated wasm as `examples.wasm` next to this file.

#import "examples-wrapper.typ": *

= typst-wasm-protocol example

== Plain text and number exports

#let input = "Hello, Typst!"

Original: #input\
Uppercase: #to-uppercase(input)\
#count-chars(input)\
10 + 32 = #add(10, 32)\
#divide-numbers(10, 2)\
Email validation: #validate-email("user@example.com")

// Uncomment to see protocol error handling from `Result::Err`:
// #divide-numbers(10, 0)

== CBOR structured exports

#let greeting = greet-person((name: "Ada", age: 36))

Greeting message: #greeting.message\
Adult: #greeting.adult\
Sum list: #sum-list((1, 2, 3, 4, 5))
