use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 4 {
        println!("Error: Please provide an action (encrypt/decrypt), a file path, and a key.");
        return;
    }

    let action: &String = &arguments[1];
    let file_path: &String = &arguments[2];
    let key: &String = &arguments[3];

    match action.as_str() {
        "encrypt" => {
            println!("Starting encryption for file: {}", file_path);

            let file_contents: Vec<u8> = fs::read(file_path)
                .expect("Failed to read the file. Check if the path is correct!");

            let key_bytes: &[u8] = key.as_bytes();
            let new_data: Vec<u8> = file_contents
                .iter()
                .zip(key_bytes.iter().cycle())
                .map(|(data_byte, key_byte)| *data_byte ^ *key_byte)
                .collect();

            let output_file_path: String = format!("{}.enc", file_path);

            fs::write(&output_file_path, new_data).expect("Failed to write the encrypted file!");

            println!("Success! File encrypted to: {}", output_file_path);
        }
        "decrypt" => {
            println!("Starting decryption for file: {}", file_path);
        }
        _ => {
            println!("Error: Action must be 'encrypt' or 'decrypt'.");
        }
    }
}
