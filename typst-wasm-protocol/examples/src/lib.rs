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

// Simple string operation function
#[wasm_export]
fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

// Using custom export name
#[wasm_export(export_rename = "count_chars")]
fn count_characters(input: &str) -> String {
    format!("Character count: {}", input.chars().count())
}

// Function returning Result type
#[wasm_export]
fn divide_numbers(a: f64, b: f64) -> Result<String, String> {
    if b == 0.0 {
        return Err("Cannot divide by zero".to_string());
    }

    let result = a / b;
    Ok(format!("Result: {:.2}", result))
}

// Function returning Result<String, String> type
#[wasm_export]
fn validate_email(input: &str) -> Result<&'static str, String> {
    let email = input.trim();

    // Simple email validation
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }

    Ok("Email is valid")
}

// Basic values can be decoded from Typst arguments and encoded back automatically.
#[wasm_export]
fn add(left: i64, right: i64) -> i64 {
    left + right
}

// CBOR mode decodes each argument with serde and encodes the return value as CBOR.
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
