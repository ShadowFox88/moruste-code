use serde_json;
use std::env;
use std::fs;
use std::process;

extern crate exitcode;

#[macro_use]
extern crate lazy_static;

fn retrieve_morse_code_mapping(name: &str) -> serde_json::Value {
    let mapping_path = format!("src/{}.json", name);
    let content = fs::read_to_string(mapping_path).expect("Could not read JSON");
    let morse_code_mapping: serde_json::Value =
        serde_json::from_str(&content).expect("Could not parse JSON");

    morse_code_mapping
}

lazy_static! {
    static ref SERIALISED_MAPPING: serde_json::Value = retrieve_morse_code_mapping("serialised");
    static ref DESERIALISED_MAPPING: serde_json::Value =
        retrieve_morse_code_mapping("deserialised");
}

fn encrypt_to_morse_code(text: &str) -> String {
    let characters: Vec<char> = text.chars().collect();
    let mut to_join: Vec<String> = vec![];

    for character in characters {
        let morse_code_character = &SERIALISED_MAPPING[character.to_string()];

        if morse_code_character.is_null() {
            continue;
        }

        let formatted = morse_code_character.to_string().replace("\"", "");

        to_join.push(formatted);
    }

    return to_join.join(" ");
}

fn decrypt_to_morse_code(text: &str) -> String {
    let characters: Vec<&str> = text.split(" ").collect();
    let mut decrypted = String::new();

    for character in characters.iter() {
        let morse_code_character = &DESERIALISED_MAPPING[character.to_string()];

        if morse_code_character.is_null() {
            continue;
        }

        let formatted = &morse_code_character.to_string().replace("\"", "");

        decrypted += formatted;
    }

    decrypted
}

fn main() {
    let mut result = String::new();
    let mut args = env::args();
    let subcommand = args.nth(1).unwrap().to_ascii_lowercase();
    let to_concatenate: Vec<String> = args.collect();
    let text = to_concatenate.join(" ");

    if text == "" {
        process::exit(exitcode::DATAERR);
    }

    if subcommand == "encrypt" {
        result = encrypt_to_morse_code(&text);
    } else if subcommand == "decrypt" {
        result = decrypt_to_morse_code(&text);
    }

    println!("{}", result);
    process::exit(exitcode::OK);
}
