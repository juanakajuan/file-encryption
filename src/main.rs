use std::env;
use std::fs;
use std::io;

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

            let output_path = format!("{}.enc", file_path);

            process_file(file_path, &output_path, key)
                .expect("A critical file I/O error occurred during processing.");

            println!("Success! File processed.");
        }
        "decrypt" => {
            let output_path: &str = file_path.trim_end_matches(".enc");

            process_file(file_path, output_path, key)
                .expect("A critical file I/O error occurred during processing.");
        }
        _ => {
            println!("Error: Action must be 'encrypt' or 'decrypt'.");
        }
    }
}

fn process_file(file_path: &str, output_path: &str, key: &str) -> io::Result<()> {
    let file_contents: Vec<u8> = fs::read(file_path)?;

    let key_bytes: &[u8] = key.as_bytes();
    let new_data: Vec<u8> = file_contents
        .iter()
        .zip(key_bytes.iter().cycle())
        .map(|(data_byte, key_byte)| *data_byte ^ *key_byte)
        .collect();

    fs::write(output_path, new_data)?;

    Ok(())
}
