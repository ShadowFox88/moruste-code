use serde_json;
use std::env;
use std::fs;

fn concatenate_vector_to_string(vector: &Vec<String>) -> String {
    let mut concatenated = String::new();

    for i in 1..vector.len() {
        let arg = &vector[i];
        concatenated += arg;

        if i < vector.len() - 1 {
            concatenated += " ";
        }
    }

    concatenated
}

fn retrieve_morse_code_mapping() -> serde_json::Value {
    let content = fs::read_to_string("src/morse-code.json").expect("Could not read JSON");
    let morse_code_mapping: serde_json::Value =
        serde_json::from_str(&content).expect("Could not parse JSON");

    morse_code_mapping
}

fn encrypt_to_morse_code(text: &str) -> String {
    let characters: Vec<char> = text.chars().collect();
    let morse_code_mapping = retrieve_morse_code_mapping();
    let mut encrypted = String::new();

    for i in 0..text.len() {
        let character = characters[i];
        let morse_code_character = &morse_code_mapping[character.to_string()];

        if character == ' ' {
            encrypted += "🥢⬛";

            continue;
        } else if morse_code_character.is_null() {
            continue;
        }

        encrypted += &morse_code_character.to_string().replace("\"", "");

        if i < text.len() - 1 {
            encrypted += "⬛";
        }
    }

    encrypted
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = concatenate_vector_to_string(&args);
    let encrypted = encrypt_to_morse_code(&text);

    println!("{}", encrypted);
}
