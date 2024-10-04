use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::panic;
use web_sys::console;

// Define the structure of the expected JSON
#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    email: String,
    address: String,
    phone: String,
    website: String,
}

// Set up the panic hook for detailed error logging in the browser
#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// Exported function to sort the JSON by `name`
#[wasm_bindgen]
pub fn sort_json(json_data: &str) -> String {
    // Log the length of the input data
    console::log_1(&format!("Received JSON data of length: {}", json_data.len()).into());

    // Deserialize the JSON string into a Vec of Persons
    let mut persons: Vec<Person> = match serde_json::from_str(json_data) {
        Ok(persons) => persons,
        Err(err) => {
            console::log_1(&format!("Failed to parse JSON: {:?}", err).into());
            wasm_bindgen::throw_str("The provided JSON is invalid or improperly formatted");
        }
    };

    // Sort the persons by `name`
    persons.sort_by(|a, b| a.name.cmp(&b.name));

    // Serialize the sorted data back into a JSON string
    let sorted_json = match serde_json::to_string(&persons) {
        Ok(json) => json,
        Err(err) => {
            console::log_1(&format!("Failed to serialize sorted JSON: {:?}", err).into());
            wasm_bindgen::throw_str("Failed to serialize sorted JSON");
        }
    };

    sorted_json
}
